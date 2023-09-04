mod instruction;
use code_generator::CodeGenerator;
use instruction::*;

mod assembly_builder;
use assembly_builder::AssemblyBuilder;
use lexer::Cursor;

mod lexer;

mod code_generator;

mod value;

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
        ".global __scheme__anonymous__function__0\n.type __scheme__anonymous__function__0, @function\n__scheme__anonymous__function__0:\n        movq $123, %rax\n        ret\n"
    );

    fs::remove_file("sourcetest.scm").unwrap();
    fs::remove_file("outputtest.asm").unwrap();
}

fn compile(source: String) -> String {
    let assembly_builder = AssemblyBuilder::new();
    let parser = Cursor::new(&source);
    let mut codegen = CodeGenerator::new(parser, assembly_builder);
    codegen.start().unwrap().build()
}

#[test]
fn compile_test() {
    let source = String::from("123");
    let compiled_source = compile(source);
    assert_eq!(
        compiled_source,
        ".global __scheme__anonymous__function__0\n.type __scheme__anonymous__function__0, @function\n__scheme__anonymous__function__0:\n        movq $123, %rax\n        ret\n"
    );
}
