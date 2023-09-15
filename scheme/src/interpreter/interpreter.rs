use std::io::Write;

use crate::parser::{parser::LispVal, Parser};

use super::environment::Environment;

pub struct Interpreter {
    env: Environment,
}

const WELCOME: &str = "Welcome to a Scheme interpreter!";
const BYE: &str = "Avē Imperātor, moritūrī tē salūtant!";
const PROMPT: &str = "]=> ";

/// This implementation always eagerly evaluates all expressions.
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Environment::new(),
        }
    }

    pub fn interpret_file(&mut self, path: std::path::PathBuf) -> Result<LispVal, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        self.interpret(&content)
    }

    pub fn interpret_repl(&mut self) {
        println!("{}", WELCOME);
        loop {
            print!("{}", PROMPT);
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            let input_size = std::io::stdin().read_line(&mut input).unwrap();
            if input_size == 0 {
                println!("{}", BYE);
                break;
            }
            let result = self.interpret(&input);
            match result {
                Ok(v) => println!("\n{:?}\n", v),
                Err(e) => println!("\nError: {}\n", e),
            }
        }
    }

    pub fn interpret(&mut self, s: &str) -> Result<LispVal, String> {
        let ast = Parser::new(s).parse();
        self.eval(&ast)
    }

    fn eval(&mut self, v: &LispVal) -> Result<LispVal, String> {
        match v {
            LispVal::Atom(s) => self.eval_atom(s),
            LispVal::List(v) => self.eval_list(v),
            i @ LispVal::Integer(_) => Ok(i.clone()),
            b @ LispVal::Bool(_) => Ok(b.clone()),
            _ => todo!(),
        }
    }

    fn eval_atom(&self, s: &str) -> Result<LispVal, String> {
        self.env
            .lookup(s)
            .cloned()
            .ok_or(format!("unknown atom {}", s))
    }

    fn eval_list(&mut self, v: &Vec<LispVal>) -> Result<LispVal, String> {
        match v.len() {
            0 => Ok(LispVal::List(v.clone())),
            _ => v
                .split_first()
                .ok_or("cannot split".to_string())
                .and_then(|v| {
                    let (LispVal::Atom(s), operands) = v else {
                        return Err(format!("{:?} is not applicable to {:?}", v.0, v.1));
                    };
                    match s.as_str() {
                        "quote" => return Ok(operands[0].clone()),
                        "define" => return self.define_value(operands.to_vec()),
                        "if" => return self.eval_if(operands.to_vec()),
                        _ => {}
                    };
                    self.apply(s, operands)
                }),
        }
    }

    fn eval_if(&mut self, to_vec: Vec<LispVal>) -> Result<LispVal, String> {
        let (cond, branches) = to_vec.split_first().ok_or("if must have a condition")?;
        let cond = self.eval(cond)?; // shadowed
        match branches.len() {
            1 => self.eval_if_only(cond, to_vec),
            2 => self.eval_if_else(cond, to_vec),
            _ => Err("if must have 2 or 3 arguments".to_string()),
        }
    }

    fn eval_if_only(&mut self, cond: LispVal, to_vec: Vec<LispVal>) -> Result<LispVal, String> {
        match cond {
            LispVal::Bool(false) => Err("Unspecified return value".to_string()),
            _ => self.eval(&to_vec[1]),
        }
    }

    fn eval_if_else(&mut self, cond: LispVal, to_vec: Vec<LispVal>) -> Result<LispVal, String> {
        match cond {
            LispVal::Bool(false) => self.eval(&to_vec[2]),
            _ => self.eval(&to_vec[1]),
        }
    }

    fn apply(&mut self, operator: &str, operands: &[LispVal]) -> Result<LispVal, String> {
        let evaluated_operands = operands
            .iter()
            .map(|v| self.eval(v))
            .collect::<Result<Vec<LispVal>, String>>()?;
        match Self::lookup_primitives(operator) {
            Ok(f) => f(evaluated_operands),
            Err(_) => self.apply_func(operator, evaluated_operands.as_slice()),
        }
    }

    fn apply_func(&mut self, operator: &str, operands: &[LispVal]) -> Result<LispVal, String> {
        let LispVal::Function { params, body } = self
            .env
            .lookup(operator)
            .cloned()
            .ok_or(format!("unknown function {}", operator))?
        else {
            return Err(format!("{} is not a function", operator));
        };
        if params.len() != operands.len() {
            return Err(format!(
                "function {} expects {} arguments, but got {}",
                operator,
                params.len(),
                operands.len()
            ));
        }
        self.env.new_frame();
        for (param, operand) in params.iter().zip(operands.iter()) {
            self.env.new_binding(param.clone(), operand.clone());
        }
        let result = self.eval(&body[0]);
        self.env.pop_frame();
        result
    }

    fn foldable_primitive(
        f: impl Fn(LispVal, &LispVal) -> Result<LispVal, String>,
        default_val: LispVal,
    ) -> impl Fn(Vec<LispVal>) -> Result<LispVal, String> {
        move |v| v.iter().try_fold(default_val.clone(), &f)
    }

    fn lift_int_funcs(
        f: fn(i64, i64) -> i64,
    ) -> impl Fn(LispVal, &LispVal) -> Result<LispVal, String> {
        move |acc, lisp_int| {
            lisp_int
                .to_integer()
                .ok_or(format!("Cannot add {:?}", lisp_int))
                .and_then(|i| {
                    let Some(ac_int) = acc.to_integer() else {
                        return Err(format!("Cannot add {:?}", acc));
                    };
                    Ok(f(ac_int, i))
                })
                .map(LispVal::Integer)
        }
    }

    /// special forms

    fn add_int(acc: i64, i: i64) -> i64 {
        acc + i
    }

    fn mul_int(acc: i64, i: i64) -> i64 {
        acc * i
    }

    fn sub_int(acc: i64, i: i64) -> i64 {
        acc - i
    }

    fn eq_lisp(v: Vec<LispVal>) -> Result<LispVal, String> {
        let mut iter = v.iter();
        let first_int = iter
            .next()
            .ok_or("Cannot compare empty list".to_string())?
            .to_integer()
            .ok_or("Cannot compare non-integer".to_string())?;
        for i in iter {
            let i = i
                .to_integer()
                .ok_or("Cannot compare non-integer".to_string())?;
            if i != first_int {
                return Ok(LispVal::Bool(false));
            }
        }
        Ok(LispVal::Bool(true))
    }

    fn sub_impl(v: Vec<LispVal>) -> Result<LispVal, String> {
        let mut iter = v.iter();
        let first_int = iter
            .next()
            .ok_or("Cannot subtract from empty list".to_string())?;
        Self::foldable_primitive(Self::lift_int_funcs(Self::sub_int), first_int.clone())(
            v[1..].to_vec(),
        )
    }

    fn car_list(v: Vec<LispVal>) -> Result<LispVal, String> {
        v.first()
            .ok_or("Cannot take car of empty list".to_string())
            .and_then(|v| {
                let LispVal::List(v) = v else {
                    return Err("Cannot take car of non-list".to_string());
                };
                Ok(v)
            })
            .and_then(|v| {
                v.first()
                    .ok_or("Cannot take car of empty list".to_string())
                    .cloned()
            })
    }

    fn cdr_list(v: Vec<LispVal>) -> Result<LispVal, String> {
        v.first()
            .ok_or("Cannot take cdr of empty list".to_string())
            .and_then(|v| {
                let LispVal::List(v) = v else {
                    return Err("Cannot take cdr of non-list".to_string());
                };
                Ok(v)
            })
            .map(|v| LispVal::List(v[1..].to_vec()))
    }

    fn cons_list(v: Vec<LispVal>) -> Result<LispVal, String> {
        v.split_first()
            .ok_or("Cannot take cons of empty list".to_string())
            .and_then(|(v, v2)| {
                let LispVal::List(mut v2) = v2[0].clone() else {
                    return Err("Cannot take cons of non-list".to_string());
                };
                v2.insert(0, v.clone());
                Ok(LispVal::List(v2))
            })
    }

    fn define_value(&mut self, v: Vec<LispVal>) -> Result<LispVal, String> {
        match &v[0] {
            LispVal::Atom(s) => {
                let val = self.eval(&v[1])?;
                self.env.new_binding(s.clone(), val.clone());
                Ok(val)
            }
            LispVal::List(_) => self.define_function(v),
            _ => Err("unknown define".to_string()),
        }
    }

    fn define_function(&mut self, v: Vec<LispVal>) -> Result<LispVal, String> {
        let LispVal::List(signature) = &v[0] else {
            return Err("define function must have signatures".to_string());
        };
        let LispVal::Atom(name) = &signature[0] else {
            return Err("define function must have a name".to_string());
        };
        let params = signature[1..]
            .iter()
            .map(|v| match v {
                LispVal::Atom(s) => Ok(s.clone()),
                _ => Err("define function signature must be atoms".to_string()),
            })
            .collect::<Result<Vec<String>, String>>();
        let body = v[1..].to_vec();
        let val = LispVal::Function {
            params: params?,
            body,
        };
        self.env.new_binding(name.clone(), val.clone());
        Ok(val)
    }

    fn lookup_primitives(
        s: &str,
    ) -> Result<Box<dyn Fn(Vec<LispVal>) -> Result<LispVal, String>>, String> {
        match s {
            "+" => Ok(Box::new(Self::foldable_primitive(
                Self::lift_int_funcs(Self::add_int),
                LispVal::Integer(0),
            ))),
            "*" => Ok(Box::new(Self::foldable_primitive(
                Self::lift_int_funcs(Self::mul_int),
                LispVal::Integer(1),
            ))),
            "-" => Ok(Box::new(Self::sub_impl)),
            "car" => Ok(Box::new(Self::car_list)),
            "cdr" => Ok(Box::new(Self::cdr_list)),
            "cons" => Ok(Box::new(Self::cons_list)),
            "eq?" | "=" => Ok(Box::new(Self::eq_lisp)),
            _ => Err(format!("unknown primitive {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_integer() {
        let interpreter = Interpreter::new().interpret("4");
        if let LispVal::Integer(4) = interpreter.unwrap() {
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_add() {
        let interpreter = Interpreter::new().interpret("(+ 1 2 3)");
        if let LispVal::Integer(6) = interpreter.unwrap() {
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_mul() {
        let interpreter = Interpreter::new().interpret("(* 4 2 3)");
        if let LispVal::Integer(i) = interpreter.unwrap() {
            assert_eq!(i, 24);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_sub() {
        let interpreter = Interpreter::new().interpret("(- 4 2 3)");
        if let LispVal::Integer(i) = interpreter.unwrap() {
            assert_eq!(i, -1);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_car() {
        let interpreter = Interpreter::new().interpret("(car '(1 2 3))");
        if let LispVal::Integer(i) = interpreter.unwrap() {
            assert_eq!(i, 1);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_cdr() {
        let interpreter = Interpreter::new().interpret("(cdr '(1 2 3))");
        if let LispVal::List(v) = interpreter.unwrap() {
            assert_eq!(v, vec![LispVal::Integer(2), LispVal::Integer(3)]);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_cons() {
        let interpreter = Interpreter::new().interpret("(cons 1 '(2 3))");
        if let LispVal::List(v) = interpreter.unwrap() {
            assert_eq!(
                v,
                vec![
                    LispVal::Integer(1),
                    LispVal::Integer(2),
                    LispVal::Integer(3)
                ]
            );
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_define() {
        let interpreter = Interpreter::new().interpret("(define x 1)");
        if let LispVal::Integer(i) = interpreter.unwrap() {
            assert_eq!(i, 1);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_define_env_bound() {
        let mut interpreter = Interpreter::new();
        interpreter.interpret("(define x 1)").unwrap();
        assert_eq!(
            format!("{:?}", interpreter.env),
            "Environment([EnvFrame([(\"x\", Integer(1))])])"
        )
    }

    #[test]
    fn test_define_lookup() {
        let mut interpreter = Interpreter::new();
        interpreter.interpret("(define x 1)").unwrap();
        assert_eq!(interpreter.env.lookup("x"), Some(&LispVal::Integer(1)))
    }

    #[test]
    fn test_define_reference() {
        let mut interpreter = Interpreter::new();
        interpreter.interpret("(define x 1)").unwrap();
        assert_eq!(interpreter.interpret("x"), Ok(LispVal::Integer(1)))
    }

    #[test]
    fn test_function_define() {
        let mut interpreter = Interpreter::new();
        interpreter.interpret("(define (add1 x) (+ x 1))").unwrap();
        assert_eq!(
            format!("{:?}", interpreter.env),
            "Environment([EnvFrame([(\"add1\", Function { params: [\"x\"], body: [List([Atom(\"+\"), Atom(\"x\"), Integer(1)])] })])])"
        )
    }

    #[test]
    fn test_function() {
        let mut interpreter = Interpreter::new();
        interpreter.interpret("(define (add1 x) (+ x 1))").unwrap();
        assert_eq!(interpreter.interpret("(add1 1)"), Ok(LispVal::Integer(2)))
    }

    #[test]
    fn test_function_composition() {
        let mut interpreter = Interpreter::new();
        interpreter.interpret("(define (add1 x) (+ x 1))").unwrap();
        interpreter
            .interpret("(define (add2 x) (+ (add1 x) 1))")
            .unwrap();
        assert_eq!(interpreter.interpret("(add2 1)"), Ok(LispVal::Integer(3)))
    }

    #[test]
    fn test_if() {
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.interpret("(if #t 1 2)"),
            Ok(LispVal::Integer(1))
        )
    }

    #[test]
    fn test_if_else() {
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.interpret("(if #f 1 2)"),
            Ok(LispVal::Integer(2))
        )
    }

    #[test]
    fn test_eq() {
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.interpret("(= 1 1)"), Ok(LispVal::Bool(true)))
    }

    #[test]
    fn test_many_eq() {
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.interpret("(= 1 1 1 1)"),
            Ok(LispVal::Bool(true))
        )
    }

    #[test]
    fn test_many_eq_f() {
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.interpret("(= 1 1 1 2)"),
            Ok(LispVal::Bool(false))
        )
    }

    #[test]
    fn test_eq_with_eval() {
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.interpret("(= (+ 1 1) 2)"),
            Ok(LispVal::Bool(true))
        )
    }

    #[test]
    fn test_fib_func() {
        let mut interpreter = Interpreter::new();
        interpreter
            .interpret(
                "(define (fib n) (if (= n 0) 0 (if (= n 1) 1 (+ (fib (- n 1)) (fib (- n 2))))))",
            )
            .unwrap();
        assert_eq!(interpreter.interpret("(fib 0)"), Ok(LispVal::Integer(0)));
        assert_eq!(interpreter.interpret("(fib 1)"), Ok(LispVal::Integer(1)));
        assert_eq!(interpreter.interpret("(fib 2)"), Ok(LispVal::Integer(1)));
        assert_eq!(interpreter.interpret("(fib 3)"), Ok(LispVal::Integer(2)));
        assert_eq!(interpreter.interpret("(fib 4)"), Ok(LispVal::Integer(3)));
        assert_eq!(interpreter.interpret("(fib 5)"), Ok(LispVal::Integer(5)));
        assert_eq!(interpreter.interpret("(fib 6)"), Ok(LispVal::Integer(8)));
        assert_eq!(interpreter.interpret("(fib 7)"), Ok(LispVal::Integer(13)));
        assert_eq!(interpreter.interpret("(fib 8)"), Ok(LispVal::Integer(21)));
        assert_eq!(interpreter.interpret("(fib 9)"), Ok(LispVal::Integer(34)));
        assert_eq!(interpreter.interpret("(fib 10)"), Ok(LispVal::Integer(55)));
    }
}
