use super::{
    ast::AstNode,
    create_scene_node::new_scene,
    parse_expression::{create_expression, eval_expression},
};
use crate::{bs_types::DataType, Token};

pub fn new_ast(tokens: &Vec<Token>, start_index: usize) -> (Vec<AstNode>, usize) {
    let mut ast = Vec::new();
    let mut i = start_index;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Comment(value) => {
                ast.push(AstNode::Comment(value.clone()));
            }

            Token::SceneHead(scene_head) => {
                ast.push(new_scene(scene_head, tokens, &mut i));
            }

            // New Function or Variable declaration or reference
            Token::Variable(name) => {
                ast.push(new_variable(name, tokens, &mut i));
            }

            Token::OpenCollection => {
                ast.push(
                    new_collection(tokens, &mut i)
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

            Token::Newline => {
                // Do nothing
            }

            Token::Print => {
                i += 1;
                ast.push(AstNode::Print(Box::new(create_expression(
                    tokens,
                    &mut i,
                ))));
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

fn new_variable(name: &String, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    
    // If already initialised, return a reference ast node
    if is_reference(tokens, i, name) {
        return AstNode::Ref(name.clone());
    }

    let mut var_is_const = true;

    *i += 1;
    match &tokens[*i] {
        &Token::Assign => {
            var_is_const = false;
        }
        &Token::Initialise => {}
        _=> { 
            return AstNode::Error("Expected ':' or '=' after variable name for initialising. Variable does not yet exsist".to_string());
        }
    }

    // Get value of variable
    *i += 1;

    let parsed_expr = create_expression(tokens, i);

    if var_is_const {
        return AstNode::ConstDeclaration(name.to_string(), Box::new(eval_expression(parsed_expr)));
    }

    // Check type or infer type
    *i += 1;

    AstNode::VarDeclaration(name.to_string(), Box::new(parsed_expr))
    // AstNode::Error("Invalid variable assignment".to_string())
}

// TO DO - SOME PLACEHOLDER CODE FOR FUNCTION DECLARATION
fn _new_function(tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut _function_name = String::new();
    let mut function_args = Vec::new();
    let mut _function_body = Vec::new();

    // Get function name
    match &tokens[*i] {
        Token::Variable(name) => {
            _function_name = name.clone();
        }
        _ => {
            return AstNode::Error("Expected function name".to_string());
        }
    }

    // Get function args
    *i += 1;
    if &tokens[*i] != &Token::OpenParenthesis {
        return AstNode::Error("Expected '(' for function args".to_string());
    }

    *i += 1;
    while &tokens[*i] != &Token::CloseParenthesis {
        match &tokens[*i] {
            Token::Variable(name) => {
                function_args.push(AstNode::VarDeclaration(
                    name.clone(),
                    Box::new(AstNode::Ref("".to_string())),
                ));
            }
            _ => {
                return AstNode::Error("Expected variable name for function args".to_string());
            }
        }
        *i += 1;
    }

    // TODO - Get function body

    AstNode::Function(_function_name, function_args, _function_body)
}

// Check if variable name has been used earlier in the vec of tokens, if it has return true
pub fn is_reference(tokens: &Vec<Token>, i: &usize, name: &String) -> bool {
    tokens[..*i].iter().rev().any(|token| match token {
        Token::Variable(var_name) => var_name == name,
        _ => false,
    })
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
                if expression_type != collection_type{
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
            _=> {
                return AstNode::Error("Expected ',' or '}' in collection after previous value".to_string());
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