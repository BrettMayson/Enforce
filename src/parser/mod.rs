use pest::Parser;

#[derive(Parser)]
#[grammar = "enforce.pest"]
pub struct EnforceParser;

mod node;
use node::Node;

pub fn parse(source: &str) -> Result<Node, String> {
  let pair = EnforceParser::parse(Rule::enforce, source).unwrap().next().unwrap();
  Ok(Node::from_expr(pair))
}
