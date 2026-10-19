//! baik binary: reads an .ina file then runs it via baik-interp.

use std::fs;
use clap::Parser;

/// BAIK v10 CLI. Replaces tbaik.c main.
#[derive(Parser, Debug)]
#[command(name = "baik", version = "10.0.0", about = "BAIK scripting language")]
struct Args {
    /// Input BAIK file (.ina)
    #[arg(value_name = "INPUT")]
    input: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let source = fs::read_to_string(&args.input).expect("file not found");
    let mut stack = vec![baik_value::new_context()];
    match baik_interp::run_simple(&source, &mut stack) {
        Ok(count) => println!("exit: {} lines", count),
        Err(error) => {
            eprintln!("failed: {}", error);
            std::process::exit(1);
        }
    }
    Ok(())
}
