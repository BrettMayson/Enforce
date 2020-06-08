use crate::parser::{AssignmentOperator, ComparisonOperator, LogicalOperator, Node};

mod heap;
use heap::Heap;

pub fn execute(ast: Vec<Node>) {
    let mut heap = Heap::new();
    for node in ast {
        doit(node, &mut heap);
    }
}

pub fn doit(node: Node, heap: &mut Heap) {
    match node {
        Node::Assignment {
            etype,
            ident,
            operator,
            value,
        } => {
            heap.set_variable(
                &ident,
                match operator {
                    AssignmentOperator::Equal => value,
                    AssignmentOperator::AddEqual => {
                        if let Some(old) = heap.get_variable(&ident) {
                            Box::new((old).add(&*value))
                        } else {
                            panic!("Attempting to += on undefined variable");
                        }
                    }
                },
                etype.is_some(),
            );
        }
        Node::Call { ident, args } => {
            if let Some(function) = heap.get_variable(&ident) {
                unimplemented!("Unable to call custom function {:?}", function);
            } else {
                match ident.as_str() {
                    "Print" => {
                        if args.len() != 1 {
                            panic!("Print expects 1 argument, got {}", args.len())
                        }
                        let output = if let Node::Ident(key) = &args[0] {
                            if let Some(o) = heap.get_variable(&key) {
                                o.to_string()
                            } else {
                                panic!("Undefined variable {:?}", &key);
                            }
                        } else {
                            args[0].to_string()
                        };
                        println!("{}", output);
                    }
                    _ => panic!("Undefined function {:?}", ident),
                }
            }
        }
        Node::eIf(comparison, statements) => {
            if let Node::LogicalExpression(checks) = *comparison {
                let mut current = true;
                for check in checks {
                    current = evaluate_comparison(current, check.0, *check.1, heap);
                }
                if current {
                    for stmt in statements {
                        doit(stmt, heap);
                    }
                }
            } else {
                panic!("what huh")
            }
        }
        Empty => {}
        _ => unimplemented!(),
    }
}

fn evaluate_comparison(current: bool, op: LogicalOperator, node: Node, heap: &mut Heap) -> bool {
    use Node::*;
    match (current, op) {
        (true, LogicalOperator::AND) | (false, LogicalOperator::OR) => {
            if let LogicVal { inverted, expr } = node {
                if let ComparisonExpression { lhs, rhs } = *expr {
                    let a = match *lhs {
                        Call { .. } => unimplemented!("unable to do calls"),
                        Ident(b) => {
                            if let Some(n) = heap.get_variable(&b) {
                                n
                            } else {
                                panic!("Undefined variable {:?}", b)
                            }
                        }
                        _ => &*lhs,
                    };
                    match rhs {
                        Some(r) => {
                            let b = match *r.1 {
                                Ident(b) => {
                                    if let Some(n) = heap.get_variable(&b) {
                                        n
                                    } else {
                                        panic!("Undefined variable {:?}", b)
                                    }
                                }
                                _ => &*r.1,
                            };
                            match r.0 {
                                ComparisonOperator::EqualEqual => a._eq(&b),
                                ComparisonOperator::GreaterThan => a._gt(&b),
                                ComparisonOperator::LessThan => a._lt(&b),
                                _ => unimplemented!(),
                            }
                        }
                        None => a._eq(&Node::Bool(true)),
                    }
                } else {
                    panic!("what")
                }
            } else {
                panic!("what")
            }
        }
        _ => current,
    }
}
