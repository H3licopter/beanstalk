use super::ast_nodes::AstNode;
use super::parse_expression::evaluate_expression;
use crate::parsers::parse_expression::create_expression;
use crate::{bs_types::DataType, Token};

pub fn new_tuple(
    tokens: &Vec<Token>,
    i: &mut usize,
    first_item: AstNode,
    ast: &Vec<AstNode>,
    starting_line_number: &u32,
) -> AstNode {
    let first_item_eval = evaluate_expression(first_item, &DataType::Inferred, ast);
    let mut items: Vec<AstNode> = vec![first_item_eval];

    while let Some(token) = tokens.get(*i) {
        match token {
            Token::CloseParenthesis => {
                *i += 1;
                break;
            }

            _ => {
                *i += 1;
                items.push(create_expression(
                    tokens,
                    i,
                    true,
                    &ast,
                    starting_line_number,
                    &DataType::Inferred,
                ));
            }
        }
    }

    return AstNode::Tuple(items, starting_line_number.to_owned());
}

pub fn new_array(
    tokens: &Vec<Token>,
    i: &mut usize,
    ast: &Vec<AstNode>,
    starting_line_number: &u32,
) -> AstNode {
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
                items.push(create_expression(
                    tokens,
                    i,
                    true,
                    ast,
                    starting_line_number,
                    &collection_type,
                ));
            }
        }

        *i += 1;
    }

    AstNode::Collection(items, collection_type)
}

// TO DO: new_struct
// TO DO: new_choice
