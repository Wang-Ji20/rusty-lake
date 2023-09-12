use clap::Parser;
use scheme::run_compiler_pipeline;

fn main() {
    let args = Cli::parse();

    if let Err(e) = run_compiler_pipeline(args.source_name, args.output_name) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}

/// A small Scheme compiler.
#[derive(Parser)]
struct Cli {
    /// source of your scheme file.
    #[arg(short, long)]
    source_name: String,
    /// output of your scheme file. default to output.asm
    #[arg(short, long, default_missing_value = "output.asm")]
    output_name: String,
}
