use std::fmt::Display;

use super::{MemAddr, Register};

#[derive(Clone)]
pub enum MovSd {
    MemToRegister(MemAddr, Register),
}

const MOVSD_OP_ATT: &str = "movsd ";

impl Display for MovSd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = MOVSD_OP_ATT.to_string().clone();
        s.push_str(
            match self {
                MovSd::MemToRegister(maddr, reg) => format!("{}, {}", maddr, reg),
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
    fn float_movsd_test() {
        let m = MovSd::MemToRegister(
            MemAddr::LabelDereference("somelabel".to_string(), Register::RIP),
            Register::XMM0,
        );
        assert_eq!(format!("{}", m), "movsd somelabel(%rip), %xmm0")
    }
}
