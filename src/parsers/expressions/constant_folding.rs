use crate::{bs_types::DataType, parsers::ast_nodes::AstNode, Token};

// This will evaluate everything possible at compile time
// returns either a literal or an evaluated runtime expression
pub fn math_constant_fold(output_stack: Vec<AstNode>, current_type: DataType) -> AstNode {
    let mut stack: Vec<AstNode> = Vec::new();

    for node in &output_stack {
        match node {
            AstNode::BinaryOperator(op, _) => {
                // Make sure there are at least 2 nodes on the stack
                if stack.len() < 2 {
                    return AstNode::Error("Not enough nodes on the stack for binary operator when parsing an expression".to_string(), 0);
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                // Check if top 2 of stack are literals
                // if at least one is not then this must be a runtime expression
                // And just push the operator onto the stack instead of evaluating
                // TO DO: GENERICS FOR THIS TO SUPPORT INTS CORRECTLY
                let left_value = match left {
                    AstNode::Literal(ref token) => match token {
                        Token::FloatLiteral(value) => *value,
                        Token::IntLiteral(value) => *value as f64,
                        _ => {
                            stack.push(left);
                            stack.push(right);
                            stack.push(node.to_owned());
                            continue;
                        }
                    },
                    _ => {
                        stack.push(left);
                        stack.push(right);
                        stack.push(node.to_owned());
                        continue;
                    }
                };

                let right_value = match right {
                    AstNode::Literal(ref token) => match token {
                        Token::FloatLiteral(value) => *value,
                        Token::IntLiteral(value) => *value as f64,
                        _ => {
                            stack.push(left);
                            stack.push(right);
                            stack.push(node.to_owned());
                            continue;
                        }
                    },
                    _ => {
                        stack.push(left);
                        stack.push(right);
                        stack.push(node.to_owned());
                        continue;
                    }
                };

                stack.push(AstNode::Literal(Token::FloatLiteral(match op {
                    Token::Add => left_value + right_value,
                    Token::Subtract => left_value - right_value,
                    Token::Multiply => left_value * right_value,
                    Token::Divide => left_value / right_value,
                    Token::Modulus => left_value % right_value,
                    _ => {
                        return AstNode::Error(format!("Unsupported operator: {:?}", op), 0);
                    }
                })));
            }
            // Some runtime thing
            _ => {
                stack.push(node.to_owned());
            }
        }
    }

    if stack.len() == 1 {
        return stack.pop().unwrap();
    }

    AstNode::RuntimeExpression(stack, current_type)
}

pub fn logical_constant_fold(output_stack: Vec<AstNode>, current_type: DataType) -> AstNode {
    let mut stack: Vec<AstNode> = Vec::new();

    for node in &output_stack {
        match node {
            AstNode::LogicalOperator(op, _) => {
                // Make sure there are at least 2 nodes on the stack
                if stack.len() < 2 {
                    return AstNode::Error("Not enough nodes on the stack for logical operator when parsing an expression".to_string(), 0);
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                // Check if top 2 of stack are literals
                // if at least one is not then this must be a runtime expression
                // And just push the operator onto the stack instead of evaluating
                let left_value = match left {
                    AstNode::Literal(Token::BoolLiteral(value)) => value,
                    _ => {
                        stack.push(left);
                        stack.push(right);
                        stack.push(node.to_owned());
                        continue;
                    }
                };

                let right_value = match right {
                    AstNode::Literal(Token::BoolLiteral(value)) => value,
                    _ => {
                        stack.push(left);
                        stack.push(right);
                        stack.push(node.to_owned());
                        continue;
                    }
                };

                stack.push(AstNode::Literal(Token::BoolLiteral(match op {
                    Token::Equal => left_value == right_value,
                    Token::And => left_value && right_value,
                    Token::Or => left_value || right_value,
                    _ => {
                        return AstNode::Error(format!("Unsupported operator: {:?}", op), 0);
                    }
                })));
            }
            // Some runtime thing
            _ => {
                stack.push(node.to_owned());
            }
        }
    }

    if stack.len() == 1 {
        return stack.pop().unwrap();
    }

    AstNode::RuntimeExpression(stack, current_type)
}
