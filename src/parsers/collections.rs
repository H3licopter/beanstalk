use crate::parsers::parse_expression::create_expression;
use crate::{bs_types::DataType, Token};
use super::ast::AstNode;

pub fn new_tuple(tokens: &Vec<Token>, i: &mut usize, first_item: AstNode) -> AstNode {
    let mut items: Vec<AstNode> = vec![first_item];

    while let Some(token) = tokens.get(*i) {
        match token {

            Token::EOF | Token::Newline | Token::SceneClose(_) | Token::CloseParenthesis => {
                break;
            }

            Token::OpenParenthesis => {
                items.push(create_expression(tokens, i, true));
            }

            Token::Comma => {
                *i += 1;
                items.push(create_expression(tokens, i, true));
            }

            _ => {
                items.push(create_expression(tokens, i, true));
            }
        }

        *i += 1;
    }

    // TO DO: Get all expressions in the tuple

    return AstNode::Tuple(items);
}


pub fn new_array(tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut items: Vec<AstNode> = Vec::new();
    let mut collection_type = DataType::InferredCollection;

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
                items.push(create_expression(tokens, i, true));
            }
        }

        *i += 1;
    }

    AstNode::Collection(items, collection_type)
}