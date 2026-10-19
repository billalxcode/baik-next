//! baik-interp: statement execution for .ina files.
//!
//! Role: replaces interpreter.c:1531 and key headers.
//! Stage 1 supports: TULIS print and TAMAT end.

use baik_value::{lookup, Context, ValueAlias};
use serde_json::json;

/// Length of one quote character.
const QUOTE_LEN: usize = 1;
/// Keyword for print statement.
const TULIS_KEYWORD: &str = "tulis";

/// Strip TULIS keyword case-insensitively and return trimmed args.
/// Returns None when text does not start with the keyword.
fn strip_tulis_args(text: &str) -> Option<&str> {
    let head = text.get(..TULIS_KEYWORD.len())?;
    if head.eq_ignore_ascii_case(TULIS_KEYWORD) {
        text.get(TULIS_KEYWORD.len()..)
            .map(|rest| rest.trim().trim_matches(';'))
    } else {
        None
    }
}

/// Stage 1 statement set.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Print value list to stdout. Maps BAIK TULIS.
    Write(Vec<String>),
    /// End of program. Maps BAIK TAMAT or END.
    End,
}

/// Run a short .ina script. Supports TULIS and TAMAT only.
/// Skips empty lines and hash comments. Returns executed line count.
pub fn run_simple(source: &str, stack: &mut Vec<Context>) -> Result<usize, String> {
    if stack.is_empty() {
        stack.push(Context::new());
    }
    let mut count = 0;
    for line in source.lines() {
        let text = line.trim();
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        let lower = text.to_lowercase();
        if lower == "tamat" || lower == "end" {
            count += 1;
            break;
        }
        if let Some(args) = strip_tulis_args(text) {
            let empty = Context::new();
            let ctx: &Context = match stack.last() {
                Some(item) => item,
                None => &empty,
            };
            let rendered = render_write(args, ctx);
            println!("{}", rendered);
            count += 1;
            continue;
        }
        // Other statements are skipped in stage 1 so legacy files still parse.
        count += 1;
    }
    Ok(count)
}

/// Render TULIS args: split commas outside quotes, resolve variables.
fn render_write(args: &str, ctx: &Context) -> String {
    let mut out = String::new();
    let mut current = String::new();
    let mut in_quote = false;
    for c in args.chars() {
        match c {
            '"' => {
                in_quote = !in_quote;
                current.push(c);
            }
            ',' if !in_quote => {
                out.push_str(&render_arg(&current, ctx));
                current.clear();
            }
            _ => current.push(c),
        }
    }
    out.push_str(&render_arg(&current, ctx));
    out
}

fn render_arg(arg: &str, ctx: &Context) -> String {
    let item = arg.trim();
    if item.len() >= QUOTE_LEN * 2 {
        if let Some(inner) = item
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
        {
            return inner.replace("\\n", "\n");
        }
    }
    if let Some(v) = ctx.get(item) {
        return display(v);
    }
    let single: &[Context] = std::slice::from_ref(ctx);
    if let Some(v) = lookup(single, item) {
        return display(&v);
    }
    item.to_string()
}

fn display(value: &ValueAlias) -> String {
    match value {
        ValueAlias::String(s) => s.clone(),
        ValueAlias::Number(n) => n.to_string(),
        ValueAlias::Bool(b) => b.to_string(),
        ValueAlias::Null => "".to_string(),
        _ => json!(value).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_counts_tulis_and_end() {
        let mut stack = vec![Context::new()];
        let src = "tulis \"halo\"\ntamat";
        assert_eq!(run_simple(src, &mut stack), Ok(2));
    }
    #[test]
    fn run_skips_comments() {
        let mut stack = vec![Context::new()];
        let src = "# comment\ntamat";
        assert_eq!(run_simple(src, &mut stack), Ok(1));
    }
    #[test]
    fn strip_tulis_handles_case_and_empty() {
        assert_eq!(strip_tulis_args("TULIS \"hi\";"), Some("\"hi\""));
        assert_eq!(strip_tulis_args("tulis"), Some(""));
        assert_eq!(strip_tulis_args("tul"), None);
        assert_eq!(strip_tulis_args(""), None);
    }
    #[test]
    fn render_write_joins_quoted_and_variable() {
        let mut ctx = Context::new();
        ctx.insert("nama".to_string(), json!("budi"));
        assert_eq!(render_write("\"halo\", nama", &ctx), "halobudi");
        assert_eq!(render_write("\"a, b\"", &ctx), "a, b");
    }
    #[test]
    fn render_arg_boundary_cases() {
        let ctx = Context::new();
        assert_eq!(render_arg("\"\"", &ctx), "");
        assert_eq!(render_arg("\"", &ctx), "\"");
        assert_eq!(render_arg("", &ctx), "");
        assert_eq!(render_arg("\"hi", &ctx), "\"hi");
        assert_eq!(render_arg("\"a\\nb\"", &ctx), "a\nb");
    }
    #[test]
    fn display_covers_variants() {
        assert_eq!(display(&json!("s")), "s");
        assert_eq!(display(&json!(7)), "7");
        assert_eq!(display(&json!(true)), "true");
        assert_eq!(display(&json!(null)), "");
        assert_eq!(display(&json!([1, 2])), "[1,2]");
        assert_eq!(display(&json!({"a": 1})), "{\"a\":1}");
    }
}
