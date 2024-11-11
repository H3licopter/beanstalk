use super::{ast_nodes::AstNode, expressions::parse_expression::create_expression};
use crate::{bs_types::DataType, Token};

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
