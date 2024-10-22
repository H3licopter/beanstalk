use super::{
    ast_nodes::AstNode, create_scene_node::new_scene, parse_expression::create_expression,
    variables::new_variable,
};
use crate::{bs_types::DataType, Token};
use colour::red_ln;
use std::path::PathBuf;

pub fn new_ast(
    tokens: Vec<Token>,
    start_index: usize,
    token_line_numbers: &Vec<u32>,
) -> (Vec<AstNode>, Vec<AstNode>) {
    let mut ast = Vec::new();
    let mut imports = Vec::new();
    let mut i = start_index;

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
                ast.push(new_scene(&tokens, &mut i, &ast, starting_line_number));
            }
            Token::ModuleStart(_) => {
                // In future, need to structure into code blocks
            }

            // New Function or Variable declaration
            Token::VarDeclaration(id, is_exported) => {
                // Need to determine if it is a const that compiles to a literal, should just push a literal in that case
                ast.push(new_variable(
                    id,
                    &tokens,
                    &mut i,
                    *is_exported,
                    &ast,
                    token_line_numbers,
                ));
            }
            Token::Export => {}
            Token::VarReference(id) => {
                ast.push(AstNode::VarReference(
                    id.to_string(),
                    get_var_declaration_type(id.to_string(), &ast),
                ));
            }
            Token::ConstReference(id) => {
                ast.push(AstNode::ConstReference(
                    id.to_string(),
                    get_var_declaration_type(id.to_string(), &ast),
                ));
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
                let starting_line_number = &token_line_numbers[i];
                ast.push(AstNode::Print(Box::new(create_expression(
                    &tokens,
                    &mut i,
                    false,
                    &ast,
                    starting_line_number,
                    &DataType::Inferred,
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

            Token::EOF => {
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

    (ast, imports)
}

fn skip_dead_code(tokens: &Vec<Token>, i: &mut usize) {
    // Check what type of dead code it is
    // If it is a variable declaration, skip to the end of the declaration

    *i += 1;
    match tokens.get(*i).unwrap_or(&Token::EOF) {
        Token::Assign | Token::InitialiseInfer(_) | Token::Colon => {
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

pub fn get_var_declaration_type(var_name: String, ast: &Vec<AstNode>) -> DataType {
    for node in ast {
        match node {
            AstNode::VarDeclaration(name, _, _, data_type, _) => {
                if *name == var_name {
                    return data_type.to_owned();
                }
            }
            _ => {}
        }
    }

    DataType::Inferred
}
