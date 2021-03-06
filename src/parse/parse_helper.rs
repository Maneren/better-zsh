use crate::types::{Token, TokenType};

#[derive(Debug)]
pub struct ParseHelper {
  tokens: Vec<Token>,
  index: usize,
}

impl ParseHelper {
  pub fn new(tokens: Vec<Token>) -> ParseHelper {
    ParseHelper { tokens, index: 0 }
  }

  pub fn get(&self, offset: i32) -> Option<&Token> {
    let i = self.index as i32 + offset;

    if i < 0 {
      return None;
    }

    self.tokens.get(i as usize)
  }

  pub fn peek(&self, offset: i32) -> Option<&TokenType> {
    let i = self.index as i32 + offset;

    if i < 0 {
      return None;
    }

    self.tokens.get(i as usize).map(|token| &token.r#type)
  }

  pub fn get_tokens(&self) -> &Vec<Token> {
    &self.tokens
  }

  #[cfg(debug_assertions)]
  pub fn get_index(&self) -> usize {
    self.index
  }

  #[cfg(debug_assertions)]
  pub fn pretty_print_tokens(&self) -> String {
    self
      .tokens
      .iter()
      .enumerate()
      .map(|(i, token)| format!("{i}: {token}"))
      .collect::<Vec<_>>()
      .join(",\n")
  }

  pub fn advance(&mut self) {
    self.index += 1;
  }
}
