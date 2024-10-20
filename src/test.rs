use colour::{blue_ln, cyan_ln, green_ln, grey_ln, red_ln};
use colour::{blue_ln_bold, dark_grey_ln, dark_yellow_ln, green_ln_bold, yellow_ln_bold};
use regex::Regex;

use crate::html_output::web_parser;
use crate::parsers::ast_nodes::AstNode;
use crate::settings::get_html_config;
use crate::tokenizer;
use crate::Token;
use crate::{dev_server, parsers};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn test_build() -> Result<(), Box<dyn Error>> {
    // Read content from a test file
    yellow_ln_bold!("\nREADING TEST FILE\n");
    let path = PathBuf::from("test_output/src/#page.bs");
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let content = fs::read_to_string(&path)?;

    // Tokenize File
    yellow_ln_bold!("TOKENIZING FILE\n");
    let (tokens, token_line_numbers) = tokenizer::tokenize(&content, file_name, Vec::new());

    for token in &tokens {
        match token {
            Token::SceneHead | Token::SceneClose(_) => {
                blue_ln!("{:?}", token);
            }
            Token::P(_)
            | Token::HeadingStart(_)
            | Token::BulletPointStart(_)
            | Token::Em(_, _)
            | Token::Superscript(_) => {
                green_ln!("{:?}", token);
            }
            Token::Empty | Token::Newline => {
                grey_ln!("{:?}", token);
            }

            // Ignore whitespace in test output
            // Token::Whitespace => {}
            _ => {
                println!("{:?}", token);
            }
        }
    }
    println!("\n");

    // Create AST
    yellow_ln_bold!("CREATING AST\n");
    let ast: Vec<AstNode> = parsers::build_ast::new_ast(tokens, 0, &token_line_numbers).0;

    for node in &ast {
        match node {
            AstNode::Scene(_, _, _, _) => {
                print_scene(node, 0);
            }
            AstNode::Element(_) => {
                green_ln!("{:?}", node);
            }
            AstNode::Error(err, line) => {
                red_ln!("Error at line {}: {}", line, err);
            }
            AstNode::Literal(_) => {
                cyan_ln!("{:?}", node);
            }
            AstNode::Comment(_) => {
                grey_ln!("{:?}", node);
            }
            _ => {
                println!("{:?}", node);
            }
        }
    }

    yellow_ln_bold!("\nCREATING HTML OUTPUT\n");
    let (html_output, js_exports, css_exports, wat) = web_parser::parse(
        ast,
        get_html_config(),
        false,
        "test".to_string(),
        false,
        String::new(),
    );
    for export in js_exports {
        println!("JS EXPORTS:");
        println!("{:?}", export.module_path);
    }
    println!("CSS EXPORTS: {}", css_exports);

    println!("WAT: {}", wat);

    // Print the HTML output
    // Create a regex to match the content between the <main> and </main> tags
    let re = Regex::new(r"(?s)<body>(.*?)</body>").unwrap();

    // Extract the content between the <main> and </main> tags
    let main_content = re
        .captures(&html_output)
        .and_then(|cap| cap.get(1))
        .map_or("", |m| m.as_str());

    // Create a regex to match HTML tags
    let re_tags = Regex::new(r"(</?\w+[^>]*>)").unwrap();

    // Insert a newline before each HTML tag
    let formatted_content = re_tags.replace_all(main_content, "\n$1");

    // Print the formatted content
    println!("\n\n{}", formatted_content);

    dev_server::start_dev_server("test_output".to_string())?;

    Ok(())
}

fn print_scene(scene: &AstNode, scene_nesting_level: u32) {
    // Indent the scene by how nested it is
    let mut indentation = String::new();
    for _ in 0..scene_nesting_level {
        indentation.push_str("\t");
    }

    match scene {
        AstNode::Scene(nodes, tags, styles, actions) => {
            blue_ln_bold!("\n{}Scene Head: ", indentation);
            for tag in tags {
                dark_yellow_ln!("{}  {:?}", indentation, tag);
            }
            for style in styles {
                cyan_ln!("{}  {:?}", indentation, style);
            }
            for action in actions {
                dark_yellow_ln!("{}  {:?}", indentation, action);
            }

            blue_ln_bold!("{}Scene Body:", indentation);

            for scene_node in nodes {
                match scene_node {
                    AstNode::Scene(_, _, _, _) => {
                        print_scene(scene_node, scene_nesting_level + 1);
                    }
                    AstNode::Element(token) => match token {
                        Token::P(_) => {
                            green_ln!("{}  {:?}", indentation, scene_node);
                        }
                        _ => {
                            grey_ln!("{}  {:?}", indentation, scene_node);
                        }
                    },
                    AstNode::Heading(_)
                    | AstNode::BulletPoint(_)
                    | AstNode::Em(_, _)
                    | AstNode::Superscript(_) => {
                        green_ln_bold!("{}  {:?}", indentation, scene_node);
                    }
                    AstNode::RuntimeExpression(_, _) => {
                        dark_yellow_ln!("{}  {:?}", indentation, scene_node);
                    }
                    AstNode::Error(err, line) => {
                        red_ln!("{}  Error at line {}: {}", indentation, line, err);
                    }
                    AstNode::Literal(_) => {
                        cyan_ln!("{}  {:?}", indentation, scene_node);
                    }
                    AstNode::Space | AstNode::Comment(_) => {
                        dark_grey_ln!("{}  {:?}", indentation, scene_node);
                    }
                    _ => {
                        println!("{}  {:?}", indentation, scene_node);
                    }
                }
            }
        }
        _ => {}
    }
    println!("\n");
}
