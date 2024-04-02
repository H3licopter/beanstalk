use super::ast::AstNode;
use crate::{bs_types::DataType, Token};

// This function takes in an Expression node that has a Vec of Nodes to evaluate
// And returns the result of the expression as a single node of the correct type
pub fn eval_expression(expr: AstNode) -> AstNode {
    match expr {
        AstNode::Expression(e, data_type) => {
            match data_type {
                DataType::Int => {
                    let mut result = 0;

                    return AstNode::Literal(Token::IntLiteral(result));
                }

                DataType::String => {
                    let mut result = String::new();

                    return AstNode::Literal(Token::StringLiteral(result));
                }

                // Eval other types here ......
                _ => {
                    return AstNode::Error("Data Type for expression not supported".to_string());
                }
            }
        }
        _ => {
            return AstNode::Error("No Expression to Evaluate".to_string());
        }
    }
}
