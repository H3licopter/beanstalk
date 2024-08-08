use colour::red_ln;

use super::{
    ast_nodes::AstNode,
    collections::new_array,
    create_scene_node::new_scene,
    parse_expression::{create_expression, eval_expression},
};
use crate::{bs_types::DataType, Token};

#[derive(PartialEq, Debug)]
enum Attribute {
    Exported,
    Constant,
    Mutable,
    Comptime,
}

pub fn new_ast(tokens: Vec<Token>, start_index: usize, token_line_numbers: &Vec<u32>) -> (Vec<AstNode>, usize) {
    let mut ast = Vec::new();
    let mut i = start_index;
    let mut attributes: Vec<Attribute> = Vec::new();

    while i < tokens.len() {
        match &tokens[i] {
            Token::Comment(value) => {
                ast.push(AstNode::Comment(value.clone()));
            }

            Token::SceneHead(scene_head) => {
                ast.push(new_scene(scene_head, &tokens, &mut i, &ast, token_line_numbers));
            }
            Token::ModuleStart(_) => {
                // In future, need to structure into code blocks
            }

            // New Function or Variable declaration
            Token::VarDeclaration(id) => {
                // Need to determine if it is a const that compiles to a literal, should just push a literal in that case
                ast.push(new_variable(
                    *id,
                    &tokens,
                    &mut i,
                    attributes.contains(&Attribute::Exported),
                    &ast,
                    token_line_numbers,
                ));
            }

            Token::Export => {
                attributes.push(Attribute::Exported);
            }

            Token::VarReference(id) => {
                ast.push(AstNode::VarReference(find_var_declaration_index(&ast, id)));
            }
            Token::ConstReference(id) => {
                ast.push(AstNode::ConstReference(find_var_declaration_index(
                    &ast, id,
                )));
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
                            token_line_numbers[i],
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
                            token_line_numbers[i],
                        ));
                    }
                }
            }

            Token::Newline | Token::Empty | Token::SceneClose(_) | Token::Whitespace => {
                // Do nothing for now
            }

            Token::Print => {
                i += 1;
                ast.push(AstNode::Print(Box::new(eval_expression(
                    create_expression(&tokens, &mut i, false, &ast, token_line_numbers),
                    &DataType::Inferred,
                    &ast,
                ))));
            }

            Token::DeadVarible => {
                // Remove entire declaration or scope of variable declaration
                // So don't put any dead code into the AST
                skip_dead_code(&tokens, &mut i);
            }

            Token::EOF => {
                break;
            }

            // Or stuff that hasn't been implemented yet
            _ => {
                ast.push(AstNode::Error(
                    format!("Compiler Error: Token not recognised by AST parser when creating AST: {:?}", &tokens[i]).to_string(),
                    token_line_numbers[i],
                ));
            }
        }

        i += 1;
    }

    (ast, i)
}

