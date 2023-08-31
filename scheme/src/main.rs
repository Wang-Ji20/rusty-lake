use std::{env, error::Error, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        show_usage();
        std::process::exit(1);
    });

    if let Err(e) = run_compiler_pipeline(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn show_usage() {
    println!("Usage: scheme <source> <output>");
}
struct Config {
    source_name: String,
    output_name: String,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let source_name = args[1].clone();
        let output_name = args[2].clone();

        Ok(Config {
            source_name,
            output_name,
        })
    }
}

#[test]
fn build_test() {
    let mock_args = vec![
        String::from("scheme"),
        String::from("source.scm"),
        String::from("output.asm"),
    ];
    let config = Config::build(mock_args).unwrap();
    assert_eq!(config.source_name, "source.scm");
    assert_eq!(config.output_name, "output.asm");
}

fn run_compiler_pipeline(config: Config) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(config.source_name)?;
    let compiled_source = compile(source);
    fs::write(config.output_name, compiled_source)?;
    Ok(())
}

#[test]
fn run_compiler_pipeline_test() {
    fs::write("sourcetest.scm", "123").unwrap();
    let config = Config {
        source_name: String::from("sourcetest.scm"),
        output_name: String::from("outputtest.asm"),
    };
    let result = run_compiler_pipeline(config);
    assert!(result.is_ok());
    let compiled_source = fs::read_to_string("outputtest.asm").unwrap();
    assert_eq!(
        compiled_source,
        ".global scheme\n.type scheme, @function\nscheme:\nmovq $123, %rax\nret"
    );
    fs::remove_file("sourcetest.scm").unwrap();
    fs::remove_file("outputtest.asm").unwrap();
}

fn compile(source: String) -> String {
    let parsed_int: i64 = source.parse().unwrap();
    format!(
        ".global scheme\n.type scheme, @function\nscheme:\nmovq ${}, %rax\nret",
        parsed_int
    )
}

#[test]
fn compile_test() {
    let source = String::from("123");
    let compiled_source = compile(source);
    assert_eq!(
        compiled_source,
        ".global scheme\n.type scheme, @function\nscheme:\nmovq $123, %rax\nret"
    );
}
