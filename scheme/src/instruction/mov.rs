use std::fmt::Display;

use crate::instruction::memory_address::MemAddr;
use crate::instruction::register::Register;

pub enum Mov {
    RegisterToRegister(Register, Register),
    ImmediateToRegister(i64, Register),
    RegisterToMem(Register, MemAddr),
    ImmediateToMem(i64, MemAddr),
    MemToRegister(MemAddr, Register),
}

const MOV_OP_ATT: &str = "movq ";

impl Display for Mov {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = MOV_OP_ATT.to_string().clone();
        s.push_str(
            match self {
                Mov::RegisterToRegister(src, dest) => format!("{}, {}", src, dest),
                Mov::ImmediateToRegister(src, dest) => format!("${}, {}", src, dest),
                Mov::RegisterToMem(src, dest) => format!("{}, {}", src, dest),
                Mov::ImmediateToMem(src, dest) => format!("${}, {}", src, dest),
                Mov::MemToRegister(src, dest) => format!("{}, {}", src, dest),
            }
            .as_str(),
        );
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mov_instr_test() {
        let m = Mov::ImmediateToRegister(12345, Register::RAX);
        assert_eq!(m.to_string(), "movq $12345, %rax")
    }
}
