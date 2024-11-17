use colour::red_ln;

use super::{
    ast_nodes::{AstNode, Reference},
    expressions::parse_expression::create_expression,
};
use crate::{bs_types::DataType, Token};

// Assumes to have started after the the open parenthesis
// Datatype must always be a tuple containing the data types of the items in the tuple
// TO DO: Add named tuples
pub fn new_tuple(
    initial_value: Option<AstNode>,
    tokens: &Vec<Token>,
    i: &mut usize,
    data_type: &DataType,
    ast: &Vec<AstNode>,
    starting_line_number: &u32,
    variable_declarations: &Vec<Reference>,
) -> AstNode {
    let mut items: Vec<AstNode> = match initial_value {
        Some(node) => vec![node],
        None => Vec::new(),
    };

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
                    red_ln!("Expected a comma between tuple items");
                    return AstNode::Error(
                        "Expected a comma between tuple items".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                next_item = false;

                // Get the datatype of this tuple item
                let item_data_type = match data_type {
                    DataType::Inferred => &DataType::Inferred,
                    DataType::Tuple(inner_types) => match inner_types.get(items.len()) {
                        Some(data_type) => data_type,
                        None => {
                            return AstNode::Error(
                                "Too many items in tuple".to_string(),
                                starting_line_number.to_owned(),
                            );
                        }
                    },
                    _ => {
                        return AstNode::Error(
                            "Invalid datatype for tuple".to_string(),
                            starting_line_number.to_owned(),
                        );
                    }
                };

                items.push(create_expression(
                    tokens,
                    i,
                    true,
                    &ast,
                    starting_line_number,
                    item_data_type,
                    tokens[*i] == Token::OpenParenthesis,
                    variable_declarations,
                ));
            }
        }
    }

    if items.len() == 1 {
        return items[0].to_owned();
    }

    if items.len() < 1 {
        return AstNode::Empty;
    }

    AstNode::Tuple(items, starting_line_number.to_owned())
}
