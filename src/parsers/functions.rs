use super::{
    ast_nodes::{AstNode, Reference},
    build_ast::new_ast,
    parse_expression::create_expression,
};
use crate::{bs_types::DataType, Token};

pub fn create_function(
    name: String,
    tokens: &Vec<Token>,
    i: &mut usize,
    is_exported: bool,
    ast: &Vec<AstNode>,
    token_line_numbers: &Vec<u32>,
    variable_declarations: &Vec<Reference>,
) -> AstNode {
    /*
        funcName fn(arg type, arg2 type = default_value) -> returnType :
            // Function body
        end
    */

    // get args (tokens should currently be at the open parenthesis)
    let (args, arg_refs) =
        match parse_args(tokens, i, ast, token_line_numbers, variable_declarations) {
            Ok(args) => args,
            Err(err) => {
                return AstNode::Error(err.to_string(), token_line_numbers[*i]);
            }
        };

    *i += 1;

    // Return type is optional (can not return anything)
    let mut return_types: Vec<DataType> = Vec::new();
    if match &tokens[*i] {
        Token::Arrow => true,
        _ => false,
    } {
        *i += 1;
        return_types = match parse_return_type(tokens, i) {
            Ok(return_type) => return_type,
            Err(err) => {
                return AstNode::Error(err.to_string(), token_line_numbers[*i]);
            }
        };
    }

    // Should now be at the colon
    if &tokens[*i] != &Token::Colon {
        return AstNode::Error(
            "Expected ':' to open function scope".to_string(),
            token_line_numbers[*i],
        );
    }

    *i += 1;

    let return_type = if return_types.len() == 1 {
        return_types[0].to_owned()
    } else {
        DataType::Tuple(Box::new(return_types))
    };

    // The function ends with the 'end' keyword
    let function_body = new_ast(
        tokens.to_vec(),
        *i,
        token_line_numbers,
        arg_refs,
        &return_type,
    )
    .0;

    AstNode::Function(name, args, function_body, is_exported, return_type)
}

fn parse_args(
    tokens: &Vec<Token>,
    i: &mut usize,
    ast: &Vec<AstNode>,
    token_line_numbers: &Vec<u32>,
    variable_declarations: &Vec<Reference>,
) -> Result<(Vec<AstNode>, Vec<Reference>), &'static str> {
    let mut args = Vec::<AstNode>::new();
    let mut arg_refs = Vec::<Reference>::new();

    // Check if there are arguments
    let mut open_parenthesis = 0;
    let mut next_in_list: bool = true;

    while *i < tokens.len() {
        match &tokens[*i] {
            Token::OpenParenthesis => {
                open_parenthesis += 1;
            }
            Token::CloseParenthesis => {
                open_parenthesis -= 1;
                if open_parenthesis < 1 {
                    break;
                }
            }
            Token::Variable(arg_name) => {
                if !next_in_list {
                    return Err("Should have a comma to seperate arguments");
                }

                // Parse the argument
                /*
                    Arguments follow this syntax:

                    variables
                    arg_name type = default_value

                    no default value
                    arg_name type
                */

                // Make sure function arguments are not redeclared variables
                for var in variable_declarations {
                    if var.name == *arg_name {
                        return Err("Function arguments must have unique names");
                    }
                }

                // Check if there is a type keyword
                *i += 1;

                let data_type = match &tokens[*i] {
                    Token::TypeKeyword(data_type) => data_type.to_owned(),
                    _ => {
                        return Err("Expected type keyword after argument name");
                    }
                };

                // Check if there is a default value
                let mut default_value: Option<Box<AstNode>> = None;
                if match &tokens[*i + 1] {
                    Token::Assign => true,
                    _ => false,
                } {
                    *i += 2;
                    // Function args are similar to a tuple,
                    // So create expression is told it's a tuple inside brackets
                    // So it only parses up to a comma or closing parenthesis
                    default_value = Some(Box::new(create_expression(
                        tokens,
                        i,
                        true,
                        ast,
                        &token_line_numbers[*i],
                        &data_type,
                        false,
                        variable_declarations,
                    )));
                }

                args.push(AstNode::FunctionArg(
                    arg_name.to_owned(),
                    data_type.to_owned(),
                    default_value,
                ));

                arg_refs.push(Reference {
                    name: arg_name.to_owned(),
                    data_type,
                    is_const: true,
                });

                next_in_list = false;
            }

            Token::Comma => {
                next_in_list = true;
            }

            _ => {
                return Err("Invalid syntax for function arguments");
            }
        }

        *i += 1;
    }

    if open_parenthesis != 0 {
        return Err("Wrong number of parenthesis used when declaring function arguments");
    }

    return Ok((args, arg_refs));
}

fn parse_return_type(tokens: &Vec<Token>, i: &mut usize) -> Result<Vec<DataType>, &'static str> {
    let mut return_type = Vec::<DataType>::new();

    // Check if there is a return type
    let mut open_parenthesis = 0;
    let mut next_in_list: bool = true;
    while tokens[*i] != Token::Colon {
        match &tokens[*i] {
            Token::OpenParenthesis => {
                open_parenthesis += 1;
                *i += 1;
            }
            Token::CloseParenthesis => {
                open_parenthesis -= 1;
                *i += 1;
            }
            Token::TypeKeyword(type_keyword) => {
                if next_in_list {
                    return_type.push(type_keyword.to_owned());
                    next_in_list = false;
                    *i += 1;
                } else {
                    return Err("Should have a comma to seperate return types");
                }
            }
            Token::Comma => {
                next_in_list = true;
                *i += 1;
            }
            _ => {
                return Err("Invalid syntax for return type");
            }
        }
    }

    assert!(tokens[*i] == Token::Colon);

    if open_parenthesis != 0 {
        return Err("Wrong number of parenthesis used when declaring return type");
    }

    return Ok(return_type);
}
