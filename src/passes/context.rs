use std::collections::HashMap;

use crate::syntax::ast::Ty;

#[derive(Debug, Clone)]
pub struct ContextStack {
    scopes: Vec<HashMap<String, Ty>>,
}

impl ContextStack {
    pub fn new() -> Self {
        ContextStack {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, name: String, ty: Ty) {
        self.scopes.last_mut().unwrap().insert(name, ty);
    }

    pub fn get(&self, name: &str) -> Option<&Ty> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty);
            }
        }
        None
    }
}
