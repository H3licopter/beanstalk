use colour::{dark_cyan_ln, dark_yellow_ln, print_bold, print_ln_bold, red_ln};

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
pub fn build(mut entry_path: String, release_build: bool) -> Result<(), Box<dyn Error>> {
    // If entry_path is "test", use the compiler test directory
    if entry_path == "test" {
        entry_path = "test_output/src/home.bs".to_string();
    }

    if entry_path == "" {
        entry_path = match std::env::current_dir() {
            Ok(dir) => dir.to_str().unwrap().to_owned(),
            Err(e) => {
                println!("Error getting current directory: {:?}", e);
                return Err(e.into());
            }
        };
    }

    // Read content from a test file
    print_ln_bold!("Project Directory: ");
    dark_yellow_ln!("{}", &entry_path);

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
            dark_cyan_ln!("Reading Config File ...");
            // Get config settings from config file
            // let project_config = get_config_data(&source_code)?;
            // Just get the default config for now until can parse the config settings
            let project_config = get_default_config();
            let src_dir: fs::ReadDir =
                match fs::read_dir(&format!("{}/{}", entry_path, project_config.src)) {
                    Ok(dir) => dir,
                    Err(e) => {
                        red_ln!("Error reading directory: {:?}", e);
                        return Err(e.into());
                    }
                };
            add_bs_files_to_parse(&mut source_code_to_parse, src_dir)?;
        }
    }

    // Compile all output files
    for file in source_code_to_parse {
        compile(file, release_build)?;
    }

    Ok(())
}

// Look for every subdirectory inside of dir and add all .bs files to the source_code_to_parse
fn add_bs_files_to_parse(
    source_code_to_parse: &mut Vec<OutputFile>,
    dir: fs::ReadDir,
) -> Result<(), Box<dyn Error>> {
    for file in dir {
        match file {
            Ok(file) => {
                let file = file.path();
                if file.extension() == Some("bs".as_ref()) {
                    let file_name = match file.file_stem() {
                        Some(name) => name.to_string_lossy(),
                        None => continue,
                    };

                    let file_str = file.parent().unwrap().to_str().unwrap();

                    let mut output_dir = String::new();

                    if let Some(src_pos) = file_str.find("/src") {
                        // Split the string slice at the position of "/src/"
                        let (root_dir, subfolders) = file_str.split_at(src_pos);

                        // Skip the "/src" part in subfolders
                        if subfolders.len() < 4 {
                            eprintln!("'src' not found in the path");
                            continue;
                        }

                        let subfolders = &subfolders[4..]; // 4 is the length of "/src"

                        // Create the output directory string
                        output_dir = format!("{}/dist{}", root_dir, subfolders);
                    } else {
                        eprintln!("'src' not found in the path");
                    }

                    let output_file = format!("{file_name}.html");
                    let file_dir = file.to_str().unwrap();

                    let code = match fs::read_to_string(&file_dir) {
                        Ok(content) => content,
                        Err(e) => {
                            red_ln!("Error reading file while adding bs files to parse: {:?}", e);
                            continue;
                        }
                    };

                    source_code_to_parse.push(OutputFile {
                        source_code: code,
                        output_dir: format!("{}/", output_dir),
                        file_name: output_file,
                    });
                }

                // HANDLE USING JS / HTML / CSS / MARKDOWN FILES MIXED INTO THE PROJECT IN THE FUTURE

                // If directory, recursively call add_bs_files_to_parse
                if file.is_dir() {
                    let new_dir = match fs::read_dir(&file) {
                        Ok(new_path) => new_path,
                        Err(e) => {
                            red_ln!(
                                "Error reading directory while adding bs files to parse: {:?}",
                                e
                            );
                            continue;
                        }
                    };

                    add_bs_files_to_parse(source_code_to_parse, new_dir)?
                }
            }

            Err(e) => {
                red_ln!("Error reading file while adding bs files to parse: {:?}", e);
            }
        }
    }

    Ok(())
}

fn compile(output: OutputFile, release_build: bool) -> Result<Vec<AstNode>, Box<dyn Error>> {
    print_bold!("Compiling: ");
    dark_yellow_ln!("{}", output.file_name);

    let tokens: Vec<Token> = tokenizer::tokenize(&output.source_code, &output.file_name);
    let ast = parsers::build_ast::new_ast(tokens, 0).0;

    // If the output directory does not exist, create it
    if !fs::metadata(&output.output_dir).is_ok() {
        fs::create_dir_all(&output.output_dir)?;
    }

    let output_path = format!("{}{}", output.output_dir, output.file_name);
    fs::write(output_path, web_parser::parse(ast, get_html_config(), release_build))?;

    Ok(Vec::new())
}

#[allow(dead_code)]
fn get_config_data(config_source_code: &str) -> Result<Config, Box<dyn Error>> {
    let config_ast = compile(OutputFile {
        source_code: config_source_code.to_string(),
        output_dir: String::new(),
        file_name: String::new(),
    }, false);
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
