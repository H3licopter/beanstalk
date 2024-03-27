use super::{generate_html::create_html_boilerplate, markdown_parser::add_markdown_tags};
use crate::{
    parsers::{ast::AstNode, util::count_newlines_at_end_of_string}, settings::get_meta_config, Token
};

// Parse ast into valid JS, HTML and CSS
pub fn parse(ast: Vec<AstNode>) -> (String, String) {
    let mut js = String::new();
    let _wasm = String::new();
    let mut html = String::new();
    let css = String::new();
    let mut page_title = String::new();

    let config = get_meta_config();

    // Parse HTML
    for node in ast {
        match node {
            // SCENES (HTML)
            AstNode::Scene(scene) => {
                html.push_str(&parse_scene(scene, &mut false).0);
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

            // JAVASCRIPT / WASM
            AstNode::VarDeclaration(name, expr ) => {
                let expressions = match *expr {
                    AstNode::Expression(e) => e,
                    _ => {
                        println!("Error: No expression found for variable declaration");
                        break;
                    }
                };
                js.push_str(&format!("let {} = {};", name, expression_to_js(expressions)));
            }
            AstNode::Function(name, args, body) => {
                js.push_str(&format!("function {}({:?}){{\n{:?}\n}}", 
                    name,
                    args,
                    body
                ));
            }
            AstNode::Print(expr) => {
                let expressions = match *expr {
                    AstNode::Expression(e) => e,
                    _ => {
                        println!("Error: No expression found for variable declaration");
                        break;
                    }
                };
                js.push_str(&format!("console.log({});", expression_to_js(expressions)));
            }

            _ => {
                println!("unknown AST node found");
            }
        }
    }
    

    (
        create_html_boilerplate(get_meta_config())
            .replace("page-template", &html)
            .replace("page-css", &css)
            .replace("page-title", &page_title),

        js
    )
}

fn expression_to_js(expr: Vec<AstNode>) -> String {
    let mut js = String::new();

    for node in expr {
        match node {
            AstNode::Literal(token) => {
                match token {
                    Token::IntLiteral(value) => {
                        js.push_str(&value.to_string());
                    }
                    Token::FloatLiteral(value) => {
                        js.push_str(&value.to_string());
                    }
                    Token::StringLiteral(value) => {
                        js.push_str(&format!("\"{}\"", value));
                    }
                    _ => {
                        println!("unknown literal found in expression");
                    }
                }
            }

            AstNode::Ref(name) => {
                js.push_str(&name);
            }

            AstNode::FunctionCall(name, args) => {
                js.push_str(&format!("{}({:?})", name, args));
            }

            AstNode::Add => {js.push_str("+");}
            AstNode::Subtract => {js.push_str(" - ");}
            AstNode::Multiply => {js.push_str("*");}
            AstNode::Divide => {js.push_str("/");}
            AstNode::Modulus => {js.push_str("%");}
            AstNode::Exponent => {js.push_str("**");}

            _ => {
                println!("unknown AST node found in expression");
            }
        }
    }

    js
}

fn parse_scene(scene: Vec<AstNode>, inside_p: &mut bool) -> (String, bool) {
    let mut html = String::new();
    let mut closing_tags = Vec::new();

    for node in scene {
        match node {
            AstNode::Element(token) => {
                match token {
                    Token::Span(content) => {
                        html.push_str(&format!(
                            "<span>{}",
                            add_markdown_tags(&mut content.clone())
                        ));
                        closing_tags.push("</span>".to_string());
                    }

                    Token::P(content) => {
                        html.push_str(collect_closing_tags(&mut closing_tags).as_str());

                        if *inside_p {
                            html.push_str(&add_markdown_tags(&mut content.clone()));
                        } else {
                            html.push_str(&format!(
                                "<p>{}",
                                add_markdown_tags(&mut content.clone())
                            ));

                            if count_newlines_at_end_of_string(&content) > 1 {
                                html.push_str("</p>");
                            } else {
                                *inside_p = true;
                                closing_tags.push("</p>".to_string());
                            }
                        }
                    }

                    Token::Heading(size, content) => {
                        html.push_str(collect_closing_tags(&mut closing_tags).as_str());
                        html.push_str(&format!(
                            "<h{}>{}",
                            size,
                            add_markdown_tags(&mut content.clone())
                        ));

                        if count_newlines_at_end_of_string(&content) > 0 {
                            html.push_str(&format!("</h{}>", size));
                        } else {
                            closing_tags.push(format!("</h{}>", size));
                        }
                    }

                    Token::BulletPoint(_indentation, content) => {
                        html.push_str(collect_closing_tags(&mut closing_tags).as_str());
                        html.push_str(&format!("<li>{}", add_markdown_tags(&mut content.clone())));
                        closing_tags.push("</li>".to_string());
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
                let new_scene = parse_scene(scene, inside_p);

                html.push_str(&new_scene.0);
            }

            AstNode::SceneTag(tag) => {
                html.push_str(&tag);
            }

            AstNode::Space => {
                html.push_str("&nbsp;");
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
        *inside_p = false;
        html.push_str(tag);
    }

    (html, *inside_p)
}

fn collect_closing_tags(closing_tags: &mut Vec<String>) -> String {
    let mut tags = String::new();

    closing_tags.reverse();
    while let Some(tag) = closing_tags.pop() {
        tags.push_str(&tag);
    }

    tags
}
