use super::build_ast::is_reference;
use crate::{ast::AstNode, Token};

enum Operator {
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

enum Expression {
    Unary(Operator, Token),         // Operator, Value
    Binary(Operator, Token, Token), // Operator, LHS value, RHS value
}

// Returns the result of the expression for compile time evaluation
pub fn parse_expression(
    tokens: &Vec<Token>,
    i: &mut usize,
    bracket_nesting: i32,
    type_declaration: &Token,
) -> AstNode {
    let mut expr = AstNode::Literal(Token::IntLiteral(0));

    match &tokens[*i] {
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
                        let arg = parse_expression(tokens, i, bracket_nesting, type_declaration);
                        args.push(arg);

                        *i += 1;
                    }

                    return AstNode::FunctionCall(name.clone(), args);
                }

                return AstNode::Ref(name.clone());
            }

            return AstNode::Error("Variable reference not defined. Maybe you're using a variable that has not yet been declared?".to_string());
        }

        // Check if is a literal
        Token::StringLiteral(string) => {
            expr = AstNode::Literal(Token::StringLiteral(string.clone()));
        }

        _ => {
            return AstNode::Error(
                "Invalid Assignment for Variable, must be assigned wih a valid datatype"
                    .to_string(),
            );
        }
    }

    expr
}

pub enum NumberType {
    Int(i32),
    Float(f64),
    Decimal(Vec<char>),
}

pub fn parse_math_exp(tokens: &Vec<AstNode>, i: &mut usize, bracket_nesting: i32) -> NumberType {
    NumberType::Int(0)
}
