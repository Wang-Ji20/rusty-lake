use clap::Parser;

/// A small Scheme interpreter, interactive mode on no arguments.
#[derive(Parser)]
struct Cli {
    /// Path to scheme source code. Default to stdin.
    #[clap(short, long)]
    path: Option<std::path::PathBuf>,
    /// Interpret some string
    expr: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let mut interpreter = scheme::interpreter::Interpreter::new();
    if let Some(expr) = args.expr {
        let v = interpreter.interpret(&expr).unwrap();
        println!("{:?}", v);
    } else if let Some(path) = args.path {
        let v = interpreter.interpret_file(path).unwrap();
        println!("{:?}", v);
    } else {
        interpreter.interpret_repl();
    }
}
