use colour::red_ln;

use crate::{
    bs_types::DataType,
    parsers::{
        ast_nodes::{AstNode, Reference},
        create_scene_node::new_scene,
        tuples::new_tuple,
    },
    Token,
};

use super::eval_expression::evaluate_expression;

// If the datatype is a collection
// The the expression must only contain references to collections
// Or collection literals
pub fn create_expression(
    tokens: &Vec<Token>,
    i: &mut usize,
    inside_tuple: bool,
    ast: &Vec<AstNode>,
    starting_line_number: &u32,
    data_type: &mut DataType,
    inside_brackets: bool,
    variable_declarations: &Vec<Reference>,
) -> AstNode {
    let mut expression = Vec::new();
    let number_union = DataType::Union(Box::new(vec![DataType::Int, DataType::Float]));

    if inside_brackets {
        *i += 1;
        if match data_type {
            DataType::Tuple(_) => true,
            _ => false,
        } {
            return new_tuple(
                None,
                tokens,
                i,
                data_type,
                ast,
                starting_line_number,
                variable_declarations,
            );
        }
    }

    // Loop through the expression and create the AST nodes (increment i each time)
    // Figure out the type it should be from the data
    // DOES NOT MOVE TOKENS PAST THE CLOSING TOKEN
    let mut next_number_negative = false;
    while let Some(token) = tokens.get(*i) {
        match token {
            // Conditions that close the expression
            Token::CloseParenthesis => {
                if inside_brackets {
                    *i += 1;
                    if expression.is_empty() {
                        return AstNode::Empty;
                    }
                    break;
                } else {
                    if inside_tuple {
                        break;
                    }
                    *i += 1;
                    // Mismatched brackets, return an error
                    return AstNode::Error(
                        "Mismatched brackets in expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
            }

            Token::OpenParenthesis => {
                return create_expression(
                    tokens,
                    i,
                    false,
                    ast,
                    starting_line_number,
                    data_type,
                    true,
                    variable_declarations,
                );
            }

            Token::EOF | Token::SceneClose(_) | Token::Arrow | Token::Colon | Token::End => {
                if inside_brackets {
                    return AstNode::Error(
                        "Not enough closing parenthesis for expression. Need more ')' at the end of the expression!".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                break;
            }

            Token::Newline => {
                // Fine if inside of brackets (not closed yet)
                // Otherwise break out of the expression
                if inside_brackets {
                    continue;
                } else {
                    break;
                }
            }

            Token::Comma => {
                if inside_tuple {
                    break;
                }
                *i += 1;

                if inside_brackets {
                    let eval_first_expr = evaluate_expression(
                        AstNode::Expression(expression, starting_line_number.to_owned()),
                        data_type,
                        ast,
                    );
                    return new_tuple(
                        Some(eval_first_expr),
                        tokens,
                        i,
                        data_type,
                        ast,
                        starting_line_number,
                        variable_declarations,
                    );
                }

                return AstNode::Error(
                    "Comma found outside of tuple".to_string(),
                    starting_line_number.to_owned(),
                );
            }

            // Check if name is a reference to another variable or function call
            Token::Variable(name) => {
                let var = variable_declarations.iter().find(|var| var.name == *name);
                match var {
                    Some(var) => {
                        // If this expression is inferring it's type from the expression
                        if *data_type == DataType::Inferred {
                            *data_type = var.data_type.to_owned();
                        }

                        // Check if this is a tuple/type/collection that is being accessed by a dot
                        match &var.data_type {
                            DataType::Tuple(inner_types) => {
                                // Check if this is a tuple access
                                if let Some(Token::Dot) = tokens.get(*i + 1) {
                                    // Move past the dot
                                    *i += 2;

                                    // Make sure an integer is next
                                    if let Some(Token::IntLiteral(index)) = tokens.get(*i) {
                                        // Check this is a valid index
                                        // Usize will flip to max number if negative
                                        // Maybe in future negative indexes with be supported (minus from the end)
                                        let idx: usize = *index as usize;
                                        if idx >= inner_types.len() {
                                            return AstNode::Error(
                                                format!(
                                                    "Index {} out of range for tuple '{}'",
                                                    index, var.name
                                                ),
                                                starting_line_number.to_owned(),
                                            );
                                        }
                                        // Check the accessed item in the tuple is the same type as the expression
                                        // Or let it through if this expression is being coerced to a string
                                        let tuple_item_type = &inner_types[idx];
                                        if !check_if_valid_type(&tuple_item_type,data_type) {
                                            return AstNode::Error(
                                                format!(
                                                    "Tuple item from '{}' is of type {:?}, but used in an expression of type {:?}",
                                                    var.name, var.data_type, data_type
                                                ),
                                                starting_line_number.to_owned(),
                                            );
                                        }
                                        expression.push(AstNode::TupleAccess(
                                            var.name.to_owned(),
                                            *index as usize,
                                            tuple_item_type.to_owned(),
                                        ));

                                        *i += 1;
                                        continue;
                                    } else {
                                        return AstNode::Error(
                                            format!(
                                                "Expected an integer index to access tuple '{}'",
                                                var.name
                                            ),
                                            starting_line_number.to_owned(),
                                        );
                                    }
                                }
                            }

                            DataType::Collection(inner_types) => {
                                // Check if this is a collection access
                                if let Some(Token::Dot) = tokens.get(*i + 1) {
                                    // Make sure the type of the collection is the same as the type of the expression
                                    if !check_if_valid_type(&inner_types, data_type) {
                                        return AstNode::Error(
                                            format!(
                                                "Collection '{}' is of type {:?}, but used in an expression of type {:?}",
                                                var.name, var.data_type, data_type
                                            ),
                                            starting_line_number.to_owned(),
                                        );
                                    }

                                    // Move past the dot
                                    *i += 2;

                                    // Make sure an integer is next
                                    if let Some(Token::IntLiteral(index)) = tokens.get(*i) {
                                        expression.push(AstNode::CollectionAccess(
                                            var.name.to_owned(),
                                            *index as usize,
                                            *inner_types.to_owned(),
                                        ));
                                        *i += 1;
                                        continue;
                                    } else {
                                        return AstNode::Error(
                                            format!(
                                                "Expected an integer index to access collection '{}'",
                                                var.name
                                            ),
                                            starting_line_number.to_owned(),
                                        );
                                    }
                                }
                            }

                            DataType::Function(argument_types, return_type) => {
                                // FUNCTION CALLS
                                // move past the variable name
                                *i += 1;
                                match get_args(
                                    tokens,
                                    i,
                                    ast,
                                    starting_line_number,
                                    variable_declarations,
                                    argument_types,
                                ) {
                                    Some(args) => {
                                        if !check_if_valid_type(&return_type, data_type) {
                                            return AstNode::Error(
                                                format!(
                                                    "Function '{}' returns type {:?}, but used in an expression of type {:?}",
                                                    var.name, return_type, data_type
                                                ),
                                                starting_line_number.to_owned(),
                                            );
                                        }

                                        expression.push(AstNode::FunctionCall(
                                            var.name.to_owned(),
                                            Box::new(args),
                                            *return_type.clone(),
                                        ));

                                        *i += 1;
                                        continue;
                                    }

                                    // Just a reference to a function
                                    None => {}
                                };
                            }
                            _ => {}
                        }

                        // If the variables type is known and not the same as the type of the expression
                        // Return a type error
                        if !check_if_valid_type(&var.data_type, data_type) {
                            return AstNode::Error(
                                format!(
                                    "Variable {} is of type {:?}, but used in an expression of type {:?}",
                                    var.name, var.data_type, data_type
                                ),
                                starting_line_number.to_owned(),
                            );
                        }

                        if var.name.to_uppercase() == var.name {
                            expression.push(AstNode::ConstReference(
                                var.name.to_owned(),
                                var.data_type.to_owned(),
                            ));
                        } else {
                            expression.push(AstNode::VarReference(
                                var.name.to_owned(),
                                var.data_type.to_owned(),
                            ));
                        };
                    }
                    None => {
                        expression.push(AstNode::Error(
                            format!("Variable {} not found in scope", name),
                            starting_line_number.to_owned(),
                        ));
                    }
                }
            }

            // Check if is a literal
            Token::FloatLiteral(mut float) => {
                if !check_if_valid_type(&DataType::Float, data_type) {
                    return AstNode::Error(
                        "Float literal used in non-float expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                if next_number_negative {
                    float = -float;
                    next_number_negative = false;
                }
                expression.push(AstNode::Literal(Token::FloatLiteral(float)));
            }
            Token::IntLiteral(int) => {
                if !check_if_valid_type(&DataType::Int, data_type) {
                    return AstNode::Error(
                        "Int literal used in non-integer expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                if next_number_negative {
                    expression.push(AstNode::Literal(Token::IntLiteral(-(*int as i64))));
                    next_number_negative = false;
                } else {
                    expression.push(AstNode::Literal(Token::IntLiteral(*int as i64)));
                }
            }
            Token::StringLiteral(string) => {
                if !check_if_valid_type(&DataType::String, data_type) {
                    return AstNode::Error(
                        "String literal used in non-string expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::Literal(Token::StringLiteral(string.clone())));
            }

            // Scenes - Create a new scene node
            // Maybe scenes can be added together like strings
            Token::SceneHead | Token::ParentScene => {
                if !check_if_valid_type(&DataType::Scene, data_type) {
                    return AstNode::Error(
                        "Scene used in non-scene expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                return new_scene(tokens, i, &ast, starting_line_number, variable_declarations);
            }

            // OPERATORS
            // Will push as a string so shunting yard can handle it later just as a string
            Token::Negative => {
                next_number_negative = true;
            }

            // BINARY OPERATORS
            Token::Add => {
                expression.push(AstNode::BinaryOperator(token.to_owned(), 1));
            }
            Token::Subtract => {
                if !check_if_valid_type(&number_union, data_type) {
                    return AstNode::Error(
                        "Subtraction used in non-numerical expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::BinaryOperator(token.to_owned(), 1));
            }
            Token::Multiply => {
                if !check_if_valid_type(&number_union,  data_type) {
                    return AstNode::Error(
                        "Multiplication used in non-numerical expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::BinaryOperator(token.to_owned(), 2));
            }
            Token::Divide => {
                if !check_if_valid_type(&number_union, data_type) {
                    return AstNode::Error(
                        "Division used in non-numerical expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::BinaryOperator(token.to_owned(), 2));
            }
            Token::Modulus => {
                if !check_if_valid_type(&number_union,  data_type) {
                    return AstNode::Error(
                        "Modulus used in non-numerical expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::BinaryOperator(token.to_owned(), 2));
            }

            // LOGICAL OPERATORS
            Token::Equal => {
                expression.push(AstNode::LogicalOperator(Token::Equal, 5));
            }
            Token::LessThan => {
                expression.push(AstNode::LogicalOperator(Token::LessThan, 5));
            }
            Token::LessThanOrEqual => {
                expression.push(AstNode::LogicalOperator(Token::LessThanOrEqual, 5));
            }
            Token::GreaterThan => {
                expression.push(AstNode::LogicalOperator(Token::GreaterThan, 5));
            }
            Token::GreaterThanOrEqual => {
                expression.push(AstNode::LogicalOperator(Token::GreaterThanOrEqual, 5));
            }
            Token::And => {
                expression.push(AstNode::LogicalOperator(Token::And, 4));
            }
            Token::Or => {
                expression.push(AstNode::LogicalOperator(Token::Or, 3));
            }

            _ => {
                expression.push(AstNode::Error(
                    format!(
                        "Invalid Expression: {:?}, must be assigned with a valid datatype",
                        token
                    ),
                    starting_line_number.to_owned(),
                ));
            }
        }

        *i += 1;
    }

    return evaluate_expression(
        AstNode::Expression(expression, starting_line_number.to_owned()),
        data_type,
        ast,
    );
}

// RETURNING NONE MEANS NOT A FUNCTION CALL -> JUST A REFERENCE
pub fn get_args(
    tokens: &Vec<Token>,
    i: &mut usize,
    ast: &Vec<AstNode>,
    token_line_number: &u32,
    variable_declarations: &Vec<Reference>,
    argument_refs: &Vec<Reference>,
) -> Option<AstNode> {
    if *i >= tokens.len() {
        return None;
    }

    // TO DO: Check the argument refs, if there are multiple, pass in a tuple of the correct types
    let mut data_type = if argument_refs.len() > 1 {
        // Create tuple of the argument types
        DataType::Tuple(Box::new(
            argument_refs
                .iter()
                .map(|arg| arg.data_type.to_owned())
                .collect(),
        ))
    } else if argument_refs.len() == 1 {
        argument_refs[0].data_type.to_owned()
    } else {
        DataType::None
    };

    // Check if the current token is an open bracket
    // This can be passed an empty tuple
    // So hopefully there will be a type error,
    // if more than 0 arguments are passed in the case of a function call with 0 args
    // Will probably be faster to check specifically for the empty tuple case before parsing in the future.
    match &tokens[*i] {
        // Check if open bracket
        Token::OpenParenthesis => Some(create_expression(
            tokens,
            &mut *i,
            false,
            ast,
            token_line_number,
            &mut data_type,
            true,
            variable_declarations,
        )),
        _ => None,
    }
}

fn check_if_valid_type(data_type: &DataType, accepted_type: &mut DataType) -> bool {
    match accepted_type {
        DataType::Inferred => {
            *accepted_type = data_type.to_owned();
            true
        }
        DataType::CoerseToString => true,
        DataType::Union(types) => {
            for t in &**types {
                if data_type == t {
                    return true;
                }
            }
            red_ln!(
                "Type Error: Expected type {:?}, but got type {:?}",
                accepted_type,
                data_type
            );
            false
        }
        _ => {
            if data_type == accepted_type {
                true
            } else {
                red_ln!(
                    "Type Error: Expected type {:?}, but got type {:?}",
                    accepted_type,
                    data_type
                );
                false
            }
        }
    }
}
