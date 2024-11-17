use colour::red_ln;

use super::{
    ast_nodes::{AstNode, Reference},
    expressions::parse_expression::create_expression,
};
use crate::{bs_types::DataType, parsers::ast_nodes::Node, Token};

// Assumes to have started after the the open parenthesis
// Datatype must always be a tuple containing the data types of the items in the tuple
// Or inferred if the data type is not known
// Also modifies the data type passed into it
// TO DO: Add named tuples
pub fn new_tuple(
    initial_value: Option<AstNode>,
    tokens: &Vec<Token>,
    i: &mut usize,
    data_type: &mut DataType,
    ast: &Vec<AstNode>,
    starting_line_number: &u32,
    variable_declarations: &Vec<Reference>,
) -> AstNode {
    let mut item_data_types = match data_type {
        DataType::Tuple(inner_types) => *inner_types.to_owned(),
        _ => Vec::new(),
    };
    let mut items: Vec<AstNode> = match initial_value {
        Some(node) => {
            item_data_types.push(node.get_type());
            vec![node]
        }
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
                let mut item_data_type = match item_data_types.get(items.len()) {
                    Some(datatype) => datatype.to_owned(),
                    None => {
                        DataType::Inferred
                    }
                };

                items.push(create_expression(
                    tokens,
                    i,
                    true,
                    &ast,
                    starting_line_number,
                    &mut item_data_type,
                    tokens[*i] == Token::OpenParenthesis,
                    variable_declarations,
                ));

                item_data_types.push(item_data_type);
            }
        }
    }

    if items.len() == 1 {
        return items[0].to_owned();
    }

    if items.len() < 1 {
        return AstNode::Empty;
    }

    *data_type = DataType::Tuple(Box::new(item_data_types));

    AstNode::Tuple(items, starting_line_number.to_owned())
}
