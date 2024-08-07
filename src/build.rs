use colour::{blue_ln, dark_cyan_ln, dark_yellow_ln, print_bold, print_ln_bold, red_ln};

use crate::html_output::web_parser;
use crate::parsers;
use crate::parsers::ast::AstNode;
use crate::settings::{get_default_config, get_html_config, Config};
use crate::tokenizer;
use crate::tokens::Token;
use std::error::Error;
use std::path::PathBuf;
use std::fs;

pub struct OutputFile {
    source_code: String,
    file: PathBuf,
}

#[allow(unused_variables)]
pub fn build(entry_path: String, release_build: bool) -> Result<(), Box<dyn Error>> {
    // Change default output directory to dev if release_build is true
    let project_config = get_default_config();
    let output_dir_folder = if release_build {
        PathBuf::from(&project_config.release_folder)
    } else {
        PathBuf::from(&project_config.dev_folder)
    };

    // Create a new PathBuf from the entry_path
    let entry_dir ;
    // If entry_path is "test", use the compiler test directory
    if entry_path == "test" {
        entry_dir = match std::env::current_dir() {
            Ok(dir) => {
                dir.join("test_output/src/index.bs")
            },
            Err(e) => {
                println!("Error getting current directory: {:?}", e);
                return Err(e.into());
            }
        };
    } else if entry_path == "" {
        entry_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("Error getting current directory: {:?}", e);
                return Err(e.into());
            }
        };
    } else {
        // turn whitespace in file name to dashes
        entry_dir = PathBuf::from(entry_path.replace(|c: char| c.is_whitespace(), "-"));
    }

    // Read content from a test file
    print_ln_bold!("Project Directory: ");
    dark_yellow_ln!("{:?}", &entry_dir);

    let mut source_code_to_parse: Vec<OutputFile> = Vec::new();

    // check to see if there is a config.bs file in this directory
    // if there is, read it and set the config settings
    // and check where the project entry points are
    enum CompileType {
        SingleFile(PathBuf, String), // File Name, Source Code
        MultiFile(PathBuf, String),          // Config file content
        Error(String),
    }

    let config = if entry_dir.extension() == Some("bs".as_ref()) {
        let source_code = fs::read_to_string(&entry_dir);
        match source_code {
            Ok(content) => {
                CompileType::SingleFile(entry_dir.with_extension("html"), content)
            }
            Err(_) => CompileType::Error("No file found".to_string()),
        }
    } else {
        let source_code = fs::read_to_string(entry_dir.join("config.bs"));
        match source_code {
            Ok(content) => CompileType::MultiFile(entry_dir.clone(), content),
            Err(_) => CompileType::Error("No config.bs file found in directory".to_string()),
        }
    };

    match config {
        CompileType::SingleFile(file_path, code) => {
            source_code_to_parse.push(OutputFile {
                source_code: code,
                file: file_path,
            });
        }

        CompileType::Error(e) => {
            return Err(e.into());
        }

        CompileType::MultiFile(entry_dir, source_code) => {
            dark_cyan_ln!("Reading Config File ...");
            // Get config settings from config file
            // let project_config = get_config_data(&source_code)?;
            // Just get the default config for now until can parse the config settings
            let src_dir: fs::ReadDir =
                match fs::read_dir(entry_dir.join(&project_config.src)) {
                    Ok(dir) => dir,
                    Err(e) => {
                        red_ln!("Error reading directory: {:?}", e);
                        return Err(e.into());
                    }
                };
            match add_bs_files_to_parse(&mut source_code_to_parse, src_dir, entry_dir.join(output_dir_folder)) {
                Ok(_) => {}
                Err(e) => {
                    red_ln!("Error adding bs files to parse: {:?}", e);
                }
            }
        }
    }

    // Compile all output files
    for file in &source_code_to_parse {
        match compile(file, release_build, &project_config) {
            Ok(_) => {}
            Err(e) => {
                red_ln!("Error compiling file: {:?}", e);
            }
        }
    }

    // Any HTML files in the output dir not on the list of files to compile should be deleted if this is a release build
    if release_build && entry_dir.is_dir() {
        let output_dir = PathBuf::from(&entry_dir).join(&project_config.release_folder);
        let dir_files = match fs::read_dir(&output_dir) {
            Ok(dir) => dir,
            Err(e) => {
                red_ln!("Error reading output_dir directory: {:?}", &output_dir);
                return Err(e.into());
            }
        };

        for file in dir_files {
            let file = match file {
                Ok(f) => f,
                Err(e) => {
                    red_ln!("Error reading file: {:?}", e);
                    continue;
                }
            };
            let file_path = file.path();
            if file_path.extension() == Some("html".as_ref()) {
                if !source_code_to_parse.iter().any(|f| f.file.file_stem() == file_path.file_stem()) {
                    match fs::remove_file(&file_path) {
                        Ok(_) => {
                            blue_ln!("Deleted unused file: {:?}", file_path);
                        }
                        Err(e) => {
                            red_ln!("Error deleting file: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

// Look for every subdirectory inside of dir and add all .bs files to the source_code_to_parse
pub fn add_bs_files_to_parse(
    source_code_to_parse: &mut Vec<OutputFile>,
    dir: fs::ReadDir,
    output_file_dir: PathBuf,
) -> Result<(), Box<dyn Error>> {
    for file in dir {
        match file {
            Ok(f) => {
                let file_path = &f.path();
                if file_path.extension() == Some("bs".as_ref()) {
                    let code = match fs::read_to_string(file_path) {
                        Ok(content) => content,
                        Err(e) => {
                            red_ln!("Error reading file while adding bs files to parse: {:?}", e);
                            continue;
                        }
                    };

                    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
                    
                    source_code_to_parse.push(OutputFile {
                        source_code: code,
                        file: output_file_dir.join(file_name).with_extension("html"),
                    });

                // If directory, recursively call add_bs_files_to_parse
                } else if file_path.is_dir() {
                    let new_dir = match fs::read_dir(file_path) {
                        Ok(new_path) => new_path,
                        Err(e) => {
                            red_ln!(
                                "Error reading directory while adding bs files to parse: {:?}",
                                e
                            );
                            continue;
                        }
                    };

                    let new_output_dir = output_file_dir.join(file_path.file_stem().unwrap());
                    match add_bs_files_to_parse(source_code_to_parse, new_dir, new_output_dir) {
                        Ok(_) => {}
                        Err(e) => {
                            red_ln!("Error adding bs files to parse: {:?}", e);
                        }
                    }
                
                // HANDLE USING JS / HTML / CSS MIXED INTO THE PROJECT
                } else {
                    match file_path.extension() {
                        Some(ext) => {
                            // TEMPORARY: JUST PUT THEM DIRECTLY INTO THE OUTPUT DIRECTORY
                            if ext == "js" || ext == "html" || ext == "css" {
                                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                                source_code_to_parse.push(OutputFile {
                                    source_code: String::new(),
                                    file: output_file_dir.join(file_name),
                                });
                            }
                        }
                        None => {}
                    }
                }


            }

            Err(e) => {
                red_ln!("Error reading file while adding bs files to parse: {:?}", e);
            }
        }
    }

    Ok(())
}

fn compile(
    output: &OutputFile,
    release_build: bool,
    config: &Config,
) -> Result<Vec<AstNode>, Box<dyn Error>> {
    print_bold!("Compiling: ");
    let file_name = output.file.file_name().unwrap().to_str().unwrap();
    dark_yellow_ln!("{:?}", file_name);

    let tokens: Vec<Token> = tokenizer::tokenize(&output.source_code, file_name);
    let ast = parsers::build_ast::new_ast(tokens, 0).0;

    // If the output directory does not exist, create it
    let parent_dir = output.file.parent().unwrap();
    if !fs::metadata(parent_dir).is_ok() {
        match fs::create_dir_all(parent_dir) {
            Ok(_) => {}
            Err(e) => {
                red_ln!("Error creating directory: {:?}", e);
            }
        }
    }

    let output_path = &output.file;

    // TO BE REPLACED WITH LOADING CONFIG.BS FILE (When all config tokens are in tokenizer)
    let mut html_config = get_html_config();

    // For each subdirectory from the dist or dev folder of the output_dir, add a ../ to the image_folder_url
    let output_dir_name = if release_build {
        &config.release_folder
    } else {
        &config.dev_folder
    };

    for ancestor in output.file.ancestors() {
        if ancestor.file_name() == Some(output_dir_name.as_ref()) {
            break;
        }
        html_config.page_dist_url.push_str("../");
    }

    match fs::write(
        output_path,
        web_parser::parse(ast, html_config, release_build),
    ) {
        Ok(_) => {}
        Err(e) => {
            red_ln!("Error writing file: {:?}", e);
        }
    }

    Ok(Vec::new())
}

// fn get_config_data(config_source_code: &str) -> Result<Config, Box<dyn Error>> {
//     let config_ast = compile(
//         OutputFile {
//             source_code: config_source_code.to_string(),
//             output_dir: String::new(),
//             file_name: String::new(),
//         },
//         false,
//     );
//     let config = get_default_config();

//     match config_ast {
//         Ok(ast) => {
//             for node in ast {
//                 match node {
//                     AstNode::Error(e) => {
//                         return Err(e.into());
//                     }
//                     _ => {}
//                 }
//             }
//         }
//         Err(e) => {
//             return Err(e.into());
//         }
//     }

//     Ok(config)
// }
