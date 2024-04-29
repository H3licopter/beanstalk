use crate::html_output::web_parser;
use crate::parsers;
use crate::parsers::ast::AstNode;
use crate::settings::{get_default_config, get_html_config, Config};
use crate::tokenizer;
use crate::tokens::Token;
use std::error::Error;
use std::fs;

struct OutputFile {
    source_code: String,
    output_dir: String,
    file_name: String,
}

#[allow(unused_variables)]
pub fn build(mut entry_path: String) -> Result<(), Box<dyn Error>> {
    // If entry_path is empty, use the current directory
    if entry_path == "test" {
        entry_path = "test_output/src/pages/home.bs".to_string();
    } else {
        let current_dir = std::env::current_dir()?;
        entry_path = format!(
            "{}/{}",
            current_dir.to_string_lossy().into_owned(),
            entry_path
        );
    }

    // Read content from a test file
    println!("Reading from: {}", &entry_path);

    let mut source_code_to_parse: Vec<OutputFile> = Vec::new();

    // check to see if there is a config.bs file in this directory
    // if there is, read it and set the config settings
    // and check where the project entry points are
    enum CompileType {
        SingleFile(String, String), // File Name, Source Code
        MultiFile(String),          // Config file content
        Error(String),
    }

    let config = if entry_path.ends_with(".bs") {
        let source_code = fs::read_to_string(&entry_path);
        let og_file_name = entry_path.split("/").last().unwrap();
        let new_file_name = og_file_name.split(".").next().unwrap();

        match source_code {
            Ok(content) => {
                let file_name = format!(
                    "{}.html",
                    if new_file_name == "home" {
                        "index"
                    } else {
                        new_file_name
                    }
                );
                CompileType::SingleFile(file_name, content)
            }
            Err(_) => CompileType::Error("No file found".to_string()),
        }
    } else {
        let source_code = fs::read_to_string(format!("{}/config.bs", &entry_path));
        match source_code {
            Ok(content) => CompileType::MultiFile(content),
            Err(_) => CompileType::Error("No config.bs file found in directory".to_string()),
        }
    };

    match config {
        CompileType::SingleFile(name, code) => {
            // Compile the induvidual file
            let entry_file_dir = entry_path.split("/").collect::<Vec<&str>>();
            let default_output_dir = format!(
                "{}/dist",
                entry_file_dir[0..entry_file_dir.len() - 3].join("/")
            );

            source_code_to_parse.push(OutputFile {
                source_code: code,
                output_dir: format!("{}/", &default_output_dir),
                file_name: name,
            });
        }

        CompileType::Error(e) => {
            return Err(e.into());
        }

        CompileType::MultiFile(source_code) => {
            // Get config settings from config file
            let project_config = get_config_data(&source_code)?;

            // TO DO, READ WHOLE PROJECT FROM CONFIG ENTRY POINT AND ADD EVERYTHING TO COMPILE LIST

            let main_code = fs::read_to_string(project_config.main);

            match main_code {
                Ok(content) => {
                    source_code_to_parse.push(OutputFile {
                        source_code: content,
                        output_dir: format!("{}/", project_config.output_folder),
                        file_name: "index.html".to_string(),
                    });
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }

    // Compile all output files
    for file in source_code_to_parse {
        compile(file)?;
    }

    Ok(())
}

fn compile(output: OutputFile) -> Result<Vec<AstNode>, Box<dyn Error>> {
    let tokens: Vec<Token> = tokenizer::tokenize(&output.source_code, &output.file_name);
    let ast = parsers::build_ast::new_ast(tokens, 0).0;

    // If no output path, just return the AST
    if !output.output_dir.is_empty() {
        let output_path = format!("{}{}", output.output_dir, output.file_name);
        println!("Compiling: {}", output_path);
        fs::write(output_path, web_parser::parse(ast, get_html_config()))?;
        return Ok(Vec::new());
    }

    Ok(ast)
}

fn get_config_data(config_source_code: &str) -> Result<Config, Box<dyn Error>> {
    let config_ast = compile(OutputFile {
        source_code: config_source_code.to_string(),
        output_dir: String::new(),
        file_name: String::new(),
    });
    let config = get_default_config();

    match config_ast {
        Ok(ast) => {
            for node in ast {
                match node {
                    AstNode::Error(e) => {
                        return Err(e.into());
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    Ok(config)
}
