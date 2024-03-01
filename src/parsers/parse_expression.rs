use crate::{ast::AstNode, Token};
use super::build_ast::is_reference;

// Returns the result of the expression at compile time
pub fn parse_expression(tokens: &Vec<Token>, i: &mut usize, bracket_nesting: i32, type_declaration: &Token) -> AstNode {
    let mut expression = AstNode::Literal(Token::IntLiteral(0));

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
            expression = AstNode::Literal(Token::StringLiteral(string.clone()));
        }

        _ => {
            return AstNode::Error("Invalid Assignment for Variable, must be assigned wih a valid datatype".to_string());
        }
    }

  expression
}

pub enum NumberType {
  Int(i32),
  Float(f64),
  Decimal(Vec<i8>),
}

pub fn parse_math_exp(tokens: &Vec<AstNode>, i: &mut usize, bracket_nesting: i32) -> NumberType {

  NumberType::Int(0)
}