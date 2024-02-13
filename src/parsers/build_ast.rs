use crate::{ast::AstNode, Token};

pub fn new_ast(tokens: &Vec<Token>, start_index: usize) -> (Vec<AstNode>, usize) {
    let mut ast = Vec::new();
    let mut  i = start_index;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Comment(value) => {
                ast.push(AstNode::Comment(value.clone()));
            }

            // New Function or Variable declaration
            Token::Variable(name) => {
                new_variable(name, tokens, &mut i);
            }

            // Or stuff that hasn't been implemented yet
            _ => {
            
            }
        }

        i += 1;
        
    }

    (ast, i)
}

fn new_variable(name: &String, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    // If already initialised, return a reference ast node
    if tokens
        .iter()
        .rev()
        .skip(tokens.len() - *i - 1)
        .any(|t| t == &Token::Variable(name.clone())) {
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
    
    // Check if value is wrapped in brackets and move on until first value is found
    while &tokens[*i + 1] == &Token::OpenBracket {
        bracket_nesting += 1;
        *i += 1;
    }

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
    match &tokens[*i] {
        Token::StringLiteral(value) => {
            AstNode::VarDeclaration(name.clone(), Box::new(AstNode::StringLiteral(value.clone())));
        }
        _ => {
            AstNode::Error("Invalid Assignment for Variable, must be assigned wih a valid datatype".to_string());
        }
    }

    AstNode::Error("Failed to create a valid variable".to_string())
}

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