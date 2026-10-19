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

fn load_source(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("cannot read file {}: {}", path, e))
}

fn run_path(path: &str) -> Result<usize, String> {
    let source = load_source(path)?;
    let mut stack = vec![baik_value::new_context()];
    baik_interp::run_simple(&source, &mut stack)
}

fn main() {
    let args = Args::parse();
    match run_path(&args.input) {
        Ok(count) => println!("exit: {} lines", count),
        Err(error) => {
            eprintln!("failed: {}", error);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_file_returns_err_without_panic() {
        let result = run_path("no_such_file_missing_12345.ina");
        assert!(result.is_err());
        match result {
            Ok(_) => assert!(false, "expected error for missing file"),
            Err(message) => assert!(message.contains("cannot read file")),
        }
    }
    #[test]
    fn load_missing_file_returns_err() {
        let result = load_source("no_such_file_missing_12345.ina");
        assert!(result.is_err());
    }
}
