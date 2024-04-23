use super::{
    ast::{AstNode, CollectionType},
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
                ast.push(
                    new_variable(*id, &tokens, &mut i)
                );
            }
            Token::Reference(var_index) => {
                ast.push(
                    create_reference(&tokens, var_index)
                );
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

            Token::Newline | Token::Empty | Token::ModuleStart(_) => {
                // Do nothing for now
            }

            Token::Print => {
                i += 1;
                ast.push(AstNode::Print(Box::new(create_expression(&tokens, &mut i))));
            }


            // Or stuff that hasn't been implemented yet
            _ => {
                ast.push(AstNode::Error(format!("Invalid Token Used: {:?}", &tokens[i]).to_string()));
            }
        }

        i += 1;
    }

    (ast, i)
}

// CAN RETURN:
// VarDeclaration, Const, Error, Function, Tuple
fn new_variable(name: usize, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    
    // Currently also whether a function is private
    let mut var_is_const = true;

    *i += 1;
    match &tokens[*i] {
        &Token::Assign => {
            var_is_const = false;
        }
        &Token::AssignConstant => {}
        &Token::Comma => {
            // TO DO: Multiple assignments
        }

        // Uninitialised variable
        &Token::Newline => {
            return AstNode::VarDeclaration(name, Box::new(AstNode::Empty));
        }
        _ => {
            return AstNode::Error("Expected ':' or '=' after variable name for initialising. Variable does not yet exsist".to_string());
        }
    }

    // Get assigned values
    // Can also be function args
    *i += 1;

    let parsed_expr = create_expression(tokens, i);

    // Check if a type of collection has been created
    // Or whether it is a literal or expression
    match parsed_expr {
        AstNode::Expression(_, _) => {
            if var_is_const {
                return AstNode::Const(name, Box::new(parsed_expr));
            }
            return AstNode::VarDeclaration(name, Box::new(parsed_expr));
        }
        AstNode::Literal(_) => {
            if var_is_const {
                return AstNode::Const(name, Box::new(parsed_expr));
            }
            return AstNode::VarDeclaration(name, Box::new(parsed_expr));
        }
        AstNode::Collection(_, _, _) => {
            if var_is_const {
                return AstNode::Const(name, Box::new(parsed_expr));
            }
            return AstNode::VarDeclaration(name, Box::new(parsed_expr));
        }
        AstNode::Error(_) => {
            return AstNode::Error("Invalid expression for variable assignment".to_string());
        }
        _ => {}
    }

    AstNode::VarDeclaration(name, Box::new(parsed_expr))
    // AstNode::Error("Invalid variable assignment".to_string())
}

// Called from new_variable
fn new_function(_public: bool, name: usize, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let function_args;
    let function_body = Vec::new();

    // Get function args
    *i += 1;

    match &tokens[*i] {
        Token::OpenScope => {
            function_args = new_collection(tokens, i);
        }
        // Can directly get args from an existing collection
        Token::VarDeclaration(_) => {
            function_args = new_variable(0, tokens, i);
        }
        _ => {
            return AstNode::Error("Expected '(' for function args".to_string());
        }
    }
    if &tokens[*i] != &Token::CloseScope {
        return AstNode::Error("Expected '(' for function args".to_string());
    }

    *i += 1;

    // TODO - Get function body

    AstNode::Function(name.clone(), Box::new(function_args), function_body)
}


pub fn new_collection(tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut collection = Vec::new();
    let mut appended_type = DataType::Inferred;
    let mut constant = true;

    // CURRENTLY JUST PARSING AS IF IT'S A NORMAL ARRAY
    let collection_type = CollectionType::Array;

    // Should always start with current token being an open collection
    *i += 1;

    // look for index of final CloseCollection in tokens,
    // And check if there is a type declaration after it
    let close_index = tokens.iter()
        .position(|x| x == &Token::CloseScope);

    match close_index {
        Some(index) => {
            if index + 1 < tokens.len() {
                match &tokens[index + 1] {
                    Token::TypeKeyword(data_type) => {
                        appended_type = data_type.clone();
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
        
        
        // TO DO: FIRST CHECK IF STRUCT WITH NAMES PARAMS


        // Parse the element inside of collection
        let element = create_expression(tokens, i);

        // Make sure the datatype is correct for the collection
        match element {
            AstNode::Expression(_, ref expression_type) => {
                if appended_type == DataType::Inferred {
                    appended_type = expression_type.clone();
                } else if *expression_type != appended_type {
                    return AstNode::Error("Invalid datatype inside collection".to_string());
                }
                constant = false;
            }
            AstNode::Literal(ref token) => {
                if appended_type == DataType::Inferred {
                    appended_type = match token {
                        Token::IntLiteral(_) => DataType::Int,
                        Token::FloatLiteral(_) => DataType::Float,
                        Token::StringLiteral(_) => DataType::String,
                        Token::RuneLiteral(_) => DataType::Rune,
                        _ => DataType::Inferred,
                    };
                } else {
                    match token {
                        Token::IntLiteral(_) => {
                            if appended_type != DataType::Int {
                                return AstNode::Error("Invalid datatype inside collection".to_string());
                            }
                        }
                        Token::FloatLiteral(_) => {
                            if appended_type != DataType::Float {
                                return AstNode::Error("Invalid datatype inside collection".to_string());
                            }
                        }
                        Token::StringLiteral(_) => {
                            if appended_type != DataType::String {
                                return AstNode::Error("Invalid datatype inside collection".to_string());
                            }
                        }
                        Token::RuneLiteral(_) => {
                            if appended_type != DataType::Rune {
                                return AstNode::Error("Invalid datatype inside collection".to_string());
                            }
                        }
                        _ => {}
                    }
                }
                constant = true;
            }
            _ => { /* Should never happen */ }
        }

        collection.push(element);

        // Check to see if there is another element, or collection is closed.
        match &tokens[*i] {
            &Token::Comma => {
                *i += 1;
            }
            &Token::OpenScope => {
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

    AstNode::Collection(collection, collection_type, constant)
}

fn create_reference(tokens: &Vec<Token>, var_index: &usize) -> AstNode {

    // Should never be out of bounds right?
    match &tokens[var_index + 1] {
        Token::Assign => {
            return AstNode::VarReference(*var_index);
        }
        Token::AssignConstant => {
            return AstNode::ConstReference(*var_index);
        }
        _ => {
            return AstNode::Error("Expected variable or reference after '&'".to_string());
        }
    }
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
