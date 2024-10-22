use crate::{bs_types::DataType, Token};

use super::{
    ast_nodes::AstNode, collections::new_collection, functions::create_function,
    parse_expression::create_expression,
};

#[derive(PartialEq, Debug)]
enum Attribute {
    Constant,
    Mutable,
    TypeDeclaration,
}

// CAN RETURN:
// VarDeclaration, Const, Error, Function, Tuple
pub fn new_variable(
    name: &String,
    tokens: &Vec<Token>,
    i: &mut usize,
    is_exported: bool,
    ast: &Vec<AstNode>,
    token_line_numbers: &Vec<u32>,
) -> AstNode {
    let mut attribute;

    *i += 1;
    match &tokens[*i] {
        &Token::InitialiseInfer(is_const) => {
            attribute = if is_const {
                Attribute::Constant
            } else {
                Attribute::Mutable
            };
        }
        &Token::Colon => {
            attribute = Attribute::TypeDeclaration;
        }

        // TO DO: Multiple assignments
        // &Token::Comma => {
        // }

        // Anything else is a syntax error
        _ => {
            return AstNode::Error(
                "Syntax Error: Expected ':' or '=' after variable name for initialising. Variable does not yet exsist".to_string(),
                token_line_numbers[*i],
            );
        }
    }

    // Get assigned values
    // Can also be function args
    *i += 1;
    let mut data_type = &DataType::Inferred;
    let parsed_expr;

    // Check if function
    match &tokens[*i] {
        // Need to check if there is an arrow after parenthesis close, if so it's a function
        // Otherwise this is just an expression or tuple wrapped in parenthesis
        Token::OpenParenthesis => {
            // Maybe something will use colon rather than constant later
            let start_line_number = &token_line_numbers[*i];
            parsed_expr = create_expression(tokens, i, false, &ast, start_line_number, data_type);

            // FUNCTION? must have arrow after parenthesis close
            if attribute == Attribute::Constant {
                match tokens[*i] {
                    Token::Arrow => {
                        *i += 1;
                        return create_function(
                            name.to_string(),
                            parsed_expr,
                            tokens,
                            i,
                            is_exported,
                            token_line_numbers,
                        );
                    }
                    // Not a function
                    _ => {}
                }
            }
        }

        Token::TypeKeyword(type_keyword) => {
            data_type = type_keyword;
            let next_token = if *i + 1 >= tokens.len() {
                return AstNode::VarDeclaration(
                    name.to_string(),
                    Box::new(AstNode::Empty),
                    is_exported,
                    data_type.to_owned(),
                    false,
                );
            } else {
                &tokens[*i + 1]
            };

            // Check after the type declaration
            match next_token {
                Token::Newline | Token::EOF => {
                    return create_zero_value_var(
                        data_type.to_owned(),
                        name.to_string(),
                        is_exported,
                    );
                }
                // Is a constant with a type declaration
                // var_name : type : expression
                Token::Colon => {
                    attribute = Attribute::Constant;
                    *i += 1;
                }
                // Is a mutable with a type declaration
                // var_name : type = expression
                Token::Assign => {
                    attribute = Attribute::Mutable;
                    *i += 1;
                }
                _ => {
                    return AstNode::Error(
                        format!("Unexpected token after type declaration: {:?}", next_token),
                        token_line_numbers[*i],
                    );
                }
            }
            let start_line_number = &token_line_numbers[*i];
            parsed_expr = create_expression(tokens, i, false, &ast, start_line_number, data_type);
        }

        // COLLECTIONS
        Token::OpenCurly => match attribute {
            // New struct declaration
            // var_name : {}
            Attribute::TypeDeclaration => {
                let start_line_number = &token_line_numbers[*i];
                return AstNode::Struct(
                    name.to_string(),
                    Box::new(new_collection(tokens, i, ast, start_line_number)),
                    is_exported,
                );
            }

            // Dynamic Collection literal
            // var_name := {}
            Attribute::Mutable => {
                let start_line_number = &token_line_numbers[*i];
                return AstNode::VarDeclaration(
                    name.to_string(),
                    Box::new(new_collection(tokens, i, ast, start_line_number)),
                    is_exported,
                    data_type.to_owned(),
                    false,
                );
            }

            // Fixed Collection literal
            // var_name :: {}
            Attribute::Constant => {
                let start_line_number = &token_line_numbers[*i];
                return AstNode::VarDeclaration(
                    name.to_string(),
                    Box::new(new_collection(tokens, i, ast, start_line_number)),
                    is_exported,
                    data_type.to_owned(),
                    true,
                );
            }
        },
        _ => {
            // Maybe need to add a check that this is an expression after the assignment?
            let start_line_number = &token_line_numbers[*i];
            parsed_expr = create_expression(tokens, i, false, &ast, start_line_number, data_type);
        }
    }

    // Check if a type of collection has been created
    // Or whether it is a literal or expression
    // If the expression is an empty expression when the variable is NOT a function, return an error
    match parsed_expr {
        AstNode::RuntimeExpression(_, ref evaluated_type) => {
            return create_var_node(
                attribute,
                name.to_string(),
                parsed_expr.to_owned(),
                is_exported,
                evaluated_type.to_owned(),
            );
        }
        AstNode::Literal(ref token) => {
            let data_type = match token {
                Token::FloatLiteral(_) => DataType::Float,
                Token::StringLiteral(_) => DataType::String,
                Token::BoolLiteral(_) => DataType::Bool,
                _ => DataType::Inferred,
            };
            return create_var_node(
                attribute,
                name.to_string(),
                parsed_expr,
                is_exported,
                data_type,
            );
        }
        AstNode::Tuple(_, _) => {
            return create_var_node(
                attribute,
                name.to_string(),
                parsed_expr,
                is_exported,
                data_type.to_owned(),
            );
        }
        AstNode::Scene(_, _, _, _) => {
            return create_var_node(
                attribute,
                name.to_string(),
                parsed_expr,
                is_exported,
                DataType::Scene,
            );
        }
        AstNode::Error(err, line) => {
            return AstNode::Error(
                format!(
                    "Error: Invalid expression for variable assignment (creating new variable: {name}) at line {}: {}",
                    line, err
                )
                .to_string(),
                line,
            );
        }

        _ => {
            return AstNode::Error(
                format!("Invalid expression for variable assignment (creating new variable: {name}). Value was: {:?}", parsed_expr),
                token_line_numbers[*i - 1],
            );
        }
    }
}

