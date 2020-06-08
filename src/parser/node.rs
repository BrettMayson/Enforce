use std::str::FromStr;

#[derive(Debug)]
pub enum Node {
    Enforce(Vec<Node>),
    Bool(bool),
    Int(i32),
    Str(String),
    Ident(String),
    eIf(Box<Node>, Vec<Node>),
    Empty,
    Call {
        ident: String,
        args: Vec<Node>,
    },
    Assignment {
        etype: eType,
        ident: String,
        operator: AssignmentOperator,
        value: Box<Node>,
    },
    ComparisonExpression {
        lhs: Box<Node>,
        rhs: Option<(ComparisonOperator, Box<Node>)>,
    },
    LogicalExpression {
        lhs: Box<Node>,
        rhs: Option<(LogicalOperator, Box<Node>)>,
    },
    LogicVal {
        inverted: bool,
        expr: Box<Node>,
    },
}

#[derive(Debug)]
enum ComparisonOperator {
    GreaterThan,
    LessThan,
    GreaterThanEqualTo,
    LessThanEqualTo,
    EqualEqual,
    NotEqual,
}
impl FromStr for ComparisonOperator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Self::GreaterThan),
            "<" => Ok(Self::LessThan),
            ">=" => Ok(Self::GreaterThanEqualTo),
            "<=" => Ok(Self::LessThanEqualTo),
            "==" => Ok(Self::EqualEqual),
            "!=" => Ok(Self::NotEqual),
            _ => Err(String::from("wtf")),
        }
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

#[derive(Debug)]
enum LogicalOperator {
    OR,
    AND,
}
impl FromStr for LogicalOperator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "||" => Ok(Self::OR),
            "&&" => Ok(Self::AND),
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
            Rule::call => {
                let mut parts = pair.into_inner();
                Node::Call {
                    ident: parts.next().unwrap().as_str().to_string(),
                    args: {
                        let parts = parts.next().unwrap().into_inner();
                        let mut nodes = Vec::new();
                        for part in parts {
                            nodes.push(Node::from_expr(part));
                        }
                        nodes
                    },
                }
            }
            Rule::int => Node::Int(pair.as_str().parse().unwrap()),
            Rule::string => Node::Str(pair.into_inner().next().unwrap().as_str().to_string()),
            Rule::ident => Node::Ident(pair.as_str().to_string()),
            Rule::bool => Node::Bool(pair.as_str().parse().unwrap()),
            Rule::eif => {
                let mut parts = pair.into_inner();
                Node::eIf(Box::new(Node::from_expr(parts.next().unwrap())), {
                    let mut nodes = Vec::new();
                    for part in parts {
                        nodes.push(Node::from_expr(part));
                    }
                    nodes
                })
            }
            Rule::comparison_expr => {
                let mut parts = pair.into_inner();
                Node::ComparisonExpression {
                    lhs: Box::new(Node::from_expr(parts.next().unwrap())),
                    rhs: if let Some(op) = parts.next() {
                        Some((
                            ComparisonOperator::from_str(op.as_str()).unwrap(),
                            Box::new(Node::from_expr(parts.next().unwrap())),
                        ))
                    } else {
                        None
                    },
                }
            }
            Rule::logic_expr => {
                let mut parts = pair.into_inner();
                Node::LogicalExpression {
                    lhs: Box::new(Node::from_expr(parts.next().unwrap())),
                    rhs: if let Some(op) = parts.next() {
                        Some((
                            LogicalOperator::from_str(op.as_str()).unwrap(),
                            Box::new(Node::from_expr(parts.next().unwrap())),
                        ))
                    } else {
                        None
                    },
                }
            }
            Rule::logic_val => {
                let parts: Vec<_> = pair.into_inner().collect();
                Node::LogicVal {
                    inverted: parts.len() == 2,
                    expr: Box::new(Node::from_expr(parts[(parts.len() == 2) as usize].clone())),
                }
            }
            Rule::EOI => Node::Empty,
            _ => unimplemented!("ahh {:#?}", pair),
        }
    }
}
