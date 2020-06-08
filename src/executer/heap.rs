use crate::parser::Node;

use std::collections::HashMap;

pub struct Heap {
    scopes: Vec<HashMap<String, Box<Node>>>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }
    pub fn lower(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn higher(&mut self) {
        self.scopes.remove(self.scopes.len() - 1);
    }
    pub fn get_variable(&self, key: &str) -> Option<&Node> {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(key) {
                return Some(scope.get(key).unwrap());
            }
        }
        None
    }

    pub fn set_variable(&mut self, key: &str, value: Box<Node>, declare: bool) -> Result<(), ()> {
        let last = self.scopes.len() - 1;
        if declare {
            self.scopes[last].insert(key.to_string(), value);
            return Ok(());
        } else {
            for scope in self.scopes.iter_mut().rev() {
                if scope.contains_key(key) {
                    scope.insert(key.to_string(), value);
                    return Ok(());
                }
            }
        }
        Err(())
    }
}
