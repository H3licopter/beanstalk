use crate::{settings::get_meta_config, ast::AstNode};
use super::generate_html::create_html_boilerplate;
use crate::parsers::markdown_parser::parse_markdown_to_html;

// Parse ast into valid JS, HTML and CSS
pub fn parse(ast: Vec<AstNode>) -> String {
    let js = String::new();
    let _wasm = String::new();
    let mut html = String::new();
    let css = String::new();
    let mut page_title = String::new();

    let config = get_meta_config();

    // Parse HTML
    for node in ast {
        match node {
            AstNode::Scene(scene) => {
                let scene_html = parse_scene(scene);
                html.push_str(scene_html.as_str());
            }
            AstNode::Title(value) => {
                page_title = value;

                if config.auto_site_title {
                    page_title += &(" | ".to_owned() + &config.site_title.clone());
                }
            }
            AstNode::Date(_value) => {
                // Eventually a way to get date information about the page
            }
        
            _ => {
                println!("unknown AST node found");
            }
        }
    }

    create_html_boilerplate(get_meta_config())
        .replace("page-js", &js)
        .replace("page-template", &html)
        .replace("page-css", &css)
        .replace("page-title", &page_title)
}


fn parse_scene(scene: Vec<AstNode>) -> String {
    let mut html = String::new();

    for node in scene {
        match node {
            AstNode::HTML(html_content) => {
                html.push_str(&html_content);
            }
            AstNode::Scene(scene) => {
                // If the newscene is inline, 
                // then move it into the last tag of the current scene
                // Otherwise just parse it into this one
                let new_scene = parse_scene(scene);
                html.push_str(&new_scene);
            }
            AstNode::Error(value) => {
                println!("Error: {}", value);
            }
            _ => {
                println!("unknown AST node found in scene");
            }
        }
    }

    html
}