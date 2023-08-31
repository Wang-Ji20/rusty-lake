use crate::instruction;
use instruction::*;

pub struct AssemblyBuilder {
    assembly_codes: Vec<String>,
    nest_count: i32,
}

impl AssemblyBuilder {
    pub fn new() -> AssemblyBuilder {
        AssemblyBuilder {
            assembly_codes: Vec::new(),
            nest_count: 0,
        }
    }

    fn add(&mut self, assembly_code: String) {
        self.assembly_codes.push(assembly_code);
    }

    pub fn build(self) -> String {
        self.validate().unwrap();
        let mut result = String::new();
        for assembly_code in self.assembly_codes {
            result.push_str(&assembly_code);
            result.push_str("\n");
        }
        result
    }

    pub fn mov(&mut self, mov_instr: Mov) {
        let mov_code = format!("{}", mov_instr);
        self.add(mov_code);
    }

    pub fn ret(&mut self) {
        self.add(String::from("ret"));
        self.nest_count -= 1;
    }

    pub fn new_fn(&mut self, name: &str) {
        self.add(format!(".global {}", name));
        self.add(format!(".type {}, @function", name));
        self.add(format!("{}:", name));
        self.nest_count += 1;
    }

    fn validate(&self) -> Result<(), String> {
        if self.nest_count != 0 {
            return Err(String::from("nest_count is not zero. you are inside some expressions, thus cannot build assembly"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let assembly_builder = AssemblyBuilder::new();
        assert_eq!(assembly_builder.assembly_codes.len(), 0);
        assert_eq!(assembly_builder.nest_count, 0);
    }

    #[test]
    fn add_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.add(String::from("movq $123, %rax"));
        assert_eq!(assembly_builder.assembly_codes.len(), 1);
        assert_eq!(assembly_builder.assembly_codes[0], "movq $123, %rax");
    }

    #[test]
    fn build_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.add(String::from("movq $123, %rax"));
        assembly_builder.add(String::from("ret"));
        let result = assembly_builder.build();
        assert_eq!(result, "movq $123, %rax\nret\n");
    }

    #[test]
    fn mov_ir_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.mov(Mov::ImmediateToRegister(123, Register::RAX));
        let result = assembly_builder.build();
        assert_eq!(result, "movq $123, %rax\n");
    }

    #[test]
    fn mov_rr_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.mov(Mov::RegisterToRegister(Register::RAX, Register::RBX));
        let result = assembly_builder.build();
        assert_eq!(result, "movq %rax, %rbx\n");
    }

    #[test]
    fn mov_rm_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.mov(Mov::RegisterToMem(Register::RAX, MemAddr::Address(0x123)));
        let result = assembly_builder.build();
        assert_eq!(result, "movq %rax, 0x123\n");
    }

    #[test]
    fn new_fn_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.new_fn("scheme");
        assembly_builder.ret();
        let result = assembly_builder.build();
        assert_eq!(
            result,
            ".global scheme\n.type scheme, @function\nscheme:\nret\n"
        );
    }

    #[test]
    fn validate_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.new_fn("scheme");
        assembly_builder.ret();
        let result = assembly_builder.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn validate_error_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.new_fn("scheme");
        assembly_builder.mov(Mov::ImmediateToRegister(123, Register::RAX));
        let result = assembly_builder.validate();
        assert!(result.is_err());
    }
}
