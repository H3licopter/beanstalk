use crate::{bs_types::DataType, Token};
use super::ast_nodes::AstNode;

pub fn create_function(
    name: String,
    args: AstNode,
    tokens: &Vec<Token>,
    i: &mut usize,
    is_exported: bool,
    token_line_numbers: &Vec<u32>,
) -> AstNode {
    let function_body = Vec::new();
    let return_type =  match parse_return_type(tokens, i) {
        Ok(return_type) => return_type,
        Err(err) => {
            return AstNode::Error(err.to_string(), token_line_numbers[*i]);
        }
    };

    if &tokens[*i] != &Token::OpenScope {
        return AstNode::Error("Expected '{' to open function scope".to_string(),token_line_numbers[*i]);
    }

    *i += 1;

    // TODO - Get function body

    AstNode::Function(name, Box::new(args), function_body, is_exported, return_type)
}

fn parse_return_type(tokens: &Vec<Token>, i: &mut usize) -> Result<Vec<DataType>, &'static str> {
    let mut return_type = Vec::<DataType>::new();

    // Check if there is a return type
    let mut open_parenthesis = 0;
    let mut next_in_list: bool = true;
    while tokens[*i] != Token::OpenScope {
        match &tokens[*i] {
            Token::OpenParenthesis => {
                open_parenthesis += 1;
                *i += 1;
            }
            Token::CloseParenthesis => {
                open_parenthesis -= 1;
                *i += 1;
            }
            Token::TypeKeyword(type_keyword) => {
                if next_in_list {
                    return_type.push(type_keyword.to_owned());
                    next_in_list = false;
                } else {
                    return Err("Should have a comma to seperate return types");
                }
            }
            Token::Comma => {
                next_in_list = true;
                *i += 1;
            }
            _ => {
                return Err("Invalid syntax for return type");
            }
        }
    }

    if open_parenthesis != 0 {
        return Err("Wrong number of parenthesis used when declaring return type")
    }

    return Ok(return_type);
}