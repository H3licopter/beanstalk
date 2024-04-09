use crate::html_output::web_parser;
use crate::parsers;
use crate::parsers::ast::AstNode;
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

    struct OutputFile {
        source_code: String,
        output_file: String,
    }
    let mut source_code_to_parse: Vec<OutputFile> = Vec::new();

    // check to see if there is a config.bs file in this directory
    // if there is, read it and set the config settings
    // and check where the project entry points are
    let config = if entry_path.ends_with(".bs") {
        Ok("single file".to_string())
    } else {
        fs::read_to_string(format!("{}/config.bs", &entry_path))
    };

    match config {
        Ok(config_content) => {
            if config_content == "single file" {
                // Compile the induvidual file
                let default_output_dir = format!("{}/dist/index.html", &entry_path);
                fs::create_dir(&default_output_dir)?;
                let source_code = fs::read_to_string(&entry_path)?;
                compile(&source_code, &default_output_dir);
                return Ok(());
            }

            // Get config settings from config file
            let project_config = get_config_data(&config_content);

            // Read the content of the entry file from config data
            let source_code = fs::read_to_string(&project_config.main);
            match source_code {
                Ok(code) => {
                    source_code_to_parse.push(OutputFile {
                        source_code: code,
                        output_file: "dist/index.html".to_string(),
                    });
                }
                Err(_) => {
                    return Err("No entry file found".into());
                }
            }
        }
        Err(_) => {
            return Err("No config.bs file found in directory".into());
        }
    }



    Ok(())
}

fn compile(source_code: &str, output_dir: &str) -> Result<Vec<AstNode>, Box<dyn Error>> {
    let tokens: Vec<Token> = tokenizer::tokenize(&source_code);
    let ast = parsers::build_ast::new_ast(&tokens, 0).0;

    // If no output path, just return the AST
    if !output_dir.is_empty() {
        fs::write(
            output_dir,
            web_parser::parse(ast),
        )?;
        return Ok(Vec::new());
    }

    Ok(ast)
}

struct ProjectConfig {
    errors: Vec<String>,
    main: String,
}

fn get_config_data(content: &str) -> ProjectConfig {
    let config_ast = compile(content, "");
    let mut config = ProjectConfig {
        errors: Vec::new(),
        main: "src/pages/home.bs".to_string(),
    };

    match config_ast {
        Ok(ast) => {
            for node in ast {
                match node {
                    AstNode::Error(e) => {
                        config.errors.push(e);
                    }
                    AstNode::Project(data) => {
                        for node in data {
                            match node {
                                AstNode::Error(e) => {
                                    config.errors.push(e);
                                }
                                AstNode::Collection(values) => {
                                    for node in values {
                                        match node {
                                            AstNode::Error(e) => {
                                                config.errors.push(e);
                                            }
                                            _=> {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            config.errors.push(e.to_string());
        }
    }

    config
}