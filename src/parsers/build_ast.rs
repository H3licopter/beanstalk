use crate::{ast::AstNode, Token};

pub fn new_ast(tokens: &Vec<Token>, start_index: usize) -> (Vec<AstNode>, usize) {
    let mut ast = Vec::new();
    let mut  i = start_index;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Comment(value) => {
                ast.push(AstNode::Comment(value.clone()));
            }

            // New Function or Variable declaration
            Token::Variable(name) => {

                // If already initialised, return a reference ast node
                if tokens
                    .iter()
                    .rev()
                    .skip(tokens.len() - i - 1)
                    .any(|t| t == &Token::Variable(name.clone())) {
                        ast.push(AstNode::Ref());
                    }
                
                
                i += 1;
                match tokens[i] {
                    // Parse Type definition
                    Token::Initialise => {

                    }

                    _ => ()
                }
            }

            // Or stuff that hasn't been implemented yet
            _ => {
            
            }
        }

        i += 1;
        
    }

    (ast, i)
}