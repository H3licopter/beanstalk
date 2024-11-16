use crate::{bs_types::DataType, Token};

use super::{
    ast_nodes::{AstNode, Node, Reference},
    collections::new_collection,
    expressions::parse_expression::{create_expression, get_args},
    functions::create_function,
};

pub fn create_new_var_or_ref(
    name: &String,
    variable_declarations: &mut Vec<Reference>,
    tokens: &Vec<Token>,
    i: &mut usize,
    is_exported: bool,
    ast: &Vec<AstNode>,
    token_line_numbers: &Vec<u32>,
) -> AstNode {
    let is_const = name.to_uppercase() == *name;

    if let Some(var) = variable_declarations.iter().find(|v| v.name == *name) {
        match var.data_type { 
            DataType::Function(ref argument_refs, ref return_type) => {
                // Parse arguments passed into the function
                let args = match get_args(tokens, i, ast, &token_line_numbers[*i], variable_declarations, argument_refs) {
                    Some(args) => args,

                    // Returning None here means no brackets, which means it's just a reference to the function
                    None => {
                        return AstNode::VarReference(var.name.to_owned(), var.data_type.to_owned());
                    }
                };

                return AstNode::FunctionCall(
                    name.to_owned(),
                    Box::new(args),
                    *return_type.to_owned(),
                );
            }
            _ => {},
        }
        if is_const {
            return AstNode::ConstReference(var.name.to_owned(), var.data_type.to_owned());
        }
        return AstNode::VarReference(var.name.to_owned(), var.data_type.to_owned());
    }

    new_variable(
        name,
        tokens,
        i,
        is_exported,
        ast,
        token_line_numbers,
        &mut *variable_declarations,
        is_const,
    )
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
    variable_declarations: &mut Vec<Reference>,
    is_const: bool,
) -> AstNode {
    *i += 1;
    let mut data_type = &DataType::Inferred;

    match &tokens[*i] {
        // Type is inferred
        &Token::Assign => {}

        &Token::FunctionKeyword => {
            *i += 1;
            return create_function(
                name.to_owned(),
                tokens,
                i,
                is_exported,
                ast,
                token_line_numbers,
                variable_declarations,
            );
        }

        // Has a type declaration
        &Token::TypeKeyword(ref type_keyword) => {
            data_type = type_keyword;
            *i += 1;

            match &tokens[*i] {
                &Token::Assign => {}
                // If this is the end of the assignment, it is an uninitalised variable
                // Currently just creates a zero value variable, should be uninitialised in future
                &Token::Newline | &Token::EOF => {
                    variable_declarations.push(Reference {
                        name: name.to_owned(),
                        data_type: data_type.to_owned(),
                        default_value: None,
                    });

                    return create_zero_value_var(
                        data_type.to_owned(),
                        name.to_string(),
                        is_exported,
                    );
                }
                _ => {
                    return AstNode::Error(
                        format!(
                            "Variable of type: {:?} does not exsist in this scope",
                            data_type
                        ),
                        token_line_numbers[*i],
                    );
                }
            }
        }

        // TO DO: Multiple assignments
        // &Token::Comma => {
        // }

        // Anything else is a syntax error
        _ => {
            return AstNode::Error(
                format!(
                    "'{}' - Invalid variable declaration: {:?}",
                    name, tokens[*i]
                ),
                token_line_numbers[*i],
            );
        }
    };

    // Current token (SHOULD BE) the assignment operator
    // Get assigned values
    *i += 1;

    let parsed_expr;
    match &tokens[*i] {
        // Check if this is a COLLECTION
        Token::OpenCurly => {
            if is_const {
                // Make a read only collection
            }

            // Dynamic Collection literal
            let start_line_number = &token_line_numbers[*i];
            let collection = new_collection(tokens, i, ast, start_line_number);
            match collection {
                AstNode::Collection(_, ref data_type) => {
                    let collection_type = data_type.to_owned();
                    variable_declarations.push(Reference {
                        name: name.to_owned(),
                        data_type: DataType::Collection(Box::new(collection_type.to_owned())),
                        default_value: None,
                    });
                    return AstNode::VarDeclaration(
                        name.to_string(),
                        Box::new(collection),
                        is_exported,
                        DataType::Collection(Box::new(collection_type)),
                        false,
                    );
                }
                _ => {
                    return AstNode::Error(
                        "Invalid collection".to_string(),
                        token_line_numbers[*i],
                    );
                }
            }
        }

        // create_expression will automatically handle tuples
        _ => {
            let start_line_number = &token_line_numbers[*i];
            parsed_expr = create_expression(
                tokens,
                i,
                false,
                &ast,
                start_line_number,
                data_type,
                false,
                &variable_declarations,
            );
        }
    }


    // Check if a type of collection / tuple has been created
    // Or whether it is a literal or expression
    // If the expression is an empty expression when the variable is NOT a function, return an error
    match parsed_expr {
        AstNode::RuntimeExpression(_, ref evaluated_type) => {
            return create_var_node(
                is_const,
                name.to_string(),
                parsed_expr.to_owned(),
                is_exported,
                evaluated_type.to_owned(),
                variable_declarations,
            );
        }
        AstNode::Literal(ref token) => {
            let data_type = match token {
                Token::FloatLiteral(_) => DataType::Float,
                Token::IntLiteral(_) => DataType::Int,
                Token::StringLiteral(_) => DataType::String,
                Token::BoolLiteral(_) => DataType::Bool,
                _ => DataType::Inferred,
            };
            return create_var_node(
                is_const,
                name.to_string(),
                parsed_expr,
                is_exported,
                data_type,
                variable_declarations,
            );
        }
        AstNode::Tuple(ref values, _) => {
            let mut tuple_data_type = Vec::new();
            for value in values {
                tuple_data_type.push(value.get_type());
            }

            let data_type = DataType::Tuple(Box::new(tuple_data_type));

            return create_var_node(
                is_const,
                name.to_string(),
                parsed_expr,
                is_exported,
                data_type,
                variable_declarations,
            );
        }
        AstNode::Scene(_, _, _, _) => {
            return create_var_node(
                is_const,
                name.to_string(),
                parsed_expr,
                is_exported,
                DataType::Scene,
                variable_declarations,
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
    is_const: bool,
    var_name: String,
    var_value: AstNode,
    is_exported: bool,
    data_type: DataType,
    variable_declarations: &mut Vec<Reference>,
) -> AstNode {
    variable_declarations.push(Reference {
        name: var_name.to_owned(),
        data_type: data_type.to_owned(),
        default_value: None,
    });

    if is_const {
        return AstNode::VarDeclaration(
            var_name,
            Box::new(var_value),
            is_exported,
            data_type,
            true,
        );
    }

    return AstNode::VarDeclaration(var_name, Box::new(var_value), is_exported, data_type, false);
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
        DataType::Int => AstNode::VarDeclaration(
            name,
            Box::new(AstNode::Literal(Token::IntLiteral(0))),
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
