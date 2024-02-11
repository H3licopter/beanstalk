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
                    let mut type_name = "function";
                    match &tokens[i] {
                        // Infer type (CONSTANT VARIABLE)
                        Token::Initialise => {

                        }
                        // Infer type (MUTABLE VARIABLE)
                        Token::Assign => {

                        }
                        // Type Declaration
                        Token::Type(token_type) => {
                            if &tokens[i + 1] == &Token::OpenBracket {
                                new_function(tokens, i);
                            } else {
                                type_name = token_type; 
                            }
                        }

                        Token::OpenBracket => {
                            new_function(tokens, i);
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

fn new_function(tokens: &Vec<Token>, start_index: usize) -> (AstNode, usize) {
    let mut i = start_index;
    let mut function_name = String::new();
    let mut function_args = Vec::new();

    // Get function name
    match &tokens[i] {
        Token::Variable(name) => {
            function_name = name.clone();
        }
        _ => {
            return (AstNode::Error("Expected function name".to_string()), i);
        }
    }

    // Get function args
    i += 1;
    if &tokens[i] != &Token::OpenBracket {
        return (AstNode::Error("Expected '(' for function args".to_string()), i);
    }

    i += 1;
    while &tokens[i] != &Token::CloseBracket {
        match &tokens[i] {
            Token::Variable(name) => {
                function_args.push(AstNode::VariableDeclaration(name.clone(), Box::new(AstNode::Ref("".to_string()))));
            }
            _ => {
                return (AstNode::Error("Expected variable name for function args".to_string()), i);
            }
        }
        i += 1;
    }

    (AstNode::Function(function_name, function_args), i)
}