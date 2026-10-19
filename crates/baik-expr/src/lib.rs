//! baik-expr: single line expression evaluator.
//!
//! Role: replaces baik_expression.c, baik_compare.c, baik_factor.c.
//! Legacy code in src/tree, src/node, src/operator, src/expr moves here gradually.

use baik_value::ValueAlias;
use serde_json::json;

/// Evaluation result: Ok value or Err message.
pub type ExprResult = Result<ValueAlias, String>;

/// Basic arithmetic for early release.
/// Supports add, sub, mul, div via f64. String add means concat.
pub fn eval_simple(op: &str, a: &ValueAlias, b: &ValueAlias) -> ExprResult {
    match op {
        "+" => add(a, b),
        "-" => sub(a, b),
        "*" => mul(a, b),
        "/" => div(a, b),
        _ => Err(format!("unsupported operator: {}", op)),
    }
}

fn as_number(v: &ValueAlias) -> Option<f64> {
    v.as_i64().map(|x| x as f64).or_else(|| v.as_f64())
}

fn add(a: &ValueAlias, b: &ValueAlias) -> ExprResult {
    if let (Some(x), Some(y)) = (as_number(a), as_number(b)) {
        let r = x + y;
        if a.is_i64() && b.is_i64() && r.fract() == 0.0 {
            return Ok(json!(r as i64));
        }
        return Ok(json!(r));
    }
    if let (Some(x), Some(y)) = (a.as_str(), b.as_str()) {
        return Ok(json!(format!("{}{}", x, y)));
    }
    Err("add requires numbers or strings".to_string())
}

fn sub(a: &ValueAlias, b: &ValueAlias) -> ExprResult {
    match (as_number(a), as_number(b)) {
        (Some(x), Some(y)) => Ok(json!(x - y)),
        _ => Err("sub requires numbers".to_string()),
    }
}

fn mul(a: &ValueAlias, b: &ValueAlias) -> ExprResult {
    match (as_number(a), as_number(b)) {
        (Some(x), Some(y)) => Ok(json!(x * y)),
        _ => Err("mul requires numbers".to_string()),
    }
}

fn div(a: &ValueAlias, b: &ValueAlias) -> ExprResult {
    match (as_number(a), as_number(b)) {
        (Some(_), Some(0.0)) => Err("division by zero".to_string()),
        (Some(x), Some(y)) => Ok(json!(x / y)),
        _ => Err("div requires numbers".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn add_integers() {
        assert_eq!(eval_simple("+", &json!(2), &json!(3)), Ok(json!(5)));
    }
    #[test]
    fn concat_strings() {
        assert_eq!(eval_simple("+", &json!("a"), &json!("b")), Ok(json!("ab")));
    }
    #[test]
    fn div_by_zero_fails() {
        assert!(eval_simple("/", &json!(1), &json!(0)).is_err());
    }
    #[test]
    fn unknown_operator_fails() {
        assert!(eval_simple("%", &json!(1), &json!(2)).is_err());
    }
}
