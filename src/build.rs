use std::error::Error;
use std::{fs, io::prelude::Write};
use crate::tokenizer;
use crate::tokens::Token;
use crate::parsers;
use crate::web_frontend::web_parser;

#[allow(unused_variables)]
pub fn build(mut entry_path: String) -> Result<(), Box<dyn Error>> {
    // If entry_path is empty, use the current directory
    if entry_path == "test" {
        entry_path = "src/test.bs".to_string();
    }
    if entry_path.is_empty() {
        let current_dir = std::env::current_dir()?;
        entry_path = current_dir.to_string_lossy().into_owned();
    }

    // Read content from a test file
    println!("Reading from: {}", entry_path);
    let content = fs::read_to_string(entry_path)?;

    // Tokenize File
    let tokens: Vec<Token> = tokenizer::tokenize(&content);

    // Check for compiler directives or config settings

    // Create AST
    let ast = parsers::build_ast::new_ast(&tokens, 0).0;    

    // Parse Tokens into code output
    let html_output = web_parser::parse(ast);

    // Write HTML output to file
    let output_path = "../test_project_output/html_project_template/dist/".to_string();
    let mut file = fs::File::create(output_path + "index.html")?;
    file.write_all(html_output.as_bytes())?;

    println!("Project built!");

    Ok(())
}