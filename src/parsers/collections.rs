use super::ast_nodes::{AstNode, Reference};
use super::parse_expression::evaluate_expression;
use crate::parsers::parse_expression::create_expression;
use crate::{bs_types::DataType, Token};

// DONT REALLY WORK YET
// Assumes already inside a tuple and there will be a final close parenthesis at the end
pub fn new_tuple(
    tokens: &Vec<Token>,
    i: &mut usize,
    first_item: AstNode,
    ast: &Vec<AstNode>,
    starting_line_number: &u32,
    variable_declarations: &Vec<Reference>,
) -> AstNode {
    let first_item_eval = evaluate_expression(first_item, &DataType::Inferred, ast);
    let mut items: Vec<AstNode> = vec![first_item_eval];
    let mut next_item: bool = true;

    while let Some(token) = tokens.get(*i) {
        match token {
            Token::CloseParenthesis => {
                *i += 1;
                break;
            }
            Token::Comma => {
                next_item = true;
                *i += 1;
            }

            _ => {
                if !next_item {
                    return AstNode::Error(
                        "Expected a comma between tuple items".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                next_item = false;
                items.push(create_expression(
                    tokens,
                    i,
                    true,
                    &ast,
                    starting_line_number,
                    &DataType::Inferred,
                    tokens[*i] == Token::OpenParenthesis,
                    variable_declarations,
                ));
            }
        }
    }

    return AstNode::Tuple(items, starting_line_number.to_owned());
}

pub fn new_collection(
    tokens: &Vec<Token>,
    i: &mut usize,
    ast: &Vec<AstNode>,
    starting_line_number: &u32,
    
) -> AstNode {
    let mut items: Vec<AstNode> = Vec::new();
    let collection_type = DataType::Collection(Box::new(DataType::Inferred));

    // Should always start with current token being an open scope
    // So skip to first value
    *i += 1;

    while let Some(token) = tokens.get(*i) {
        match token {
            Token::CloseCurly => {
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
                    tokens[*i] == Token::OpenParenthesis,
                    &Vec::new(),
                ));
            }
        }

        *i += 1;
    }

    AstNode::Collection(items, collection_type)
}

// TO DO: new_struct
// TO DO: new_choice
