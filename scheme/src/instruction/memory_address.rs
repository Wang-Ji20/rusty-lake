use crate::instruction::register::Register;
use std::fmt::Display;

pub enum MemAddr {
    Address(i64),
    OffsetDereference(i64, Register),
    LabelDereference(String, Register),
}

impl Display for MemAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mem_addr = match self {
            MemAddr::Address(addr) => format!("{:#x}", addr),
            MemAddr::OffsetDereference(offset, base) => format!("{}({})", offset, base),
            MemAddr::LabelDereference(label, register) => format!("{}({})", label, register),
        };
        write!(f, "{}", mem_addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_addr_test() {
        let mem_addr = MemAddr::Address(0);
        assert_eq!(mem_addr.to_string(), "0x0");

        let mem_addr = MemAddr::OffsetDereference(0, Register::RAX);
        assert_eq!(mem_addr.to_string(), "0(%rax)");

        let mem_addr = MemAddr::LabelDereference(".LC".to_string(), Register::RIP);
        assert_eq!(mem_addr.to_string(), ".LC(%rip)")
    }
}
