//! baik-value: core value and context types.
//!
//! Role: replaces VAL_LABEL in baik_struct.h:150.
//! Runtime value uses serde_json::Value for compat with legacy code.

use std::collections::HashMap;
use serde_json::Value;

/// BAIK runtime value: null, bool, number, string, array, object.
pub type ValueAlias = Value;
/// Single variable scope: name to value.
pub type Context = HashMap<String, ValueAlias>;
/// Scope stack. Index 0 is the global scope.
pub type ContextStack = Vec<Context>;
/// User function table: name to body hash.
pub type FunctionTable = HashMap<String, String>;

/// Create an empty global context.
pub fn new_context() -> Context {
    HashMap::new()
}

/// Look up a variable from top to bottom of the stack.
pub fn lookup(stack: &[Context], name: &str) -> Option<ValueAlias> {
    for ctx in stack.iter().rev() {
        if let Some(v) = ctx.get(name) {
            return Some(v.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn lookup_finds_top_scope() {
        let mut base = new_context();
        base.insert("x".to_string(), json!(1));
        let mut top = new_context();
        top.insert("x".to_string(), json!(2));
        let stack = vec![base, top];
        assert_eq!(lookup(&stack, "x"), Some(json!(2)));
    }
    #[test]
    fn lookup_returns_none_for_missing() {
        let stack = vec![new_context()];
        assert_eq!(lookup(&stack, "missing"), None);
    }
    #[test]
    fn lookup_empty_stack_returns_none() {
        let stack: Vec<Context> = vec![];
        assert_eq!(lookup(&stack, "x"), None);
    }
    #[test]
    fn lookup_three_layers_prefers_top() {
        let mut first = new_context();
        first.insert("a".to_string(), json!(1));
        first.insert("shared".to_string(), json!("first"));
        let mut second = new_context();
        second.insert("b".to_string(), json!(2));
        second.insert("shared".to_string(), json!("second"));
        let mut third = new_context();
        third.insert("c".to_string(), json!(3));
        third.insert("shared".to_string(), json!("third"));
        let stack = vec![first, second, third];
        assert_eq!(lookup(&stack, "a"), Some(json!(1)));
        assert_eq!(lookup(&stack, "b"), Some(json!(2)));
        assert_eq!(lookup(&stack, "c"), Some(json!(3)));
        assert_eq!(lookup(&stack, "shared"), Some(json!("third")));
        assert_eq!(lookup(&stack, "absent"), None);
    }
}
