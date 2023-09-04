use crate::lexer::Literals;
use crate::value::Value;
use crate::Mov;
use crate::Register;
use crate::{assembly_builder::AssemblyBuilder, lexer::Cursor};

///! Code Generator receives the high level structure of scheme code,
///! it does not deal with the assembly code, only structure.
pub struct CodeGenerator<'a> {
    parser: Cursor<'a>,
    asm_builder: AssemblyBuilder,
    function_count: u64,
    nested_count: u64,
}

impl CodeGenerator<'_> {
    pub fn new(parser: Cursor, builder: AssemblyBuilder) -> CodeGenerator {
        CodeGenerator {
            parser: parser,
            asm_builder: builder,
            function_count: 0,
            nested_count: 0,
        }
    }

    pub fn start(&mut self) -> Result<&Self, &'static str> {
        loop {
            match self.parser.get_next_token() {
                crate::lexer::Literals::Int(i) => {
                    self.int(i);
                }
                s @ crate::lexer::Literals::Float(_) => {
                    self.float(s);
                }
                crate::lexer::Literals::Boolean(_) => todo!(),
                crate::lexer::Literals::Char(_) => todo!(),
                crate::lexer::Literals::Unknown => break Err("unknown token\n"),
                crate::lexer::Literals::EOF => break Ok(self),
            }
        }
    }

    fn new_fn(&mut self, s: &str) -> &mut Self {
        self.asm_builder.new_fn(s);
        self.nested_count += 1;
        self
    }

    fn new_anonymous_fn(&mut self) {
        let s = format!("__scheme__anonymous__function__{}", self.function_count);
        self.function_count += 1;
        self.new_fn(&s);
    }

    fn ret(&mut self) -> &mut Self {
        self.asm_builder.ret();
        self.nested_count -= 1;
        self
    }

    fn int(&mut self, i: i64) -> &mut Self {
        self.new_anonymous_fn();
        self.asm_builder
            .mov(Mov::ImmediateToRegister(i, Register::RAX));
        self.ret();
        self
    }

    fn float(&mut self, f: Literals) -> &Value {
        self.asm_builder.new_float(f)
    }

    fn validate(&self) {
        assert!(self.nested_count == 0);
    }

    pub fn build(&self) -> String {
        self.validate();
        self.asm_builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_function_test() {
        let mut codegen = CodeGenerator::new(Cursor::new(""), AssemblyBuilder::new());
        let result = codegen.new_fn("scheme").ret().build();
        assert_eq!(
            result,
            ".global scheme\n.type scheme, @function\nscheme:\n        ret\n"
        )
    }
}
