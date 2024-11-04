use super::{
    ast_nodes::{AstNode, Reference}, create_scene_node::new_scene, parse_expression::create_expression,
    variables::create_new_var_or_ref,
};
use crate::{bs_types::DataType, Token};
use colour::red_ln;
use std::path::PathBuf;

pub fn new_ast(
    tokens: Vec<Token>,
    start_index: usize,
    token_line_numbers: &Vec<u32>,
    mut variable_declarations: Vec<Reference>,
    return_type: &DataType,
) -> (Vec<AstNode>, Vec<AstNode>) {
    let mut ast = Vec::new();
    let mut imports = Vec::new();
    let mut i = start_index;
    let mut exported: bool = false;
    let mut needs_to_return = return_type != &DataType::None;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Comment(value) => {
                ast.push(AstNode::Comment(value.clone()));
            }
            Token::Import => {
                i += 1;
                match &tokens[i] {
                    // Module path that will have all it's exports dumped into the module
                    Token::StringLiteral(value) => {
                        imports.push(AstNode::Use(PathBuf::from(value.clone())));
                    }
                    _ => {
                        ast.push(AstNode::Error(
                            "Import must have a valid path as a argument".to_string(),
                            token_line_numbers[i],
                        ));
                    }
                }
            }
            Token::SceneHead | Token::ParentScene => {
                let starting_line_number = &token_line_numbers[i];
                ast.push(new_scene(&tokens, &mut i, &ast, starting_line_number, &variable_declarations));
            }
            Token::ModuleStart(_) => {
                // In future, need to structure into code blocks
            }

            // New Function or Variable declaration
            Token::Variable(name) => {
                ast.push(create_new_var_or_ref(
                    name, 
                    &mut variable_declarations,
                    &tokens,
                    &mut i,
                    exported,
                    &ast,
                    token_line_numbers,
                ));
            }
            Token::Export => {
                exported = true;
            }

            Token::JS(value) => {
                ast.push(AstNode::JS(value.clone()));
            }
            Token::Title => {
                i += 1;
                match &tokens[i] {
                    Token::StringLiteral(value) => {
                        ast.push(AstNode::Title(value.clone()));
                    }
                    _ => {
                        ast.push(AstNode::Error(
                            "Title must have a valid string as a argument".to_string(),
                            token_line_numbers[i],
                        ));
                    }
                }
            }

            Token::Date => {
                i += 1;
                match &tokens[i] {
                    Token::StringLiteral(value) => {
                        ast.push(AstNode::Date(value.clone()));
                    }
                    _ => {
                        ast.push(AstNode::Error(
                            "Date must have a valid string as a argument".to_string(),
                            token_line_numbers[i],
                        ));
                    }
                }
            }

            Token::Newline | Token::Empty | Token::SceneClose(_) => {
                // Do nothing for now
            }

            Token::Print => {
                i += 1;
                let inside_brackets = &tokens[i] == &Token::OpenParenthesis;
                let starting_line_number = &token_line_numbers[i];
                ast.push(AstNode::Print(Box::new(create_expression(
                    &tokens,
                    &mut i,
                    false,
                    &ast,
                    starting_line_number,
                    &DataType::CoerseToString,
                    inside_brackets,
                    &variable_declarations,
                ))));
            }

            Token::DeadVarible(name) => {
                // Remove entire declaration or scope of variable declaration
                // So don't put any dead code into the AST
                skip_dead_code(&tokens, &mut i);
                ast.push(AstNode::Error(
                    format!(
                        "Dead Variable Declaration. Variable is never used or declared: {}",
                        name
                    ),
                    token_line_numbers[i - 1],
                ));
            }

            Token::Return => {
                if !needs_to_return {
                    ast.push(AstNode::Error(
                        "Return statement used in function that doesn't return a value".to_string(),
                        token_line_numbers[i],
                    ));
                }

                needs_to_return = false;
                i += 1;

                let starting_line_number = &token_line_numbers[i];

                let return_value = create_expression(
                    &tokens,
                    &mut i,
                    false,
                    &ast,
                    starting_line_number,
                    return_type,
                    false,
                    &variable_declarations,
                );

                ast.push(AstNode::Return(Box::new(return_value)));
            }

            // TOKEN END SHOULD NEVER BE AT TOP LEVEL
            // This is to break out of blocks only
            // There should be a way to handle this to throw a syntax error if 'end' is used at the top level
            Token::EOF | Token::End => {
                break;
            }

            // Or stuff that hasn't been implemented yet
            _ => {
                ast.push(AstNode::Error(
                    format!("Compiler Error: Token not recognised by AST parser when creating AST: {:?}", &tokens[i]).to_string(),
                    token_line_numbers[i - 1],
                ));
            }
        }

        i += 1;
    }

    if needs_to_return {
        ast.push(AstNode::Error(
            "Function does not return a value".to_string(),
            token_line_numbers[i - 1],
        ));
    }

    (ast, imports)
}

fn skip_dead_code(tokens: &Vec<Token>, i: &mut usize) {
    // Check what type of dead code it is
    // If it is a variable declaration, skip to the end of the declaration

    *i += 1;
    match tokens.get(*i).unwrap_or(&Token::EOF) {
        Token::Assign | Token::Colon => {
            *i += 1;
        }
        Token::Newline => {
            *i += 1;
            return;
        }
        _ => {
            return;
        }
    }

    // Skip to end of variable declaration
    let mut open_parenthesis = 0;
    while let Some(token) = tokens.get(*i) {
        match token {
            Token::OpenParenthesis => {
                *i += 1;
                open_parenthesis += 1;
            }
            Token::CloseParenthesis => {
                *i += 1;

                if open_parenthesis < 1 {
                    red_ln!("Error: Closing parenthesis without opening parenthesis in dead variable code");
                    return;
                }
                open_parenthesis -= 1;
                if open_parenthesis == 0 {
                    break;
                }
            }
            Token::Newline => {
                *i += 1;
                if open_parenthesis < 1 {
                    return;
                }
            }
            Token::EOF => {
                break;
            }
            _ => {
                *i += 1;
            }
        }
    }
}

// pub fn get_var_declaration_type(var_name: String, ast: &Vec<AstNode>) -> DataType {
//     for node in ast {
//         match node {
//             AstNode::VarDeclaration(name, _, _, data_type, _) => {
//                 if *name == var_name {
//                     return data_type.to_owned();
//                 }
//             }
//             _ => {}
//         }
//     }

//     DataType::Inferred
// }
