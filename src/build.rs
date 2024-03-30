use crate::html_output::web_parser;
use crate::parsers;
use crate::tokenizer;
use crate::tokens::Token;
use std::error::Error;
use std::fs;

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
    println!("Reading from: {}", &entry_path);
    let content = fs::read_to_string(&entry_path)?;

    // Tokenize File
    let tokens: Vec<Token> = tokenizer::tokenize(&content);

    // Check for compiler directives or config settings

    // Create AST
    let ast = parsers::build_ast::new_ast(&tokens, 0).0;

    // Parse Tokens into code output
    // 0 = HTML, 1 = JS, 2 = CSS
    let web_parser_output = web_parser::parse(ast);

    // Write HTML output to file
    // TEMPORARY TESTING ENTRY PATH
    entry_path = "../html_project_template/dist/".to_string();
    fs::write(entry_path.clone() + "index.html", web_parser_output.0)?;
    fs::write(entry_path + "bs.js", web_parser_output.1)?;

    Ok(())
}
