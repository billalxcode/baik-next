//! baik-syntax: grammar and lexer.
//!
//! Role: replaces C lexer baik_lexer.h and baik_ident.c.
//! Stage 1: re-export legacy pest grammar so old build keeps working.
//! Stage 2: move src/grammar/baik.pest here fully.

use pest_derive::Parser;

/// Pest based lexer.
#[derive(Parser)]
#[grammar = "baik.pest"]
pub struct BaikLexer;

/// Check syntax without execution. Returns Ok on successful parse.
pub fn check_syntax(source: &str) -> Result<(), String> {
    use pest::Parser;
    BaikLexer::parse(Rule::input, source).map(|_| ()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_tulis_line() {
        assert!(check_syntax("tulis \"halo\"").is_ok());
    }
    #[test]
    fn rejects_broken_input() {
        assert!(check_syntax("tulis \"unclosed").is_err());
        assert!(check_syntax("### unclosed comment").is_err());
    }
}
