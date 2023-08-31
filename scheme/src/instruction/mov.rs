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

impl Display for Mov {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mov_code = match self {
            Mov::RegisterToRegister(src, dest) => format!("movq {}, {}", src, dest),
            Mov::ImmediateToRegister(src, dest) => format!("movq ${}, {}", src, dest),
            Mov::RegisterToMem(src, dest) => format!("movq {}, {}", src, dest),
            Mov::ImmediateToMem(src, dest) => format!("movq ${}, {}", src, dest),
            Mov::MemToRegister(src, dest) => format!("movq {}, {}", src, dest),
        };
        write!(f, "{}", mov_code)
    }
}
