// use crate::html_output::web_parser;
use crate::parsers;
// use crate::settings::get_html_config;
use crate::tokenizer;
use crate::Token;
// use regex::Regex;
use std::error::Error;
use std::fs;

pub fn test_build() -> Result<(), Box<dyn Error>> {
    // Read content from a test file
    println!("READING TEST FILE\n");
    let content = fs::read_to_string("test_output/src/index.bs")?;

    // Tokenize File
    println!("TOKENIZING FILE\n");
    let tokens: Vec<Token> = tokenizer::tokenize(&content, &"Test File".to_string());

    println!("TOKENS:");
    for token in &tokens {
        println!("{:?}", token);
    }
    println!("\n");

    // Create AST
    println!("CREATING AST\n");
    let ast: Vec<parsers::ast::AstNode> = parsers::build_ast::new_ast(tokens, 0).0;

    println!("AST:");
    println!("{:?}", ast);
    println!("\n");

    // println!("CREATING HTML OUTPUT\n");
    // let html_output = web_parser::parse(ast, get_html_config(), false);

    // // Print the HTML output
    // // Create a regex to match the content between the <main> and </main> tags
    // let re = Regex::new(r"(?s)<body>(.*?)</body>").unwrap();

    // // Extract the content between the <main> and </main> tags
    // let main_content = re
    //     .captures(&html_output)
    //     .and_then(|cap| cap.get(1))
    //     .map_or("", |m| m.as_str());

    // // Create a regex to match HTML tags
    // let re_tags = Regex::new(r"(</?\w+[^>]*>)").unwrap();

    // // Insert a newline before each HTML tag
    // let formatted_content = re_tags.replace_all(main_content, "\n$1");

    // // Print the formatted content
    // println!("\nHTML:\n{}", formatted_content);

    Ok(())
}
