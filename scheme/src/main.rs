use scheme::run_compiler_pipeline;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        show_usage();
        std::process::exit(1);
    });

    if let Err(e) = run_compiler_pipeline(config.source_name, config.output_name) {
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
