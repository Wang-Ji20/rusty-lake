mod instruction;
use instruction::*;

mod assembly_builder;
use assembly_builder::AssemblyBuilder;

use std::{error::Error, fs};

pub fn run_compiler_pipeline(
    source_file_name: String,
    output_file_name: String,
) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(source_file_name)?;
    let compiled_source = compile(source);
    fs::write(output_file_name, compiled_source)?;
    Ok(())
}

#[test]
fn run_compiler_pipeline_test() {
    fs::write("sourcetest.scm", "123").unwrap();

    let result = run_compiler_pipeline("sourcetest.scm".to_string(), "outputtest.asm".to_string());

    assert!(result.is_ok());
    let compiled_source = fs::read_to_string("outputtest.asm").unwrap();
    assert_eq!(
        compiled_source,
        ".global scheme\n.type scheme, @function\nscheme:\nmovq $123, %rax\nret\n"
    );

    fs::remove_file("sourcetest.scm").unwrap();
    fs::remove_file("outputtest.asm").unwrap();
}

fn compile(source: String) -> String {
    let mut assembly_builder = AssemblyBuilder::new();
    assembly_builder.new_fn("scheme");
    let parsed_int: i64 = source.parse().unwrap();
    assembly_builder.mov(Mov::ImmediateToRegister(parsed_int, Register::RAX));
    assembly_builder.ret();
    assembly_builder.build()
}

#[test]
fn compile_test() {
    let source = String::from("123");
    let compiled_source = compile(source);
    assert_eq!(
        compiled_source,
        ".global scheme\n.type scheme, @function\nscheme:\nmovq $123, %rax\nret\n"
    );
}
