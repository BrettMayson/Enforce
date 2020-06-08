use crate::parser::{AssignmentOperator, Node};

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
        Empty => {}
        _ => unimplemented!(),
    }
}
