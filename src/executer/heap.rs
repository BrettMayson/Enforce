use super::Scope;
use crate::parser::Node;

use std::collections::HashMap;

struct Heap {
  scopes: Vec<HashMap<String, Box<Node>>>,
}

impl Heap {
  fn get_variable(&self, key: &str) -> Node {
    for scope in self.scopes.into_iter().rev() {
      if scope.
    }
  }
}
