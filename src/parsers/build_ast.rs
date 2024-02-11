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
                        ast.push(AstNode::Ref(name.clone()));
                } else {
                    i += 1;
                    if &tokens[i] != &Token::Initialise {
                        ast.push(AstNode::Error("Expected ':' for initialising".to_string()));
                    }

                    // check type or infer type
                    i += 1;
                    let mut type_name = "int";
                    match &tokens[i] {
                        // Infer type (CONSTANT)
                        Token::Initialise => {

                        }
                        // Infer type (MUTABLE)
                        Token::Assign => {

                        }
                        // Type Declaration
                        Token::Type(token_type) => {
                            if &tokens[i + 1] == &Token::OpenBracket {
                                type_name = "function";
                            } else {
                                type_name = token_type; 
                            }
                        }

                        Token::OpenBracket => {
                            type_name = "function";
                        }

                        _ => {
                            ast.push(AstNode::Error("Expected either type definition or another ':' or '=' for initialising".to_string()));
                        }
                    }

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