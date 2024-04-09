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
        entry_path = "test_output/src/pages/home.bs".to_string();
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
    enum CompileType {
        SingleFile(String, String), // File Name, Source Code
        MultiFile(String), // Config file content
        Error(String)
    }
    let config = if entry_path.ends_with(".bs") {
        let source_code = fs::read_to_string(&entry_path);
        let og_file_name = entry_path.split("/").last().unwrap();
        let new_file_name = og_file_name.split(".").next().unwrap();

        match source_code {
            Ok(content) => CompileType::SingleFile(new_file_name.to_string(), content),
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
        CompileType::SingleFile(file_name, code) => {
            // Compile the induvidual file
            let entry_file_dir = entry_path.split("/").collect::<Vec<&str>>();
            let default_output_dir = format!("{}/dist", entry_file_dir[0..entry_file_dir.len()-3].join("/"));

            source_code_to_parse.push(OutputFile {
                source_code: code,
                output_file: format!("{}/{}.html", &default_output_dir, file_name)
            });
        }

        CompileType::Error(e) => {
            return Err(e.into());
        }

        CompileType::MultiFile(source_code) => {
            // Get config settings from config file
            let project_config = get_config_data(&source_code);

            // TO DO, READ WHOLE PROJECT FROM CONFIG ENTRY POINT AND ADD EVERYTHING TO COMPILE LIST
            
            let main_code = fs::read_to_string(project_config.main);
            
            match main_code {
                Ok(content) => {
                    source_code_to_parse.push(OutputFile {
                        source_code: content,
                        output_file: format!("{}/index.html", project_config.output)
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
        compile(&file.source_code, &file.output_file)?;
    }

    Ok(())
}

fn compile(source_code: &str, output_dir: &str) -> Result<Vec<AstNode>, Box<dyn Error>> {
    println!("Compiling: {}", &output_dir);
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
    output: String,
}
fn get_config_data(config_source_code: &str) -> ProjectConfig {
    let config_ast = compile(config_source_code, "");
    let mut config = ProjectConfig {
        errors: Vec::new(),
        main: "src/pages/home.bs".to_string(),
        output: "dist/".to_string(),
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