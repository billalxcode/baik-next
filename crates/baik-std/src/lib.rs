//! baik-std: builtin functions.
//!
//! Role: replaces funcMatematik.h, funcString.h, funcKata.h, funcArsip.h.
//! Stage 1: provides core functions used by legacy tests.

use baik_value::ValueAlias;
use serde_json::{json, Value};

/// Standard result: Ok value or Err message.
pub type StdResult = Result<ValueAlias, String>;

/// Length of string or array. Replaces BAIK `panjang` keyword.
pub fn len(v: &ValueAlias) -> StdResult {
    match v {
        Value::String(s) => Ok(json!(s.chars().count())),
        Value::Array(a) => Ok(json!(a.len())),
        _ => Err("len requires string or array".to_string()),
    }
}

/// Minimum value from a number list.
pub fn min(values: &[ValueAlias]) -> StdResult {
    let mut out: Option<f64> = None;
    for x in values {
        let n = x
            .as_i64()
            .map(|i| i as f64)
            .or_else(|| x.as_f64())
            .ok_or("min requires numbers".to_string())?;
        out = Some(out.map_or(n, |o: f64| o.min(n)));
    }
    out.map(|x| json!(x))
        .ok_or("min requires one argument".to_string())
}

/// Maximum value from a number list.
pub fn max(values: &[ValueAlias]) -> StdResult {
    let mut out: Option<f64> = None;
    for x in values {
        let n = x
            .as_i64()
            .map(|i| i as f64)
            .or_else(|| x.as_f64())
            .ok_or("max requires numbers".to_string())?;
        out = Some(out.map_or(n, |o: f64| o.max(n)));
    }
    out.map(|x| json!(x))
        .ok_or("max requires one argument".to_string())
}

/// Empty check for string and array. Replaces BAIK `kosong` keyword.
pub fn is_empty(v: &ValueAlias) -> StdResult {
    match v {
        Value::String(s) => Ok(json!(s.is_empty())),
        Value::Array(a) => Ok(json!(a.is_empty())),
        Value::Null => Ok(json!(true)),
        _ => Ok(json!(false)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn len_counts_string() {
        assert_eq!(len(&json!("halo")), Ok(json!(4)));
    }
    #[test]
    fn min_picks_lowest() {
        assert_eq!(min(&[json!(30), json!(5), json!(20)]), Ok(json!(5.0)));
    }
    #[test]
    fn max_picks_highest() {
        assert_eq!(max(&[json!(30), json!(5), json!(20)]), Ok(json!(30.0)));
    }
    #[test]
    fn empty_check() {
        assert_eq!(is_empty(&json!("")), Ok(json!(true)));
        assert_eq!(is_empty(&json!("x")), Ok(json!(false)));
    }
    #[test]
    fn len_counts_array() {
        assert_eq!(len(&json!([1, 2, 3])), Ok(json!(3)));
    }
    #[test]
    fn len_rejects_number() {
        assert_eq!(
            len(&json!(5)),
            Err("len requires string or array".to_string())
        );
    }
    #[test]
    fn min_rejects_empty_and_non_number() {
        assert_eq!(
            min(&[]),
            Err("min requires one argument".to_string())
        );
        assert!(min(&[json!("x")]).is_err());
    }
    #[test]
    fn max_rejects_empty_and_non_number() {
        assert_eq!(
            max(&[]),
            Err("max requires one argument".to_string())
        );
        assert!(max(&[json!(null)]).is_err());
    }
    #[test]
    fn empty_covers_null_array_number_object() {
        assert_eq!(is_empty(&json!(null)), Ok(json!(true)));
        assert_eq!(is_empty(&json!([])), Ok(json!(true)));
        assert_eq!(is_empty(&json!([1])), Ok(json!(false)));
        assert_eq!(is_empty(&json!(0)), Ok(json!(false)));
        assert_eq!(is_empty(&json!({"a": 1})), Ok(json!(false)));
    }
}
