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
}
