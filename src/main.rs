use std::time::Instant;
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

pub mod bs_types;
mod build;
mod create_new_project;
pub mod dev_server;
mod settings;
mod test;
mod tokenizer;
mod tokens;
mod parsers {
    pub mod ast;
    pub mod build_ast;
    mod create_scene_node;
    pub mod parse_expression;
    pub mod styles;
    pub mod util;
}
mod html_output {
    mod generate_html;
    mod markdown_parser;
    pub mod web_parser;
}
pub use tokens::Token;
enum Command {
    NewHTMLProject(String),
    Build(String),
    Test,
    Dev(String), // Runs local dev server
}

fn main() {
    let command = collect_user_input();
    match command {
        Command::NewHTMLProject(path) => {
            let args = prompt_user_for_input("Project name: ".to_string());
            let name_args = args.get(0);

            let project_name = match name_args {
                Some(name) => {
                    if name.is_empty() {
                        "test_output".to_string()
                    } else {
                        name.to_string()
                    }
                }
                None => "test_output".to_string(),
            };

            match create_new_project::create_project(path, &project_name) {
                Ok(_) => {
                    println!("Creating new HTML project...");
                    main();
                }
                Err(e) => {
                    println!("Error creating project: {:?}", e);
                }
            }
        }
        Command::Build(path) => {
            println!("Building project...");
            let start = Instant::now();
            match build::build(path) {
                Ok(_) => {
                    let duration = start.elapsed();
                    println!("Project built in: {:?}", duration);
                    main();
                }
                Err(e) => {
                    println!("Error building project: {:?}", e);
                }
            }
        }
        Command::Test => {
            println!("Testing...");
            match test::test_build() {
                Ok(_) => {
                    main();
                }
                Err(e) => {
                    println!("Error while testing: {:?}", e);
                }
            }
        }
        Command::Dev(path) => {
            println!("Starting dev server...");
            dev_server::launch_server(path);
        }
    }
}

fn collect_user_input() -> Command {
    let args = prompt_user_for_input("Enter compiler command: ".to_string());

    match args.get(0).map(String::as_str) {
        Some("new") => {
            // Check type of project
            match args.get(1).map(String::as_str) {
                Some("html") => {
                    let dir = &prompt_user_for_input("Enter project path: ".to_string())[0];
                    if check_if_valid_directory_path(&dir) {
                        return Command::NewHTMLProject(dir.to_string());
                    }
                }
                _ => {
                    println!("Invalid project type");
                }
            }
        }
        Some("build") => {
            match args.get(1).map(String::as_str) {
                Some(string) => {
                    // Check if path is valid, if not, throw error
                    return Command::Build(string.to_string());
                }
                _ => {
                    // Return current working directory path
                    return Command::Build("".to_string());
                }
            }
        }
        Some("test") => {
            return Command::Test;
        }
        Some("dev") => {
            match args.get(1) {
                Some(path) => {
                    if path.is_empty() {
                        return Command::Dev("test_output".to_string());
                    } else {
                        return Command::Dev(path.to_string());
                    }
                }
                None => return Command::Dev("test_output".to_string()),
            };
        }
        _ => {
            return Command::Test;
        }
    }

    // Help is default case if no command is entered
    // Display possible commands
    // println!("Possible commands:");
    // println!("new html");
    // println!("build");
    // println!("test");
    // println!("build test");

    collect_user_input()
}

fn check_if_valid_directory_path(path: &str) -> bool {
    let path = Path::new(path);

    // Check if the path exists
    if !path.exists() {
        println!("Path does not exist: {}", path.display());
        return false;
    }

    // Check if the path is a directory
    if !path.is_dir() {
        println!("Path is not a directory: {}", path.display());
        return false;
    }

    // Check if the directory is writable
    let metadata = fs::metadata(path).expect("Unable to read metadata");
    if metadata.permissions().readonly() {
        println!("Directory is not writable: {}", path.display());
        return false;
    }

    true
}

fn prompt_user_for_input(msg: String) -> Vec<String> {
    let mut input = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
    io::stdin().read_line(&mut input).unwrap();
    let args: Vec<String> = input.split_whitespace().map(String::from).collect();

    args
}
