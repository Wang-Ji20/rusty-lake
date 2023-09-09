use crate::parser::{parser::LispVal, Parser};

use super::environment::Environment;

pub struct Interpreter<'a> {
    parser: Parser<'a>,
    env: Environment,
}

/// This implementation always eagerly evaluates all expressions.
impl Interpreter<'_> {
    fn new(s: &str) -> Interpreter {
        Interpreter {
            parser: Parser::new(s),
            env: Environment::new(),
        }
    }

    fn eval_all(&mut self) -> Result<LispVal, String> {
        let ast = self.parser.parse();
        self.eval(&ast)
    }

    fn eval(&mut self, v: &LispVal) -> Result<LispVal, String> {
        match v {
            LispVal::Atom(_) => todo!(),
            LispVal::List(v) => self.eval_list(v),
            i @ LispVal::Integer(_) => Ok(i.clone()),
            LispVal::Bool(_) => todo!(),
        }
    }

    fn eval_list(&mut self, v: &Vec<LispVal>) -> Result<LispVal, String> {
        match v.len() {
            0 => Ok(LispVal::List(v.clone())),
            _ => v
                .split_first()
                .ok_or("cannot split".to_string())
                .and_then(|v| {
                    let (LispVal::Atom(s), operands) = v else {
                                   return Err(format!("{:?} is not applicable to {:?}", v.0, v.1))
                               };
                    if s == "quote" {
                        return Ok(operands[0].clone());
                    }
                    self.apply(s, operands)
                }),
        }
    }

    fn apply(&mut self, operator: &str, operands: &[LispVal]) -> Result<LispVal, String> {
        operands
            .iter()
            .map(|v| self.eval(v))
            .collect::<Result<Vec<LispVal>, String>>()
            .and_then(Self::lookup_primitives(operator)?)
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
                        return Err(format!("Cannot add {:?}", acc))
                    };
                    Ok(f(ac_int, i))
                })
                .map(LispVal::Integer)
        }
    }

    fn add_int(acc: i64, i: i64) -> i64 {
        acc + i
    }

    fn mul_int(acc: i64, i: i64) -> i64 {
        acc * i
    }

    fn sub_int(acc: i64, i: i64) -> i64 {
        acc - i
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
                    return Err("Cannot take car of non-list".to_string())
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
                    return Err("Cannot take cdr of non-list".to_string())
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
                    return Err("Cannot take cons of non-list".to_string())
                };
                v2.insert(0, v.clone());
                Ok(LispVal::List(v2))
            })
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
            _ => Err(format!("unknown primitive {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter;

    use super::*;

    #[test]
    fn test_eval_integer() {
        let mut interpreter = Interpreter::new("4");
        if let LispVal::Integer(4) = interpreter.eval_all().unwrap() {
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_add() {
        let mut interpreter = Interpreter::new("(+ 1 2 3)");
        if let LispVal::Integer(6) = interpreter.eval_all().unwrap() {
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_mul() {
        let mut interpreter = Interpreter::new("(* 4 2 3)");
        if let LispVal::Integer(i) = interpreter.eval_all().unwrap() {
            assert_eq!(i, 24);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_sub() {
        let mut interpreter = Interpreter::new("(- 4 2 3)");
        if let LispVal::Integer(i) = interpreter.eval_all().unwrap() {
            assert_eq!(i, -1);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_car() {
        let mut interpreter = Interpreter::new("(car '(1 2 3))");
        if let LispVal::Integer(i) = interpreter.eval_all().unwrap() {
            assert_eq!(i, 1);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_cdr() {
        let mut interpreter = Interpreter::new("(cdr '(1 2 3))");
        if let LispVal::List(v) = interpreter.eval_all().unwrap() {
            assert_eq!(v, vec![LispVal::Integer(2), LispVal::Integer(3)]);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_eval_cons() {
        let mut interpreter = Interpreter::new("(cons 1 '(2 3))");
        if let LispVal::List(v) = interpreter.eval_all().unwrap() {
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
}
