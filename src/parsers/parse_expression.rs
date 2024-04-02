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

enum _Expression {
    Unary(_Operator, Token),         // Operator, Value
    Binary(_Operator, Token, Token), // Operator, LHS value, RHS value
}

// Returns the result of the expression for compile time evaluation
pub fn parse_expression(
    tokens: &Vec<Token>,
    i: &mut usize,
    type_declaration: &DataType,
) -> AstNode {
    let mut expression = Vec::new();

    // Check if value is wrapped in brackets and move on until first value is found
    let mut bracket_nesting: i32 = 0;
    while &tokens[*i] == &Token::OpenBracket {
        bracket_nesting += 1;
        *i += 1;
    }

    while let Some(token) = tokens.get(*i) {
        match token {
            // Conditions that end the expression
            Token::Newline => {
                if bracket_nesting == 0 {
                    break;
                }
            }
            Token::EOF => {
                break;
            }
            Token::CloseBracket => {
                if bracket_nesting > 1 {
                    bracket_nesting -= 1;
                }
                if bracket_nesting == 1 {
                    break;
                }
                if bracket_nesting < 1 {
                    expression.push(AstNode::Error("Too many closing brackets".to_string()));
                }
            }

            // Check if name is a reference to another variable or function call
            Token::Variable(name) => {
                if is_reference(tokens, i, name) {
                    // Check if is function call
                    if &tokens[*i + 1] == &Token::OpenBracket {
                        // Read function args
                        let mut args = Vec::new();
                        *i += 2;
                        while &tokens[*i] != &Token::CloseBracket {
                            // TO DO, CHECK IS VALID ARGUMENT
                            let arg = parse_expression(tokens, i, &type_declaration);
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
            Token::Add => {
                expression.push(AstNode::Add);
            }
            Token::Subtract => {
                expression.push(AstNode::Subtract);
            }
            Token::Multiply => {
                expression.push(AstNode::Multiply);
            }
            Token::Divide => {
                expression.push(AstNode::Divide);
            }
            Token::Modulus => {
                expression.push(AstNode::Modulus);
            }
            Token::Exponent => {
                expression.push(AstNode::Exponent);
            }

            _ => {
                expression.push(AstNode::Error(
                    "Invalid Expression, must be assigned wih a valid datatype".to_string(),
                ));
            }
        }

        *i += 1;
    }

    AstNode::Expression(expression, type_declaration.clone())
}
