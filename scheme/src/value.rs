use std::fmt::Display;

use crate::{
    instruction::{MemAddr, Register},
    lexer::Literals,
};

#[derive(Clone, Debug)]
pub struct Value {
    literal: Literals,
    pub label: String,
}

impl Value {
    pub fn new(literal: Literals, label: String) -> Value {
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
            Literals::Int(i) => format!(".quad   {}", i),
            Literals::Float(f) => format!(".quad   {:#x}", f.to_bits()),
            Literals::Boolean(b) => format!(".byte   {}", b as u64),
            Literals::Char(c) => format!(".long {}", c as u32),
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
        let v = Value::new(Literals::Float(2.0), "LC_0".to_string());
        assert_eq!(
            format!("{}", v),
            r"LC_0:
    .quad   0x4000000000000000
"
        );
    }
}
