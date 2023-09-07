use std::fmt::Display;

use crate::{
    instruction::{MemAddr, Register},
    lexer::Tokens,
};

#[derive(Clone, Debug)]
pub struct Value {
    literal: Tokens,
    pub label: String,
}

impl Value {
    pub fn new(literal: Tokens, label: String) -> Value {
        Value {
            literal: literal,
            label: label,
        }
    }

    pub fn get_address(&self) -> MemAddr {
        MemAddr::LabelDereference(self.label.clone(), Register::RIP)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_value = match self.literal {
            Tokens::Int(i) => format!(".quad   {}", i),
            Tokens::Float(f) => format!(".quad   {:#x}", f.to_bits()),
            Tokens::Boolean(b) => format!(".byte   {}", b as u64),
            Tokens::Char(c) => format!(".long {}", c as u32),
            _ => todo!(),
        };
        write!(f, "{}:\n    {}\n", self.label, type_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_value() {
        let v = Value::new(Tokens::Float(2.0), "LC_0".to_string());
        assert_eq!(
            format!("{}", v),
            r"LC_0:
    .quad   0x4000000000000000
"
        );
    }
}
