use crate::{ast::AstNode, Token};

pub fn new_ast(tokens: &Vec<Token>, start_index: usize) -> (Vec<AstNode>, usize) {
  let mut ast = Vec::new();
  let mut current_token_index = start_index;

  while current_token_index < tokens.len() {
    match &tokens[current_token_index] {
      
      Token::Comment(value) => {
        ast.push(AstNode::Comment(value.clone()));
        current_token_index += 1;
      }

      // New Function or Variable declaration
      Token::Variable(name) => {
        current_token_index += 1;
      }

      // Pure Expression or Lambda
      Token::OpenBracket => {
        current_token_index += 1;
      }

      Token::Return => {
        current_token_index += 1;
      }

      // End of Function or Variable declaration
      // Or stuff that hasn't been implemented yet
      _ => {
        current_token_index += 1;
      }
    }
  }

  (ast, current_token_index)
}