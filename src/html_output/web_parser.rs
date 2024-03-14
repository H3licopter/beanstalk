use crate::{ast::AstNode, parsers::util::count_newlines_at_end_of_string, settings::get_meta_config, Token};
use super::{generate_html::create_html_boilerplate, markdown_parser::add_tags};

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
                html.push_str(&parse_scene(scene));
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
    let mut closing_tags = Vec::new();

    for node in scene {
        match node {

            AstNode::Element(token) => {
                match token {

                    Token::Span(content) => {
                        html.push_str(&format!("<span>{}", add_tags(&mut content.clone(), &mut 0)));
                        closing_tags.push("</span>".to_string());
                    }

                    Token::P(content) => {
                        html.push_str(collect_closing_tags(&mut closing_tags).as_str());
                        html.push_str(&format!("<p>{}", add_tags(&mut content.clone(), &mut 0)));

                        if count_newlines_at_end_of_string(&content) > 1 {
                            html.push_str("</p>");
                        } else {
                            closing_tags.push("</p>".to_string());
                        }
                    }

                    Token::Heading(size, content) => {
                        html.push_str(collect_closing_tags(&mut closing_tags).as_str());
                        html.push_str(&format!("<h{}>{}", size, add_tags(&mut content.clone(), &mut 0)));

                        if count_newlines_at_end_of_string(&content) > 0 {
                            html.push_str(&format!("</h{}>", size));
                        } else {
                            closing_tags.push(format!("</h{}>", size));
                        }
                    }

                    Token::Pre(content) => {
                        html.push_str(collect_closing_tags(&mut closing_tags).as_str());
                        html.push_str(&format!("<pre>{}", content));
                        closing_tags.push("</pre>".to_string());
                    }

                    _ => {}
                };
            }

            AstNode::Scene(scene) => {
                let new_scene = parse_scene(scene);
                html.push_str(&new_scene);
            }

            AstNode::SceneTag(tag) => {
                html.push_str(&tag);
            }

            AstNode::Error(value) => {
                println!("Error: {}", value);
            }

            _ => {
                println!("unknown AST node found in scene");
            }
        }
    }

    for tag in closing_tags.iter().rev() {
        html.push_str(tag);
    }

    html
}

fn collect_closing_tags(closing_tags: &mut Vec<String>) -> String {
    let mut tags = String::new();
    
    closing_tags.reverse();
    while let Some(tag) = closing_tags.pop() {
        tags.push_str(&tag);
    }

    tags
}