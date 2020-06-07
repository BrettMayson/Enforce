use std::str::FromStr;

#[derive(Debug)]
pub enum Node {
  Enforce(Vec<Node>),
  Bool(bool),
  Int(i32),
  Str(String),
  Call {
    ident: String,
    args: Vec<Node>,
  },
  Assignment {
    etype: eType,
    ident: String,
    operator: AssignmentOperator,
    value: Box<Node>,
  }
}

#[derive(Debug)]
enum eType {
  Int,
  String,
  Bool,
}
impl FromStr for eType {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "int" => Ok(Self::Int),
      "string" => Ok(Self::String),
      "bool" => Ok(Self::Bool),
      _ => Err(String::from("wtf")),
    }
  }
}

#[derive(Debug)]
enum AssignmentOperator {
  Equal,
  AddEqual,
}
impl FromStr for AssignmentOperator {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "=" => Ok(Self::Equal),
      "+=" => Ok(Self::AddEqual),
      _ => Err(String::from("wtf")),
    }
  }
}

use super::Rule;

impl Node {
  pub fn from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
      Rule::enforce => {
        let parts = pair.into_inner();
        let mut nodes = Vec::new();
        for part in parts {
          nodes.push(Node::from_expr(part));
        }
        Self::Enforce(nodes)
      }
      Rule::assignment => {
        let mut parts = pair.into_inner();
        Node::Assignment {
          etype: eType::from_str(parts.next().unwrap().as_str()).unwrap(),
          ident: parts.next().unwrap().as_str().to_string(),
          operator: AssignmentOperator::from_str(parts.next().unwrap().as_str()).unwrap(),
          value: Box::new(Node::from_expr(parts.next().unwrap())),
        }
      }
      Rule::int => {
        Node::Int(pair.as_str().parse().unwrap())
      }
      _ => unimplemented!("ahh {:#?}", pair)
    }
  }
}
