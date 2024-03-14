use std::error::Error;
use std::fs;
use crate::tokenizer;
use crate::tokens::Token;
use crate::parsers;
use crate::html_output::web_parser;

pub fn test_build() -> Result<(), Box<dyn Error>> {
    
    // Read content from a test file
    println!("READNING TEST FILE\n");
    let content = fs::read_to_string("src/test.bs")?;
    
    // Tokenize File
    println!("TOKENIZING FILE\n");
    let tokens: Vec<Token> = tokenizer::tokenize(&content);
    
    println!("TOKENS:");
    for token in &tokens {
        println!("{:?}", token);
    }
    println!("\n");

    // Create AST
    println!("CREATING AST\n");
    let ast: Vec<crate::ast::AstNode> = parsers::build_ast::new_ast(&tokens, 0).0;
    
    println!("AST:");
    println!("{:?}", ast);
    println!("\n");

    println!("CREATING HTML OUTPUT\n");
    let html_output = web_parser::parse(ast);
    println!("HTML:\n");
    println!("{:?}", html_output.split("<body>").collect::<Vec<&str>>()[1]);

    Ok(())
}