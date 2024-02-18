use crate::{settings::get_meta_config, ast::AstNode};
use super::generate_html::create_html_boilerplate;

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
                html.push_str(scene_html.0.as_str());
            }
            AstNode::Title(value) => {
                page_title = value;

                if config.auto_site_title {
                    page_title += &(" | ".to_owned() + &config.site_title.clone());
                }
            }
            AstNode::Page => {

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


fn parse_scene(scene: Vec<AstNode>) -> ( String, bool ) {
    let mut html = String::new();
    let mut properties = String::new();
    let mut tag = String::new();
    let mut inline = false;
    let mut scene_tag = String::new();

    for node in scene {
        match node {
            AstNode::ElementProperties(props, ele_tag) => {
                properties = props;
                scene_tag = ele_tag;
                if scene_tag == "span" {
                    inline = true;
                }
            }
            AstNode::HTML(content_tag, html_content) => {
                // If the new tag is different from the last tag, 
                // And tag is not empty, close the last tag
                if tag != content_tag && !tag.is_empty() {
                    html.push_str(&format!("</{}>", tag));
                }
                tag = content_tag;
                html.push_str(&html_content);
            }
            AstNode::Scene(scene) => {
                // If the newscene is inline, 
                // then move it into the last tag of the current scene
                // Otherwise just parse it into this one
                let new_scene = parse_scene(scene);
                if new_scene.1 {
                    html.push_str(&format!("{}</{}>", &new_scene.0, tag));
                } else {
                    html.push_str(&new_scene.0);
                }
            }
            AstNode::Gap => {
                // html.push_str(&format!("</{}>", tag));
                inline = false;
            }
            _ => {
                println!("unknown AST node found");
            }
        }
    }

    // Wrap html output in correct tag and return
    ( format!("<{} {}>{}</{}>", scene_tag, properties, html, scene_tag), inline )
}