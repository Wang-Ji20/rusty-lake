use std::fmt::Display;

#[derive(Clone)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
    RIP,
    XMM0,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let register_name = match self {
            Register::RAX => "%rax",
            Register::RBX => "%rbx",
            Register::RCX => "%rcx",
            Register::RDX => "%rdx",
            Register::RSI => "%rsi",
            Register::RDI => "%rdi",
            Register::RSP => "%rsp",
            Register::RBP => "%rbp",
            Register::RIP => "%rip",
            Register::XMM0 => "%xmm0",
        };
        write!(f, "{}", register_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rax_test() {
        let register = Register::RAX;
        assert_eq!(register.to_string(), "%rax");
    }

    #[test]
    fn rbx_test() {
        let register = Register::RBX;
        assert_eq!(register.to_string(), "%rbx");
    }

    #[test]
    fn rcx_test() {
        let register = Register::RCX;
        assert_eq!(register.to_string(), "%rcx");
    }

    #[test]
    fn rdx_test() {
        let register = Register::RDX;
        assert_eq!(register.to_string(), "%rdx");
    }

    #[test]
    fn rsi_test() {
        let register = Register::RSI;
        assert_eq!(register.to_string(), "%rsi");
    }

    #[test]
    fn rdi_test() {
        let register = Register::RDI;
        assert_eq!(register.to_string(), "%rdi");
    }

    #[test]
    fn rsp_test() {
        let register = Register::RSP;
        assert_eq!(register.to_string(), "%rsp");
    }

    #[test]
    fn rbp_test() {
        let register = Register::RBP;
        assert_eq!(register.to_string(), "%rbp");
    }

    #[test]
    fn rip_test() {
        let register = Register::RIP;
        assert_eq!(register.to_string(), "%rip");
    }
}
