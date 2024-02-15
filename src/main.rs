use std::{io::{self, Write}, fs, path::Path};

mod tokenizer;
mod tokens;
mod settings;
mod ast;
mod build;
mod test;
mod create_new_project;
mod parsers {
    pub mod build_ast;
}
mod web_frontend {
    pub mod web_parser;
    mod generate_html;
}
pub use tokens::Token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = collect_user_input();
    match command {
        Command::NewHTMLProject(path) => {
            create_new_project::create_project(path)?;
            println!("Creating new HTML project...");
        },
        Command::Build(path) => {
            build::build(path)?;
            println!("Building project...");
        },
        Command::Test => {
            println!("Testing...");
            test::test_build()?;
        }
    }

    Ok(())
}
enum Command {
    NewHTMLProject(String),
    Build(String),
    Test,
}

fn collect_user_input() -> Command {
    let mut input = String::new();
    print!("Enter compiler command: ");
    io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
    io::stdin().read_line(&mut input).unwrap();
    let args: Vec<String> = input
        .split_whitespace()
        .map(String::from)
        .collect();

    match args.get(0).map(String::as_str) {
        Some("new") => {
            // Check type of project
            match args.get(1).map(String::as_str) {
                Some("html") => {
                    match args.get(2).map(String::as_str) {
                        Some(string) => {
                            // Check if path is valid, if not, throw error
                            if check_if_valid_directory_path(string) {
                                return Command::NewHTMLProject(string.to_string());
                            }
                        },
                        _=> {
                            return Command::NewHTMLProject("".to_string());
                        }
                    }
                },
                _ => {
                    println!("Invalid project type");
                }
            }
        },
        Some("build") => {
            match args.get(1).map(String::as_str) {
                Some(string) => {
                    // Check if path is valid, if not, throw error
                    return Command::Build(string.to_string());
                },
                _=> {
                    // Return current working directory path
                    return Command::Build("".to_string());
                }
            }
        },
        Some("test") => {
            return Command::Test;
        },
        _ => {
            println!("Building test project....");
            build::build("test".to_string());
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