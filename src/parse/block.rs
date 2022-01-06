use super::{
  error::{Error, Result},
  node::Node,
  parse_helper::ParseHelper,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
  pub body: Vec<Node>,
}

pub fn parse(ph: &mut ParseHelper) -> Result<Node> {
  println!("ph: {:?}", ph);
  unimplemented_f!("Not implemented")
}