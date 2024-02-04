use std::error::Error;
use std::fs;
use crate::tokenizer;
use crate::tokens::Token;
use crate::parsers;

pub fn test_build() -> Result<(), Box<dyn Error>> {
    // Read content from a test file
    println!("READNING TEST FILE");
    let content = fs::read_to_string("../../test.bs")?;
    
    // Tokenize File
    println!("TOKENIZING FILE");
    let tokens: Vec<Token> = tokenizer::tokenize(&content);
    println!("Tokens: {:?}", tokens);

    // Create AST
    println!("CREATING AST");
    let ast = parsers::build_ast::new_ast(&tokens, 0).0;
    println!("AST: {:?}", ast);

    Ok(())
}