use super::{generate_html::create_html_boilerplate, markdown_parser::add_markdown_tags};
use crate::{
    parsers::{
        ast::AstNode,
        styles::{Style, Tag},
        util::count_newlines_at_end_of_string,
    },
    settings::get_meta_config,
    Token,
};

// Parse ast into valid JS, HTML and CSS
pub fn parse(ast: Vec<AstNode>) -> String {
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
            AstNode::VarDeclaration(name, expr) => {
                let expressions = match *expr {
                    AstNode::Expression(e, _) => e,
                    _ => {
                        println!("Error: No expression found for variable declaration");
                        break;
                    }
                };
                js.push_str(&format!(
                    "let {} = {};",
                    name,
                    expression_to_js(expressions)
                ));
            }
            AstNode::Function(name, args, body) => {
                js.push_str(&format!("function {}({:?}){{\n{:?}\n}}", name, args, body));
            }
            AstNode::Print(expr) => {
                let expressions = match *expr {
                    AstNode::Expression(e, _) => e,
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

    create_html_boilerplate(get_meta_config())
        .replace("page-template", &html)
        .replace("@page-css", &css)
        .replace("page-title", &page_title)
        .replace("//js", &js)
}

fn expression_to_js(expr: Vec<AstNode>) -> String {
    let mut js = String::new();

    for node in expr {
        match node {
            AstNode::Literal(token) => match token {
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
            },

            AstNode::Ref(name) => {
                js.push_str(&name);
            }

            AstNode::FunctionCall(name, args) => {
                js.push_str(&format!("{}({:?})", name, args));
            }

            AstNode::Add => {
                js.push_str("+");
            }
            AstNode::Subtract => {
                js.push_str(" - ");
            }
            AstNode::Multiply => {
                js.push_str("*");
            }
            AstNode::Divide => {
                js.push_str("/");
            }
            AstNode::Modulus => {
                js.push_str("%");
            }
            AstNode::Exponent => {
                js.push_str("**");
            }

            _ => {
                println!("unknown AST node found in expression");
            }
        }
    }

    js
}

struct SceneTag {
    tag: Tag,
    properties: String,
    style: String,
}

fn parse_scene(scene: Vec<AstNode>, inside_p: &mut bool) -> (String, bool) {
    let mut html = String::new();
    let mut closing_tags = Vec::new();

    let mut scene_wrap = SceneTag {
        tag: Tag::None,
        properties: String::new(),
        style: String::new(),
    };

    for node in scene {
        match node {
            AstNode::SceneTag(tags, styles) => {
                // Should automatically create and format a grid
                // If there are mutltiple video/ img elements
                // If there is an image element inside of the video element,
                // Then the image element becomes the thumbnail
                let mut images: Vec<String> = Vec::new();

                for style in styles {
                    match style {
                        Style::TextColor(r, g, b) => {
                            scene_wrap
                                .style
                                .push_str(&format!("color:rgb({},{},{});", r, g, b));
                            scene_wrap.tag = Tag::Span;
                        }
                        Style::Width(value) => {
                            scene_wrap.style.push_str(&format!("width:{}px;", value));
                        }
                        Style::Height(value) => {
                            scene_wrap.style.push_str(&format!("height:{}px;", value));
                        }
                        _ => {}
                    }
                }

                let img_count = images.len();

                for tag in &tags {
                    match tag {
                        Tag::Img(src) => {
                            images.push(src.to_string());
                        }
                        Tag::Video(src) => {
                            scene_wrap.tag = Tag::Video(src.to_string());
                            if img_count > 0 {
                                scene_wrap
                                    .properties
                                    .push_str(&format!(" poster=\"{}\"", images[0]));
                            }

                            continue;
                        }
                        _ => {}
                    }
                }

                if img_count == 1 && scene_wrap.tag == Tag::Span {
                    scene_wrap.tag = Tag::Img(images[0].to_string());
                }

                // If there are multiple images, turn it into a grid of images
                if img_count > 1 {
                    scene_wrap.tag = Tag::Div;
                    scene_wrap
                        .properties
                        .push_str(&format!(" class=\"bs-img-grid\""));
                    let img_resize = 100.0 / f32::sqrt(img_count as f32);
                    for image in images {
                        html.push_str(&format!(
                            "<img src=\"{}\" alt=\"\" style=\"width:{}%;height:{}%;\"/>",
                            image, img_resize, img_resize
                        ));
                    }
                }
            }

            AstNode::Element(token) => {
                match token {
                    Token::Span(content) => {
                        html.push_str(&format!(
                            "<span>{}",
                            add_markdown_tags(&mut content.clone())
                        ));
                        closing_tags.push("</span>".to_string());

                        if *inside_p && count_newlines_at_end_of_string(&content) > 1 {
                            html.push_str("</p>");
                            *inside_p = false;
                        }
                    }

                    Token::P(content) => match scene_wrap.tag {
                        Tag::Img(_) | Tag::Video(_) => {
                            scene_wrap
                                .properties
                                .push_str(&format!(" alt=\"{}\"", content));
                        }
                        _ => {
                            html.push_str(collect_closing_tags(&mut closing_tags).as_str());

                            let parsed_content = add_markdown_tags(&mut content.clone());
                            if *inside_p {
                                html.push_str(&parsed_content);
                                closing_tags.push("</p>".to_string());
                            } else {
                                html.push_str(&if scene_wrap.tag == Tag::Span {
                                    format!(
                                        "<p><span style=\"{}\">{}</span>",
                                        scene_wrap.style, parsed_content
                                    )
                                } else {
                                    format!("<p>{}", parsed_content)
                                });
                            }

                            if count_newlines_at_end_of_string(&content) > 1 {
                                html.push_str("</p>");
                                *inside_p = false;
                            } else {
                                *inside_p = true;
                            }
                        }
                    },

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

                    Token::Superscript(content) => {
                        html.push_str(&format!("<sup>{}</sup>", content));
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
        html.push_str(tag);
    }

    match scene_wrap.tag {
        Tag::P => {
            html.insert_str(
                0,
                &format!(
                    "<p style=\"{}\" {}>",
                    scene_wrap.style, scene_wrap.properties
                ),
            );
            html.push_str("</p>");
        }
        Tag::Span => {
            html.insert_str(
                0,
                &format!(
                    "<span style=\"{}\" {}>",
                    scene_wrap.style, scene_wrap.properties
                ),
            );
            html.push_str("</span>");
        }
        Tag::Div => {
            html.insert_str(
                0,
                &format!(
                    "<div style=\"{}\" {}>",
                    scene_wrap.style, scene_wrap.properties
                ),
            );
            html.push_str("</div>");
        }
        Tag::A(href) => {
            html.insert_str(
                0,
                &format!(
                    "<a href=\"{}\" style=\"{}\" {}>",
                    href, scene_wrap.style, scene_wrap.properties
                ),
            );
            html.push_str("</a>");
        }
        Tag::Img(src) => {
            html.insert_str(
                0,
                &format!(
                    "<img src=\"{}\" style=\"{}\" {}",
                    src, scene_wrap.style, scene_wrap.properties
                ),
            );
            html.push_str("\"/>");
        }
        Tag::Video(src) => {
            html.insert_str(
                0,
                &format!(
                    "<video src=\"{}\" style=\"{}\" {} controls />",
                    src, scene_wrap.style, scene_wrap.properties
                ),
            );
        }
        Tag::Audio(src) => {
            html.insert_str(
                0,
                &format!(
                    "<audio src=\"{}\" style=\"{}\" {} controls />",
                    src, scene_wrap.style, scene_wrap.properties
                ),
            );
        }
        _ => {}
    };

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
