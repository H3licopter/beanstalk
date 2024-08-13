use crate::{bs_types::DataType, Token};

use super::{ast_nodes::AstNode, collections::new_array, create_scene_node::new_scene, functions::create_function, parse_expression::create_expression};

#[derive(PartialEq, Debug)]
enum Attribute {
    Constant,
    Mutable,
    TypeDeclaration,
}


// CAN RETURN:
// VarDeclaration, Const, Error, Function, Tuple
pub fn new_variable(
    name: usize,
    tokens: &Vec<Token>,
    i: &mut usize,
    is_exported: bool,
    ast: &Vec<AstNode>,
    token_line_numbers: &Vec<u32>,
) -> AstNode {
    let mut attribute;

    *i += 1;
    match &tokens[*i] {
        &Token::AssignConstant => {
            attribute = Attribute::Constant;
        }
        &Token::AssignVariable => {
            attribute = Attribute::Mutable;
        }
        &Token::Colon => {
            attribute = Attribute::TypeDeclaration;
        }
        &Token::Comma => {
            // TO DO: Multiple assignments
            attribute = Attribute::Constant;
        }

        // Uninitialised variable
        &Token::Newline => {
            return AstNode::VarDeclaration(name, Box::new(AstNode::Empty), is_exported, DataType::Inferred);
        }
        _ => {
            return AstNode::Error(
                "Expected ':' or '=' after variable name for initialising. Variable does not yet exsist".to_string(),
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
                        return create_function(name, parsed_expr, tokens, i, is_exported, token_line_numbers);
                    }
                    // Not a function
                    _ => {

                    }
                }
            }
        }

        Token::TypeKeyword(type_keyword) => {
            data_type = type_keyword;
            let next_token = if *i + 1 >= tokens.len() { 
                return AstNode::VarDeclaration(name, Box::new(AstNode::Empty), is_exported, data_type.to_owned());
            } else { 
                &tokens[*i + 1]
            };

            // Check after the type declaration
            match next_token {
                Token::Newline | Token::EOF => {
                    return create_zero_value_var(data_type.to_owned(), name, is_exported);
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
                    return AstNode::Error(format!("Unexpected token after type declaration: {:?}", next_token), token_line_numbers[*i]);
                }
            }
            let start_line_number = &token_line_numbers[*i];
            parsed_expr = create_expression(tokens, i, false, &ast, start_line_number, data_type);
        }

        Token::OpenScope => match attribute {
            // New struct
            // var_name : {}
            Attribute::TypeDeclaration => {
                let start_line_number = &token_line_numbers[*i];
                return AstNode::Struct(name, Box::new(new_array(tokens, i, ast, start_line_number)), is_exported)
            }

            // Struct literal
            // var_name :: {}
            // var_name := {}
            Attribute::Mutable | Attribute::Constant => {
                let start_line_number = &token_line_numbers[*i];
                return AstNode::VarDeclaration(
                    name,
                    Box::new(new_array(tokens, i, ast, start_line_number)),
                    is_exported,
                    data_type.to_owned(),
                )
            }
        }

        Token::SceneHead(scene_head) => {
            if data_type == &DataType::Inferred {
                data_type = &DataType::Scene;
            }

            // in future, can just parse the expression if adding scenes together will be a thing
            let start_line_number = &token_line_numbers[*i];
            return AstNode::VarDeclaration(name, Box::new(new_scene(scene_head, tokens, i, ast, start_line_number)), is_exported, data_type.to_owned());
        }
        _ => {
            // Maybe need to add a check that this is an expression after the assignment?
            let start_line_number = &token_line_numbers[*i];
            parsed_expr = create_expression(tokens, i, false, &ast, start_line_number, data_type);
        }
    }

    // create_expression does not move the token index past the closing token so it is incremented past it here
    *i += 1;

    // Check if a type of collection has been created
    // Or whether it is a literal or expression
    // If the expression is an empty expression when the variable is NOT a function, return an error
    match parsed_expr {
        AstNode::Expression(_, _) | AstNode::Tuple(_, _) => {
            return create_var_node(attribute, name, parsed_expr, is_exported, data_type.to_owned());
        }
        AstNode::Error(err, line) => {
            return AstNode::Error(
                format!(
                    "Error: Invalid expression for variable assignment (creating new variable: {name}) at line {}: {}",
                    line, err
                )
                .to_string(),
                token_line_numbers[*i],
            );
        }
        _ => {
            return AstNode::Error(
                format!("Invalid expression for variable assignment (creating new variable: {name}) at line {}", token_line_numbers[*i]),
                token_line_numbers[*i],
            );
        }
    }
}

fn create_var_node(
    attribute: Attribute,
    var_name: usize,
    var_value: AstNode,
    is_exported: bool,
    data_type: DataType,
) -> AstNode {
    match attribute {
        Attribute::Constant | Attribute::TypeDeclaration => {
            return AstNode::Const(var_name, Box::new(var_value), is_exported, data_type);
        }
        Attribute::Mutable => {
            return AstNode::VarDeclaration(var_name, Box::new(var_value), is_exported, data_type);
        }
    }
}

pub fn find_var_declaration_index(ast: &Vec<AstNode>, var_name: &usize) -> usize {
    for (i, node) in ast.iter().enumerate().rev() {
        match node {
            AstNode::VarDeclaration(name, _, _, _) | AstNode::Const(name, _, _, _) => {
                if name == var_name {
                    return i;
                }
            }
            _ => {}
        }
    }

    0
}

fn create_zero_value_var(data_type: DataType, name: usize, is_exported: bool) -> AstNode {
    match data_type {
        DataType::Float => {
            AstNode::VarDeclaration(name, Box::new(AstNode::Literal(Token::FloatLiteral(0.0))), is_exported, data_type)
        }
        DataType::String => {
            AstNode::VarDeclaration(name, Box::new(AstNode::Literal(Token::StringLiteral("".to_string()))), is_exported, data_type)
        }
        DataType::Bool => {
            AstNode::VarDeclaration(name, Box::new(AstNode::Literal(Token::BoolLiteral(false))), is_exported, data_type)
        }
        _ => {
            AstNode::VarDeclaration(name, Box::new(AstNode::Empty), is_exported, data_type)
        }
    }
}