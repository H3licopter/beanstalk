use crate::{ast::AstNode, settings::get_meta_config, Token};
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
                let scene_html = parse_scene(scene, &Vec::new()).0;
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


fn parse_scene(scene: Vec<AstNode>, parent_open_tags: &Vec<String>) -> ( String, Vec<String> ) {
    let mut html = String::new();
    let mut closing_tags = Vec::new();

    for node in scene {
        match node {

            AstNode::Block(token) => {
                match token {

                    Token::P(value) => {
                        html.push_str(&format!("<p>{}", value));
                        closing_tags.push("</p>".to_string());
                    }

                    Token::Heading(size, value) => {
                        html.push_str(&format!("<h{}>{}", size, value));
                        closing_tags.push(format!("</h{}>", size));
                    }
                    
                    Token::A(href, content) => {
                        html.push_str(&format!("<a href=\"{}\">{}", href, content));
                        closing_tags.push("</a>".to_string());
                    }
                    _ => {}
                };
            }

            AstNode::Inline(token) => {
                html.push_str("<span>");
                closing_tags.push("</span>".to_string());

                // Check if the last parent closing tag is the same as the element
                // If it is, this element does not have tags
                match token {
                    Token::P(value) => {
                        if previous_tag_is_same_as_current(&"</p>".to_string(), &parent_open_tags) {
                            html.push_str(&value);
                        } else {
                            html.push_str(&format!("<p>{}", value));
                            closing_tags.push("</p>".to_string());
                        }
                    }

                    Token::Heading(size, value) => {
                        if previous_tag_is_same_as_current(&format!("</h{}>", size), &parent_open_tags) {
                            html.push_str(&value);
                        } else {
                            html.push_str(&format!("<h{}>{}", size, value));
                            closing_tags.push(format!("</h{}>", size));
                        }
                    }

                    Token::A(href, content) => {
                        html.push_str(&format!("<a href=\"{}\">{}", href, content));
                        closing_tags.push("</a>".to_string());
                    }
                    _ => {}
                };
            }

            AstNode::Scene(scene) => {
                let new_scene = parse_scene(scene, &closing_tags).0;
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

    // Close off any remaining open tags created inside of this scene
    for tag in closing_tags.iter().rev() {
        html.push_str(tag);
    }

    ( html, closing_tags )
}

fn previous_tag_is_same_as_current(tag: &String, parent_open_tags: &Vec<String>) -> bool {
    match parent_open_tags.last() {
        Some(last_tag) => {
            last_tag == tag
        }
        _ => {
            false
        }
    }
}