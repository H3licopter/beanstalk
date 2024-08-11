use colour::red_ln;

use super::{
    ast_nodes::AstNode, create_scene_node::new_scene, parse_expression::create_expression, variables::{find_var_declaration_index, new_variable}
};
use crate::{bs_types::DataType, Token};


pub fn new_ast(tokens: Vec<Token>, start_index: usize, token_line_numbers: &Vec<u32>) -> (Vec<AstNode>, usize) {
    let mut ast = Vec::new();
    let mut i = start_index;
    let mut is_exported = false;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Comment(value) => {
                ast.push(AstNode::Comment(value.clone()));
            }

            Token::SceneHead(scene_head) => {
                ast.push(new_scene(scene_head, &tokens, &mut i, &ast, token_line_numbers));
            }
            Token::ModuleStart(_) => {
                // In future, need to structure into code blocks
            }

            // New Function or Variable declaration
            Token::VarDeclaration(id) => {
                // Need to determine if it is a const that compiles to a literal, should just push a literal in that case
                ast.push(new_variable(
                    *id,
                    &tokens,
                    &mut i,
                    is_exported,
                    &ast,
                    token_line_numbers,
                ));
                is_exported = false;
            }

            Token::Export => {
                is_exported = true;
            }

            Token::VarReference(id) => {
                ast.push(AstNode::VarReference(find_var_declaration_index(&ast, id)));
            }
            Token::ConstReference(id) => {
                ast.push(AstNode::ConstReference(find_var_declaration_index(
                    &ast, id,
                )));
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

            Token::Newline | Token::Empty | Token::SceneClose(_) | Token::Whitespace => {
                // Do nothing for now
            }

            Token::Print => {
                i += 1;
                ast.push(AstNode::Print(Box::new(
                    create_expression(&tokens, &mut i, false, &ast, token_line_numbers, &DataType::Inferred),
                )));
            }

            Token::DeadVarible => {
                // Remove entire declaration or scope of variable declaration
                // So don't put any dead code into the AST
                skip_dead_code(&tokens, &mut i);
            }

            Token::EOF => {
                break;
            }

            // Or stuff that hasn't been implemented yet
            _ => {
                ast.push(AstNode::Error(
                    format!("Compiler Error: Token not recognised by AST parser when creating AST: {:?}", &tokens[i]).to_string(),
                    token_line_numbers[i],
                ));
            }
        }

        i += 1;
    }

    (ast, i)
}

fn skip_dead_code(tokens: &Vec<Token>, i: &mut usize) {
    // Check what type of dead code it is
    // If it is a variable declaration, skip to the end of the declaration

    *i += 1;
    match tokens.get(*i).unwrap_or(&Token::EOF) {
        Token::Assign | Token::AssignConstant | Token::Colon => {
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

