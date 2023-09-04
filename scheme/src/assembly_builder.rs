use crate::{instruction, lexer::Literals, value::Value};
use instruction::*;

pub struct AssemblyBuilder {
    code_section: Vec<String>,
    data_section: Vec<Value>,
}

impl AssemblyBuilder {
    pub fn new() -> AssemblyBuilder {
        AssemblyBuilder {
            code_section: Vec::new(),
            data_section: Vec::new(),
        }
    }

    fn add(&mut self, assembly_code: String) {
        self.code_section.push(assembly_code);
    }

    pub fn build(&self) -> String {
        let mut result = String::new();
        self.push_instructions(&mut result);
        self.push_data(&mut result);
        result
    }

    fn push_data(&self, result: &mut String) {
        for program_data in &self.data_section {
            result.push_str(format!("{}", program_data).as_str());
        }
    }

    fn push_instructions(&self, result: &mut String) {
        for assembly_code in &self.code_section {
            let indent_spaces = match assembly_code.starts_with('.') || assembly_code.ends_with(':')
            {
                true => "",
                false => "        ",
            };
            result.push_str(format!("{}{}\n", indent_spaces, assembly_code).as_str());
        }
    }

    pub fn mov(&mut self, mov_instr: Mov) {
        let mov_code = format!("{}", mov_instr);
        self.add(mov_code);
    }

    pub fn ret(&mut self) {
        self.add(String::from("ret"));
    }

    pub fn new_fn(&mut self, name: &str) {
        self.add(format!(".global {}", name));
        self.add(format!(".type {}, @function", name));
        self.add(format!("{}:", name));
    }

    pub fn new_float(&mut self, f: Literals) -> &Value {
        self.data_section
            .push(Value::new(f, format!("LC_{}", self.data_section.len())));
        self.data_section.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let assembly_builder = AssemblyBuilder::new();
        assert_eq!(assembly_builder.code_section.len(), 0);
        assert_eq!(assembly_builder.data_section.len(), 0);
    }

    #[test]
    fn add_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.add(String::from("movq $123, %rax"));
        assert_eq!(assembly_builder.code_section.len(), 1);
        assert_eq!(assembly_builder.code_section[0], "movq $123, %rax");
    }

    #[test]
    fn build_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.add(String::from("movq $123, %rax"));
        assembly_builder.add(String::from("ret"));
        let result = assembly_builder.build();
        assert_eq!(result, "        movq $123, %rax\n        ret\n");
    }

    #[test]
    fn indent_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.mov(Mov::ImmediateToRegister(123, Register::RAX));
        let result = assembly_builder.build();
        assert_eq!(result, "        movq $123, %rax\n");
    }

    #[test]
    fn noindent_indent_test() {
        let mut assembly_builder = AssemblyBuilder::new();
        assembly_builder.new_fn("scheme");
        assembly_builder.ret();
        let result = assembly_builder.build();
        assert_eq!(
            result,
            ".global scheme\n.type scheme, @function\nscheme:\n        ret\n"
        );
    }
}
