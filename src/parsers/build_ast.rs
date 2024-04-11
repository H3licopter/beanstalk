use super::{
    ast::AstNode,
    create_scene_node::new_scene,
    parse_expression::create_expression,
};
use crate::{bs_types::DataType, Token};

pub fn new_ast(tokens: Vec<Token>, start_index: usize) -> (Vec<AstNode>, usize) {
    let mut ast = Vec::new();
    let mut i = start_index;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Comment(value) => {
                ast.push(AstNode::Comment(value.clone()));
            }

            Token::SceneHead(scene_head) => {
                ast.push(new_scene(scene_head, &tokens, &mut i));
            }

            // New Function or Variable declaration or reference
            Token::VarDeclaration(id) => {
                ast.push(new_variable(*id, &tokens, &mut i));
            }

            Token::OpenCollection => {
                ast.push(new_collection(&tokens, &mut i));
            }

            Token::Title => {
                i += 1;
                match &tokens[i] {
                    Token::StringLiteral(value) => {
                        ast.push(AstNode::Title(value.clone()));
                    }
                    _ => {
                        ast.push(AstNode::Error(
                            "Title must have a valid string as a argument".to_string(),
                        ));
                    }
                }
            }

            Token::Date => {
                i += 1;
                match &tokens[i] {
                    Token::StringLiteral(value) => {
                        ast.push(AstNode::Date(value.clone()));
                    }
                    _ => {
                        ast.push(AstNode::Error(
                            "Date must have a valid string as a argument".to_string(),
                        ));
                    }
                }
            }

            Token::Newline => {
                // Do nothing
            }

            Token::Print => {
                i += 1;
                ast.push(AstNode::Print(Box::new(create_expression(&tokens, &mut i))));
            }

            // Or stuff that hasn't been implemented yet
            _ => {
                ast.push(AstNode::Error("Invalid Token Used".to_string()));
            }
        }

        i += 1;
    }

    (ast, i)
}

// CAN ALSO RETURN A FUNCTION
fn new_variable(name: usize, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut var_is_const = true;

    *i += 1;
    match &tokens[*i] {
        &Token::Assign => {
            var_is_const = false;
        }
        &Token::Initialise => {}
        &Token::FunctionInitPrivate => {
            return new_function(false, name, tokens, i);
        }
        &Token::FunctionInitPublic => {
            return new_function(true, name, tokens, i);
        }
        _ => {
            return AstNode::Error("Expected ':' or '=' after variable name for initialising. Variable does not yet exsist".to_string());
        }
    }

    // Get value of variable
    *i += 1;
    let parsed_expr;

    // Check if collection
    if &tokens[*i] == &Token::OpenCollection {
        parsed_expr = new_collection(tokens, i);
    } else {
        parsed_expr = create_expression(tokens, i);
    }

    if var_is_const {
        return AstNode::Const(name, Box::new(parsed_expr));
    }

    AstNode::VarDeclaration(name, Box::new(parsed_expr))
    // AstNode::Error("Invalid variable assignment".to_string())
}

fn new_function(_public: bool, name: usize, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut function_args = AstNode::Collection(Vec::new());
    let function_body = Vec::new();

    // Get function args
    *i += 1;

    match &tokens[*i] {
        Token::OpenCollection => {
            function_args = new_collection(tokens, i);
        }
        // Can directly get args from an existing collection
        Token::VarDeclaration(_) => {
            // PROBABLY SHOULDEN'T LET ANY VARIABLE BE USED AS FUNCTION ARGS
            // but for now it's just here incase it's a collection
        }
        _ => {
            return AstNode::Error("Expected '(' for function args".to_string());
        }
    }
    if &tokens[*i] != &Token::OpenCollection {
        return AstNode::Error("Expected '(' for function args".to_string());
    }

    *i += 1;

    // TODO - Get function body

    AstNode::Function(name.clone(), Box::new(function_args), function_body)
}

pub fn new_collection(tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut collection = Vec::new();
    let mut collection_type = &DataType::Inferred;

    // Should always start with current token being an open collection
    *i += 1;

    // look for index of final CloseCollection in tokens,
    // And check if there is a type declaration after it
    let close_index = tokens.iter().position(|x| x == &Token::CloseCollection);
    match close_index {
        Some(index) => {
            if index + 1 < tokens.len() {
                match &tokens[index + 1] {
                    Token::TypeKeyword(data_type) => {
                        collection_type = data_type;
                    }
                    _ => {}
                }
            }
        }
        None => {
            return AstNode::Error("Expected closing '}' for collection".to_string());
        }
    }

    while *i < tokens.len() {
        // Parse the element inside of collection
        let element = create_expression(tokens, i);

        // Make sure the datatype is correct for the collection
        match element {
            AstNode::Expression(_, ref expression_type) => {
                if expression_type != collection_type {
                    return AstNode::Error("Invalid datatype inside collection".to_string());
                }
            }
            _ => { /* Should never happen */ }
        }

        collection.push(element);

        // Check to see if there is another element, or collection is closed.
        match &tokens[*i] {
            &Token::Comma => {
                *i += 1;
            }
            &Token::CloseCollection => {
                *i += 1;
                break;
            }
            _ => {
                return AstNode::Error(
                    "Expected ',' or '}' in collection after previous value".to_string(),
                );
            }
        }
    }

    AstNode::Collection(collection)
}

/*
match &tokens[*i] {
    // Infer type (CONSTANT VARIABLE)
    Token::Initialise => {}

    // Infer type (MUTABLE VARIABLE)
    Token::Assign => {
        var_is_const = false;
    }

    // Explicit Type Declarations
    Token::TypeInt => {
        type_declaration = DataType::Int;
    }
    Token::TypeFloat => {
        type_declaration = DataType::Float;
    }
    Token::TypeString => {
        type_declaration = DataType::String;
    }
    Token::TypeRune => {
        type_declaration = DataType::Rune;
    }

    // Function with implicit return type
    Token::OpenParenthesis => return new_function(tokens, i),

    _ => {
        return AstNode::Error(
            "Expected either type definition or another ':' or '=' for initialising"
                .to_string(),
        )
    }
}
*/
