use crate::{settings::get_meta_config, ast::AstNode};
use super::generate_html::create_html_boilerplate;

// Parse ast into valid JS, HTML and CSS
pub fn parse(ast: Vec<AstNode>) -> String {
    let js = String::new();
    let _wasm = String::new();
    let mut html = String::new();
    let css = String::new();

    // Parse HTML
    for node in ast {
        match node {
            AstNode::Scene(scene) => {
                let scene_html = parse_scene(scene);
                html.push_str(scene_html.as_str());
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
}


fn parse_scene(scene: Vec<AstNode>) -> String {
    let mut html = String::new();

    for node in scene {
        match node {
            AstNode::HTML(html_content) => {
                html.push_str(&html_content);
            }
            AstNode::Scene(scene) => {
                html.push_str(parse_scene(scene).as_str());
            }
            _ => {
                println!("unknown AST node found");
            }
        }
    }

    html
}