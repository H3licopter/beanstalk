use std::error::Error;
use std::fs;
use crate::tokenizer;
use crate::tokens::Token;
use crate::parsers;
use crate::web_frontend::web_parser;

pub fn test_build() -> Result<(), Box<dyn Error>> {
    
    // Read content from a test file
    println!("READNING TEST FILE");
    let content = fs::read_to_string("src/test.bs")?;
    
    // Tokenize File
    println!("TOKENIZING FILE:");
    let tokens: Vec<Token> = tokenizer::tokenize(&content);
    println!("Tokens: {:?}", tokens);

    println!("\n");
    println!("\n");


    // Create AST
    println!("CREATING AST");
    let ast = parsers::build_ast::new_ast(&tokens, 0).0;
    println!("AST: {:?}", ast);

    println!("\n");
    println!("\n");


    println!("CREATING HTML OUTPUT");
    let html_output = web_parser::parse(ast);
    println!("HTML: {:?}", html_output.split("<body>").collect::<Vec<&str>>()[1]);

    Ok(())
}