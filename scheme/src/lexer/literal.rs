use std::str::Chars;

#[derive(Clone, Debug)]
pub enum Tokens {
    /// "("
    LPAREN,
    /// ")"
    RPAREN,
    /// "'"
    QUOTE,
    /// Literals:
    Atom(String),
    Int(i64),
    Float(f64),
    /// #t, #f
    Boolean(bool),
    /// #\c
    Char(char),
    /// wait for next stage...
    Unknown,
    EOF,
}

pub struct Cursor<'a> {
    text: Chars<'a>,
}

const EOF_SYMBOL: char = '\0';

impl Cursor<'_> {
    pub fn new(text: &str) -> Cursor {
        Cursor { text: text.chars() }
    }

    fn peek(&self) -> char {
        self.text.clone().next().unwrap_or(EOF_SYMBOL)
    }

    fn is_delimiter(&self) -> bool {
        self.is_eof() || self.peek().is_whitespace()
    }

    fn is_eof(&self) -> bool {
        self.text.as_str().is_empty()
    }

    fn has_next(&self) -> bool {
        !self.is_eof()
    }

    fn consume(&mut self) -> Option<char> {
        self.text.next()
    }

    fn consume_delimiter(&mut self) {
        self.consume_while(|c: char| c.is_whitespace())
    }

    fn consume_while_clone(&mut self, mut predicate: impl FnMut(char) -> bool) -> String {
        let mut s = String::new();
        while predicate(self.peek()) && self.has_next() {
            s.push(self.consume().unwrap());
        }
        s
    }

    fn consume_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) && self.has_next() {
            self.consume();
        }
    }

    pub fn get_next_token(&mut self) -> Tokens {
        self.consume_delimiter();
        match self.peek() {
            '(' => {
                self.consume();
                Tokens::LPAREN
            }
            ')' => {
                self.consume();
                Tokens::RPAREN
            }
            '\'' => {
                self.consume();
                Tokens::QUOTE
            }
            c if c.is_ascii_digit() => self.get_number(),
            '#' => self.get_hashtag_literals(),
            EOF_SYMBOL => Tokens::EOF,
            atom => {
                Tokens::Atom(self.consume_while_clone(|c: char| !c.is_whitespace() && c != ')'))
            }
        }
    }

    fn get_number(&mut self) -> Tokens {
        let number = self.consume_while_clone(|c: char| c.is_ascii_digit());

        match self.peek() {
            '.' => self.get_float_after_dot(number),
            c if self.is_delimiter() || c == ')' => Tokens::Int(number.parse().unwrap()),
            _ => Tokens::Unknown,
        }
    }

    fn get_float_after_dot(&mut self, mut left_part: String) -> Tokens {
        self.consume();
        let after_dot = self.consume_while_clone(|c: char| c.is_ascii_digit());
        left_part.push('.');
        left_part.push_str(&after_dot);

        match self.is_delimiter() {
            true => Tokens::Float(left_part.parse().unwrap()),
            false => Tokens::Unknown,
        }
    }

    fn get_hashtag_literals(&mut self) -> Tokens {
        self.consume();
        match self.consume().unwrap() {
            't' => Tokens::Boolean(true),
            'f' => Tokens::Boolean(false),
            '\\' => self.get_char(),
            _ => Tokens::Unknown,
        }
    }

    fn get_char(&mut self) -> Tokens {
        let c = self.consume().unwrap();
        if self.is_delimiter() {
            Tokens::Char(c)
        } else {
            Tokens::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UNREACHABLE: bool = false;

    #[test]
    fn basic_primitive_test() {
        let mut lexer = Cursor::new("12345");
        assert_eq!(lexer.peek(), '1');
        assert_eq!(lexer.consume(), Some('1'));
        assert!(lexer.has_next());
        assert_eq!(
            lexer.consume_while_clone(|c: char| c.is_ascii_digit()),
            "2345"
        );
        assert!(lexer.is_eof());
        assert!(lexer.is_delimiter())
    }

    #[test]
    fn get_int_test() {
        let number_text = "12345";
        let mut lexer = Cursor::new(number_text);
        if let Tokens::Int(i) = lexer.get_number() {
            assert_eq!(i, 12345);
        } else {
            assert!(UNREACHABLE);
        }
    }

    #[test]
    fn parse_int_test() {
        let number_test = "12345";
        let mut lexer = Cursor::new(number_test);
        if let Tokens::Int(i) = lexer.get_next_token() {
            assert_eq!(i, 12345);
        } else {
            assert!(UNREACHABLE);
        }
    }

    #[test]
    fn parse_float_test() {
        let float_test = "1.2";
        let mut lexer = Cursor::new(float_test);
        if let Tokens::Float(i) = lexer.get_next_token() {
            assert_eq!(i, 1.2);
            return;
        }
        assert!(UNREACHABLE)
    }

    #[test]
    fn parse_bool_true_test() {
        let bool_test = "#t";
        let mut lexer = Cursor::new(bool_test);
        if let Tokens::Boolean(b) = lexer.get_next_token() {
            assert!(b);
            return;
        }
        assert!(UNREACHABLE)
    }

    #[test]
    fn parse_bool_false_test() {
        let bool_test = "#f";
        let mut lexer = Cursor::new(bool_test);
        if let Tokens::Boolean(b) = lexer.get_next_token() {
            assert!(!b);
            return;
        }
        assert!(UNREACHABLE)
    }

    #[test]
    fn parse_char_test() {
        let char_test = r"#\c";
        let mut lexer = Cursor::new(char_test);
        if let Tokens::Char(c) = lexer.get_next_token() {
            assert_eq!(c, 'c');
            return;
        }
        assert!(UNREACHABLE)
    }

    #[test]
    fn parse_two_tokens() {
        let tokens_test = r"123.456 #t";
        let mut lexer = Cursor::new(tokens_test);
        if let Tokens::Float(f) = lexer.get_next_token() {
            assert_eq!(f, 123.456);
        } else {
            assert!(UNREACHABLE)
        }

        if let Tokens::Boolean(b) = lexer.get_next_token() {
            assert_eq!(b, true)
        } else {
            assert!(UNREACHABLE)
        }
    }

    #[test]
    fn parse_paren_atom() {
        let paren_test = r"(+ 1 2)";
        let mut lexer = Cursor::new(paren_test);

        if let Tokens::LPAREN = lexer.get_next_token() {
        } else {
            unreachable!();
        }

        if let Tokens::Atom(s) = lexer.get_next_token() {
            assert_eq!(s, "+");
        } else {
            unreachable!();
        }

        if let Tokens::Int(1) = lexer.get_next_token() {
        } else {
            unreachable!();
        }

        if let Tokens::Int(2) = lexer.get_next_token() {
        } else {
            unreachable!();
        }

        if let Tokens::RPAREN = lexer.get_next_token() {
        } else {
            unreachable!();
        }
    }
}