fn create_var_node(
    attribute: Attribute,
    var_name: String,
    var_value: AstNode,
    is_exported: bool,
    data_type: DataType,
) -> AstNode {
    match attribute {
        Attribute::Constant | Attribute::TypeDeclaration => {
            return AstNode::VarDeclaration(
                var_name,
                Box::new(var_value),
                is_exported,
                data_type,
                true,
            );
        }
        Attribute::Mutable => {
            return AstNode::VarDeclaration(
                var_name,
                Box::new(var_value),
                is_exported,
                data_type,
                false,
            );
        }
    }
}

fn create_zero_value_var(data_type: DataType, name: String, is_exported: bool) -> AstNode {
    match data_type {
        DataType::Float => AstNode::VarDeclaration(
            name,
            Box::new(AstNode::Literal(Token::FloatLiteral(0.0))),
            is_exported,
            data_type,
            false,
        ),
        DataType::String => AstNode::VarDeclaration(
            name,
            Box::new(AstNode::Literal(Token::StringLiteral("".to_string()))),
            is_exported,
            data_type,
            false,
        ),
        DataType::Bool => AstNode::VarDeclaration(
            name,
            Box::new(AstNode::Literal(Token::BoolLiteral(false))),
            is_exported,
            data_type,
            false,
        ),
        _ => AstNode::VarDeclaration(
            name,
            Box::new(AstNode::Empty),
            is_exported,
            data_type,
            false,
        ),
    }
}
