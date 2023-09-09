use crate::lexer::{self, Cursor, Tokens};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LispVal {
    Atom(String),
    List(Vec<LispVal>),
    Integer(i64),
    Bool(bool),
}

impl LispVal {
    pub fn to_integer(&self) -> Option<i64> {
        match self {
            LispVal::Integer(i) => Some(*i),
            _ => None,
        }
    }
}

pub struct Parser<'a> {
    lexer: Cursor<'a>,
}

impl Parser<'_> {
    pub fn new(text: &str) -> Parser {
        Parser {
            lexer: Cursor::new(text),
        }
    }

    pub fn parse(&mut self) -> LispVal {
        let tok = self.lexer.get_next_token();
        self.parse_literals(tok)
    }

    fn parse_literals(&mut self, l: Tokens) -> LispVal {
        match l {
            lexer::Tokens::RPAREN => todo!(),
            lexer::Tokens::Float(f) => todo!(),
            lexer::Tokens::QUOTE => {
                LispVal::List(vec![LispVal::Atom("quote".to_string()), self.parse()])
            }
            lexer::Tokens::Char(_) => todo!(),
            lexer::Tokens::Unknown => todo!(),
            lexer::Tokens::EOF => todo!(),
            Tokens::Atom(s) => LispVal::Atom(s),
            Tokens::Int(i) => LispVal::Integer(i),
            Tokens::Boolean(b) => LispVal::Bool(b),
            Tokens::LPAREN => self.parse_list(),
            _ => panic!("not Literal"),
        }
    }

    fn parse_list(&mut self) -> LispVal {
        let mut list_children = Vec::new();
        loop {
            match self.lexer.get_next_token() {
                lexer::Tokens::RPAREN => break,
                otherwise => list_children.push(self.parse_literals(otherwise)),
            }
        }
        LispVal::List(list_children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let test_text = "4";
        let mut parser = Parser::new(test_text);
        if let LispVal::Integer(4) = parser.parse() {
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_parse_easy_list() {
        let mut parser = Parser::new("(+ 1 1)");
        if let LispVal::List(v) = parser.parse() {
            assert_eq!(v[0], LispVal::Atom("+".to_string()));
            assert_eq!(v[1], LispVal::Integer(1));
            assert_eq!(v[2], LispVal::Integer(1));
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_many_add() {
        let mut parser = Parser::new("(+ 1 1 1 1 1 1)");
        let equivlent_lispval = LispVal::List(vec![
            LispVal::Atom("+".to_string()),
            LispVal::Integer(1),
            LispVal::Integer(1),
            LispVal::Integer(1),
            LispVal::Integer(1),
            LispVal::Integer(1),
            LispVal::Integer(1),
        ]);
        assert_eq!(parser.parse(), equivlent_lispval);
    }

    #[test]
    fn test_deep_nested_list() {
        let mut parser = Parser::new(
            "
        (* (cond ((> a b) a)
        ((< a b) b)
        (else 1))
        (+ a 1))
        ",
        );

        let equivlent_lispval = LispVal::List(vec![
            LispVal::Atom("*".to_string()),
            LispVal::List(vec![
                LispVal::Atom("cond".to_string()),
                LispVal::List(vec![
                    LispVal::List(vec![
                        LispVal::Atom(">".to_string()),
                        LispVal::Atom("a".to_string()),
                        LispVal::Atom("b".to_string()),
                    ]),
                    LispVal::Atom("a".to_string()),
                ]),
                LispVal::List(vec![
                    LispVal::List(vec![
                        LispVal::Atom("<".to_string()),
                        LispVal::Atom("a".to_string()),
                        LispVal::Atom("b".to_string()),
                    ]),
                    LispVal::Atom("b".to_string()),
                ]),
                LispVal::List(vec![LispVal::Atom("else".to_string()), LispVal::Integer(1)]),
            ]),
            LispVal::List(vec![
                LispVal::Atom("+".to_string()),
                LispVal::Atom("a".to_string()),
                LispVal::Integer(1),
            ]),
        ]);

        assert_eq!(parser.parse(), equivlent_lispval);
    }

    #[test]
    fn test_parse_quote() {
        let mut parser = Parser::new("'(1 2 3)");
        let equivlent_lispval = LispVal::List(vec![
            LispVal::Atom("quote".to_string()),
            LispVal::List(vec![
                LispVal::Integer(1),
                LispVal::Integer(2),
                LispVal::Integer(3),
            ]),
        ]);
        assert_eq!(parser.parse(), equivlent_lispval);
    }
}
