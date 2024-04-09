use super::{ast::AstNode, build_ast::is_reference};
use crate::{bs_types::DataType, Token};

enum _Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Root,
    Modulus,
    Exponent,
    And,
    Or,
    Not,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,
}

// Creates an expression node from a list of tokens
pub fn create_expression(
    tokens: &Vec<Token>,
    i: &mut usize,
) -> AstNode {
    let mut expression = Vec::new();

    // Check if value is wrapped in brackets and move on until first value is found
    let mut bracket_nesting: i32 = 0;
    while &tokens[*i] == &Token::OpenParenthesis {
        bracket_nesting += 1;
        *i += 1;
    }

    // Find the end of the expression and check if it is assigned a data type at the end
    let mut expression_end = *i;
    if bracket_nesting > 0 {
        // Find the last closing bracket and end expression there
        let mut total_open_brackets = bracket_nesting;
        while total_open_brackets > 0 && &expression_end < &tokens.len()  {
            expression_end += 1;
            if &tokens[expression_end] == &Token::OpenParenthesis {
                total_open_brackets += 1;
            } else if &tokens[expression_end] == &Token::CloseParenthesis {
                total_open_brackets -= 1;
            }
        }
    } else {
        // Find the next newline and end expression there
        while &expression_end < &tokens.len() {
            match &tokens[expression_end] {
                Token::Newline | Token::Comma | Token::SceneClose(_) => {
                    break;
                }
                _ => {
                    expression_end += 1;
                }
            }
        }
    }

    // Get the data type of the expression if there is one after the expression
    let mut data_type = &DataType::Inferred;
    if expression_end < tokens.len() {
        match &tokens[expression_end + 1] {
            Token::TypeKeyword(type_keyword) => {
                data_type = &type_keyword
            }
            _ => {}
        };
    }




    // Loop through the expression and create the AST nodes
    // Figure out the type from the data
    // If the type does not match the assigned datatype then throw an error
    while let Some(token) = tokens.get(*i) {
        match token {
            // Conditions that close the expression
            Token::Newline => {
                if bracket_nesting == 0 {
                    break;
                }
            }
            Token::EOF | Token::Comma | Token::CloseCollection => {
                if bracket_nesting == 0 {
                    break;
                }
                return AstNode::Error("Not enough closing parenthesis for expression. Need more ')'!".to_string());
            }
            Token::CloseParenthesis => {
                if bracket_nesting > 1 {
                    bracket_nesting -= 1;
                } else {
                    break;
                }
            }

            // Check if name is a reference to another variable or function call
            Token::Variable(name) => {
                if is_reference(tokens, i, name) {
                    // Check if is function call
                    if &tokens[*i + 1] == &Token::OpenParenthesis {
                        // Read function args
                        let mut args = Vec::new();
                        *i += 2;
                        while &tokens[*i] != &Token::CloseParenthesis {
                            // TO DO, CHECK IS VALID ARGUMENT
                            let arg = create_expression(tokens, i);
                            args.push(arg);

                            *i += 1;
                        }

                        expression.push(AstNode::FunctionCall(name.clone(), args));
                    }

                    expression.push(AstNode::Ref(name.clone()));
                }

                expression.push(AstNode::Error("Variable reference not defined. Maybe you're using a variable that has not yet been declared?".to_string()));
            }

            // Check if is a literal
            Token::IntLiteral(int) => {
                expression.push(AstNode::Literal(Token::IntLiteral(*int)));
            }
            Token::StringLiteral(string) => {
                expression.push(AstNode::Literal(Token::StringLiteral(string.clone())));
            }
            Token::FloatLiteral(float) => {
                expression.push(AstNode::Literal(Token::FloatLiteral(*float)));
            }

            // Check if operator
            Token::Negative => {
                expression.push(AstNode::UnaryOperator(Token::Negative));
            }
            Token::Add => {
                expression.push(AstNode::BinaryOperator(Token::Add));
            }
            Token::Subtract => {
                expression.push(AstNode::BinaryOperator(Token::Subtract));
            }
            Token::Multiply => {
                expression.push(AstNode::BinaryOperator(Token::Multiply));
            }
            Token::Divide => {
                expression.push(AstNode::BinaryOperator(Token::Divide));
            }
            Token::Modulus => {
                expression.push(AstNode::BinaryOperator(Token::Modulus));
            }
            Token::Exponent => {
                expression.push(AstNode::BinaryOperator(Token::Exponent));
            }

            _ => {
                expression.push(AstNode::Error(
                    "Invalid Expression, must be assigned wih a valid datatype".to_string(),
                ));
            }
        }

        *i += 1;
    }

    AstNode::Expression(expression, data_type.clone())
}

// This function takes in an Expression node that has a Vec of Nodes to evaluate
// And evaluates everything possible at compile time
// If it returns a literal, then everything was evaluated at compile time
// Otherwise it will return a simplified expression for runtime evaluation
pub fn eval_expression(expr: AstNode) -> AstNode {
    match expr {
        AstNode::Expression(e, data_type) => {
            match data_type {
                DataType::Float => {
                    let mut result = 0.0;

                    for token in e {
                        match token {
                            AstNode::Literal(Token::FloatLiteral(float)) => {
                                result = float;
                            }
                            AstNode::Literal(Token::IntLiteral(int)) => {
                                result = int as f64;
                            }
                            _ => {
                                return AstNode::Error(
                                    "(Eval Expression) Unknown Operator used in Expression"
                                        .to_string(),
                                );
                            }
                        }
                    }

                    return AstNode::Literal(Token::FloatLiteral(result));
                }

                // UNIMPLIMENTED DATA TYPES
                DataType::Int => {
                    let mut result = 0;

                    for token in e {
                        match token {
                            AstNode::Literal(Token::FloatLiteral(float)) => {
                                result = float as i64;
                            }
                            AstNode::Literal(Token::IntLiteral(int)) => {
                                result = int;
                            }

                            _ => {
                                return AstNode::Error(
                                    "Unknown Operator used in Expression".to_string(),
                                );
                            }
                        }
                    }

                    return AstNode::Literal(Token::IntLiteral(result));
                }
                DataType::String => {
                    let mut _result = String::new();

                    return AstNode::Literal(Token::StringLiteral(_result));
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
