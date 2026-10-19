//! baik-interp: statement execution for .ina files.
//!
//! Role: replaces interpreter.c:1531 and key headers.
//! Stage 1 supports: TULIS print and TAMAT end.

use baik_value::{lookup, Context, ValueAlias};
use serde_json::json;

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
        if lower.starts_with("tulis") {
            let args = text[5..].trim().trim_matches(';').to_string();
            let ctx = stack.last().cloned().unwrap_or_default();
            let rendered = render_write(&args, &ctx);
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
    if item.len() >= 2 && item.starts_with('"') && item.ends_with('"') {
        return item[1..item.len() - 1].replace("\\n", "\n");
    }
    if let Some(v) = ctx.get(item) {
        return display(v);
    }
    let single = vec![ctx.clone()];
    if let Some(v) = lookup(&single, item) {
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
}
