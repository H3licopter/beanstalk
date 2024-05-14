use super::ast::AstNode;
use super::parse_expression::eval_expression;
use crate::parsers::parse_expression::create_expression;
use crate::{bs_types::DataType, Token};

pub fn new_tuple(
    tokens: &Vec<Token>,
    i: &mut usize,
    first_item: AstNode,
    ast: &Vec<AstNode>,
) -> AstNode {
    let first_item_eval = eval_expression(first_item, &DataType::Inferred, ast);
    let mut items: Vec<AstNode> = vec![first_item_eval];

    while let Some(token) = tokens.get(*i) {
        match token {
            Token::CloseParenthesis => {
                break;
            }

            Token::OpenParenthesis | Token::Comma => {
                *i += 1;
                items.push(create_expression(tokens, i, true, &ast));
            }

            _ => {
                items.push(create_expression(tokens, i, true, &ast));
            }
        }

        match token {
            Token::CloseParenthesis => {
                *i += 1;
                break;
            }

            _ => {
                *i += 1;
            }
        }
    }

    // TO DO: Get all expressions in the tuple

    return AstNode::Tuple(items);
}

pub fn new_array(tokens: &Vec<Token>, i: &mut usize, ast: &Vec<AstNode>) -> AstNode {
    let mut items: Vec<AstNode> = Vec::new();
    let collection_type = DataType::InferredCollection;

    // Should always start with current token being an open scope
    // So skip to first value
    *i += 1;

    while let Some(token) = tokens.get(*i) {
        match token {
            Token::CloseScope => {
                break;
            }

            // TO DO: Type checking and adding values to array
            _ => {
                items.push(create_expression(tokens, i, true, ast));
            }
        }

        *i += 1;
    }

    AstNode::Collection(items, collection_type)
}

// TO DO: new_struct
// TO DO: new_choice
