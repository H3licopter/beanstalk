use crate::{ast::AstNode, Token};

pub fn new_ast(tokens: &Vec<Token>, start_index: usize) -> (Vec<AstNode>, usize) {
    let mut ast = Vec::new();
    let mut  i = start_index;

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

            Token::Newline => {
                // Do nothing
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


// Recursive function to parse scenes
fn new_scene(scene_head: &Vec<Token>, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    // parse scene head properties to determine how scene body is parsed

    while *i < tokens.len() {
        match &tokens[*i] {
            Token::SceneClose => {
                return AstNode::Scene(scene)
            }
            Token::SceneHead(_) => {
                let nested_scene = new_scene(scene_head, tokens, i);
                scene.push(nested_scene);
            }
            Token::Markdown(md_content) => {
                // Skip token if empty markdown
                if !md_content.is_empty() {
                    let html = markdown::to_html(md_content);
                    scene.push(AstNode::HTML(html));

                    // GFM Style markdown parsing?
                    // let parsed_markdown = markdown::to_html_with_options(md_content, &markdown::Options::gfm());
                    // match parsed_markdown {
                    //     Ok(html) => {
                    //         scene.push(AstNode::HTML(html));
                    //     }
                    //     Err(_) => {
                    //         scene.push(AstNode::Error("Error parsing markdown".to_string()));
                    //     }
                    // }
                }
            }

            // Scene head keywords and expressions

            _ => {
                scene.push(AstNode::Error("Invalid Token Used".to_string()));
            }
        }
        *i += 1;
    }

    AstNode::Scene(scene)
}



fn new_variable(name: &String, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    
    // If already initialised, return a reference ast node
    if is_reference(tokens, i, name) {
        return AstNode::Ref(name.clone());
    }

    *i += 1;
    if &tokens[*i] != &Token::Initialise {
        return AstNode::Error("Expected ':' for initialising. Variable does not yet exsist".to_string());
    }

    // Check type or infer type
    *i += 1;

    // Variable Properties
    let mut type_declaration = Token::Error("Can't figure out datatype".to_string());
    let mut var_is_const = true;
    let mut bracket_nesting = 0;

    match &tokens[*i] {
        
        // Infer type (CONSTANT VARIABLE)
        Token::Initialise => { 
            type_declaration = infer_datatype(&tokens[*i + 1]) 
        }
        // Infer type (MUTABLE VARIABLE)
        Token::Assign => { 
            var_is_const = false;
            type_declaration = infer_datatype(&tokens[*i + 1])
        }
        
        // Explicit Type Declarations
        Token::TypeInt => { type_declaration = Token::TypeInt; }
        Token::TypeFloat => { type_declaration = Token::TypeFloat; }
        Token::TypeString => { type_declaration = Token::TypeString; }
        Token::TypeCollection => { type_declaration = Token::TypeCollection; }
        Token::TypeObject => { type_declaration = Token::TypeObject; }
        Token::TypeRune => { type_declaration = Token::TypeRune; }
        Token::TypeDecimal => { type_declaration = Token::TypeDecimal;}
        Token::TypeBool => { type_declaration = Token::TypeBool; }
        Token::TypeScene => { type_declaration = Token::TypeScene; }
        
        // Function with implicit return type
        Token::OpenBracket => { return new_function(tokens, i) }

        _ => {
            return AstNode::Error("Expected either type definition or another ':' or '=' for initialising".to_string())
        }
    }

    // Get value of variable
    *i += 1;
    // Check if value is wrapped in brackets and move on until first value is found
    while &tokens[*i] == &Token::OpenBracket {
        bracket_nesting += 1;
        *i += 1;
    }

    match &tokens[*i] {
        // Check if value is a reference to another variable or function call
        Token::Variable(value) => {
            
            if is_reference(tokens, i, value) {
                
                // Check if is function call
                if &tokens[*i + 1] == &Token::OpenBracket {
                    
                    // Read function args
                    let mut args = Vec::new();
                    *i += 2;
                    while &tokens[*i] != &Token::CloseBracket {

                        // TO DO, CHECK IS VALID ARGUMENT
                        args.push(new_variable(value, tokens, i));
                        
                        *i += 1;
                        // Make sure a comma is serperating args
                        if &tokens[*i] == &Token::Comma {
                            *i += 1;
                        } else {
                            return AstNode::Error("Expected ',' to seperate function args".to_string());
                        }
                    }
                     
                    return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::FunctionCall(value.clone(), args)));
                }

                return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::Ref(value.clone())));
            }
        }

        Token::StringLiteral(value) => {
            return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::StringLiteral(value.clone())));
        }

        Token::RawStringLiteral(value) => {
            return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::RawStringLiteral(value.clone())));
        }

        Token::RuneLiteral(value) => {
            return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::RuneLiteral(value.clone())));
        }

        Token::IntLiteral(value) => {
            return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::IntLiteral(value.clone())));
        }

        Token::FloatLiteral(value) => {
            return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::FloatLiteral(value.clone())));
        }

        Token::DecLiteral(value) => {
            return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::DecLiteral(value.clone())));
        }

        Token::BoolLiteral(value) => {
            return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::BoolLiteral(value.clone())));
        }

        Token::CollectionOpen => {
            return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::Collection(vec![])));
        }

        Token::SceneOpen => {
            return AstNode::VarDeclaration(name.clone(), Box::new(AstNode::Scene(vec![])));
        }

        _ => {
            return AstNode::Error("Invalid Assignment for Variable, must be assigned wih a valid datatype".to_string());
        }
    }

    AstNode::Error("Invalid variable assignment".to_string())
}



// TO DO - SOME PLACEHOLDER CODE FOR FUNCTION DECLARATION
fn new_function(tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut function_name = String::new();
    let mut function_args = Vec::new();

    // Get function name
    match &tokens[*i] {
        Token::Variable(name) => {
            function_name = name.clone();
        }
        _ => {
            return AstNode::Error("Expected function name".to_string());
        }
    }

    // Get function args
    *i += 1;
    if &tokens[*i] != &Token::OpenBracket {
        return AstNode::Error("Expected '(' for function args".to_string());
    }

    *i += 1;
    while &tokens[*i] != &Token::CloseBracket {
        match &tokens[*i] {
            Token::Variable(name) => {
                function_args.push(AstNode::VarDeclaration(name.clone(), Box::new(AstNode::Ref("".to_string()))));
            }
            _ => {
                return AstNode::Error("Expected variable name for function args".to_string());
            }
        }
        *i += 1;
    }

    AstNode::Function(function_name, function_args)
}



// Check if variable name has been used earlier in the vec of tokens, if it has return true
fn is_reference(tokens: &Vec<Token>, i: &usize, name: &String) -> bool {
    for j in (0..=*i -1).rev() {
        if let Token::Variable(value) = &tokens[j] {
            if value == name {
                return true
            }
        }
    }
    false
}



fn infer_datatype(value: &Token) -> Token {
    match value {
        Token::StringLiteral(_) => { Token::TypeString }
        Token::RawStringLiteral(_) => { Token::TypeString }
        Token::RuneLiteral(_) => { Token::TypeRune }
        Token::IntLiteral(_) => { Token::TypeInt }
        Token::FloatLiteral(_) => { Token::TypeFloat }
        Token::BoolLiteral(_) => { Token::TypeBool }
        Token::DecLiteral(_) => { Token::TypeDecimal }
        Token::CollectionOpen => { Token::TypeCollection }
        Token::SceneOpen => { Token::TypeScene }
        _ => { Token::Error("Invalid Assignment for Variable, must be assigned wih a valid datatype".to_string()) }
    }
}