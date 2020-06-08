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
        etype: Option<eType>,
        ident: String,
        operator: AssignmentOperator,
        value: Box<Node>,
    },
    ComparisonExpression {
        lhs: Box<Node>,
        rhs: Option<(ComparisonOperator, Box<Node>)>,
    },
    LogicalExpression(Vec<(LogicalOperator, Box<Node>)>),
    LogicVal {
        inverted: bool,
        expr: Box<Node>,
    },
}

#[derive(Debug)]
pub enum ComparisonOperator {
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
pub enum eType {
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
pub enum AssignmentOperator {
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
pub enum LogicalOperator {
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
                let parts: Vec<_> = pair.into_inner().collect();
                let size = parts.len();
                let mut parts = parts.into_iter();
                Node::Assignment {
                    etype: if size == 4 {
                        Some(eType::from_str(parts.next().unwrap().as_str()).unwrap())
                    } else {
                        None
                    },
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
                Node::LogicalExpression({
                    let mut extra = Vec::new();
                    extra.push((
                        LogicalOperator::AND,
                        Box::new(Node::from_expr(parts.next().unwrap())),
                    ));
                    while parts.peek().is_some() {
                        extra.push((
                            LogicalOperator::from_str(parts.next().unwrap().as_str()).unwrap(),
                            Box::new(Node::from_expr(parts.next().unwrap())),
                        ));
                    }
                    extra
                })
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

    pub fn add(&self, other: &Node) -> Node {
        use Node::*;
        match (self, other) {
            (Int(a), Int(b)) => Int(a + b),
            (Str(a), Str(b)) => Str({
                let mut r = a.to_string();
                r.push_str(&b);
                r
            }),
            (Int(a), Str(b)) => Str({
                let mut r = a.to_string();
                r.push_str(&b);
                r
            }),
            (Str(a), Int(b)) => Str({
                let mut r = a.to_string();
                r.push_str(&b.to_string());
                r
            }),
            _ => unimplemented!(),
        }
    }

    pub fn _eq(&self, other: &Node) -> bool {
        use Node::*;
        match (self, other) {
            (Int(a), Int(b)) => a == b,
            (Int(a), Bool(b)) => *a == *b as i32,
            (Bool(a), Int(b)) => *b == *a as i32,
            _ => unimplemented!(),
        }
    }

    pub fn _gt(&self, other: &Node) -> bool {
        use Node::*;
        match (self, other) {
            (Int(a), Int(b)) => a > b,
            (Int(a), Bool(b)) => *a > *b as i32,
            (Bool(a), Int(b)) => *b < *a as i32,
            _ => unimplemented!(),
        }
    }

    pub fn _lt(&self, other: &Node) -> bool {
        use Node::*;
        match (self, other) {
            (Int(a), Int(b)) => a < b,
            (Int(a), Bool(b)) => *a < *b as i32,
            (Bool(a), Int(b)) => *b > *a as i32,
            _ => unimplemented!(),
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        use Node::*;
        match self {
            Str(a) => a.to_string(),
            Int(a) => a.to_string(),
            Bool(a) => a.to_string(),
            _ => panic!("Attempting to print {:?}", self),
        }
    }
}
