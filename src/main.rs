use std::path::PathBuf;

use brainfuck::Brainfuck;
use clap::{Parser, ArgGroup};


#[derive(Parser, Debug, Default)]
#[clap(author="Katie Janzen", version, about)]
#[clap(group(
    ArgGroup::new("input")
        .required(false)
        .args(&["script", "filename"]),
))]
/// A Brainfuck Interpreter, written in Rust
struct Arguments {
    /// Path to a Brainfuck source file
    filename: Option<PathBuf>,

    #[clap(short='e')]
    /// Code to execute
    script: Option<String>,
}

fn main() {
    let args = Arguments::parse();
    let mut bf = Brainfuck::new();

    if let Some(path) = args.filename {
        match std::fs::read_to_string(path) {
            Ok(src) => bf.exec(&src),
            Err(e) => eprintln!("{}", e)
        }
    }

    if let Some(src) = args.script {
        bf.exec(&src);
    }
}