// CAN RETURN:
// VarDeclaration, Const, Error, Function, Tuple
fn new_variable(
    name: usize,
    tokens: &Vec<Token>,
    i: &mut usize,
    is_exported: bool,
    ast: &Vec<AstNode>,
    token_line_numbers: &Vec<u32>,
) -> AstNode {
    let attribute;

    *i += 1;
    match &tokens[*i] {
        &Token::AssignConstant => {
            attribute = Attribute::Constant;
        }
        &Token::AssignVariable => {
            attribute = Attribute::Mutable;
        }
        &Token::Colon => {
            attribute = Attribute::Comptime;
        }
        &Token::Comma => {
            // TO DO: Multiple assignments
            attribute = Attribute::Constant;
        }

        // Uninitialised variable
        &Token::Newline => {
            return AstNode::VarDeclaration(name, Box::new(AstNode::Empty), is_exported);
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

    // Check if array/struct/choice/scene
    match &tokens[*i] {
        Token::OpenScope => match attribute {
            Attribute::Comptime => {
                return AstNode::Struct(name, Box::new(new_array(tokens, i, ast, token_line_numbers)), is_exported)
            }
            Attribute::Mutable | Attribute::Constant => {
                return AstNode::VarDeclaration(
                    name,
                    Box::new(new_array(tokens, i, ast, token_line_numbers)),
                    is_exported,
                )
            }
            _ => {
                return AstNode::Error(
                    "Invalid assignment declaration for collection - possibly not supported yet?"
                        .to_string(),
                    token_line_numbers[*i],
                );
            }
        },
        Token::SceneHead(scene_head) => match attribute {
            Attribute::Comptime => {
                return AstNode::Const(
                    name,
                    Box::new(new_scene(scene_head, tokens, i, &ast, token_line_numbers)),
                    is_exported,
                )
            }
            Attribute::Mutable | Attribute::Constant => {
                return AstNode::VarDeclaration(
                    name,
                    Box::new(new_scene(scene_head, tokens, i, &ast, token_line_numbers)),
                    is_exported,
                )
            }
            _ => {
                return AstNode::Error(
                    "Invalid assignment declaration for scene - possibly not supported yet?"
                        .to_string(),
                    token_line_numbers[*i],
                );
            }
        },
        _ => {}
    }

    let mut data_type = &DataType::Inferred;
    // Can be a collection, expression, literal or empty tuple
    let parsed_expr = create_expression(tokens, i, false, &ast, token_line_numbers);

    // create_expression does not move the token index past the closing token so it is incremented past it here
    *i += 1;

    // Check if the variable is a function, prototype, choice, has a type declaration or an exsisting choice/prototype type
    match &tokens[*i] {
        Token::Arrow => {
            return new_function(name, parsed_expr, tokens, i, is_exported, token_line_numbers);
        }
        Token::TypeKeyword(type_declaration) => {
            data_type = type_declaration;
        }
        _ => {
            *i -= 1;
        }
    }

    // Check if a type of collection has been created
    // Or whether it is a literal or expression
    // If the expression is an empty expression when the variable is NOT a function, return an error
    match parsed_expr {
        AstNode::Expression(_, _) | AstNode::Tuple(_, _) => {
            let evaluated_expression = eval_expression(parsed_expr, data_type, ast);
            return create_var_node(attribute, name, evaluated_expression, is_exported, &token_line_numbers[*i]);
        }
        // AstNode::Collection(items, collection_type) => {

        // }
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
                "Invalid expression for variable assignment".to_string(),
                token_line_numbers[*i],
            );
        }
    }
}

// Called from new_variable
fn new_function(
    name: usize,
    args: AstNode,
    tokens: &Vec<Token>,
    i: &mut usize,
    is_exported: bool,
    token_line_numbers: &Vec<u32>,
) -> AstNode {
    let function_body = Vec::new();

    // Check
    *i += 1;

    if &tokens[*i] != &Token::CloseScope {
        return AstNode::Error("Expected '(' for function args".to_string(),token_line_numbers[*i]);
    }

    *i += 1;

    // TODO - Get function body

    AstNode::Function(name.clone(), Box::new(args), function_body, is_exported)
}

fn create_var_node(
    attribute: Attribute,
    var_name: usize,
    var_value: AstNode,
    is_exported: bool,
    line_number: &u32,
) -> AstNode {
    match attribute {
        Attribute::Constant | Attribute::Comptime => {
            return AstNode::Const(var_name, Box::new(var_value), is_exported);
        }
        Attribute::Mutable => {
            return AstNode::VarDeclaration(var_name, Box::new(var_value), is_exported);
        }
        Attribute::Exported => {
            return AstNode::Error("Exported variable not yet supported".to_string(), *line_number);
        }
    }
}

pub fn find_var_declaration_index(ast: &Vec<AstNode>, var_name: &usize) -> usize {
    for (i, node) in ast.iter().enumerate().rev() {
        match node {
            AstNode::VarDeclaration(name, _, _) | AstNode::Const(name, _, _) => {
                if name == var_name {
                    return i;
                }
            }
            _ => {}
        }
    }

    0
}

fn skip_dead_code(tokens: &Vec<Token>, i: &mut usize) {
    // Check what type of dead code it is
    // If it is a variable declaration, skip to the end of the declaration

    *i += 1;
    match tokens.get(*i).unwrap_or(&Token::EOF) {
        Token::Assign | Token::AssignConstant | Token::Colon => {
            *i += 1;
        }
        Token::Newline => {
            *i += 1;
            return;
        }
        _ => {
            return;
        }
    }

    // Skip to end of variable declaration
    let mut open_parenthesis = 0;
    while let Some(token) = tokens.get(*i) {
        match token {
            Token::OpenParenthesis => {
                *i += 1;
                open_parenthesis += 1;
            }
            Token::CloseParenthesis => {
                *i += 1;

                if open_parenthesis < 1 {
                    red_ln!("Error: Closing parenthesis without opening parenthesis in dead variable code");
                    return;
                }
                open_parenthesis -= 1;
                if open_parenthesis == 0 {
                    break;
                }
            }
            Token::Newline => {
                *i += 1;
                if open_parenthesis < 1 {
                    return;
                }
            }
            Token::EOF => {
                break;
            }
            _ => {
                *i += 1;
            }
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
