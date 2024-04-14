use super::{generate_html::create_html_boilerplate, js_parser::{collection_to_vec_of_js, combine_collection_to_js, expression_to_js}, markdown_parser::add_markdown_tags};
use crate::{
    parsers::{
        ast::AstNode,
        styles::{Style, Tag},
        util::count_newlines_at_end_of_string,
    },
    settings::{get_html_config, HTMLMeta},
    Token,
};

// Parse ast into valid JS, HTML and CSS
pub fn parse(ast: Vec<AstNode>, config: HTMLMeta) -> String {
    let mut js = String::new();
    let _wasm = String::new();
    let mut html = String::new();
    let css = String::new();
    let mut page_title = String::new();

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
                js.push_str(&format!(
                    "let {} = {};",
                    name,
                    expression_to_js(&expr)
                ));
            }
            AstNode::Function(name, args, body) => {
                js.push_str(&format!("function {}({:?}){{\n{:?}\n}}", name, args, body));
            }
            AstNode::Print(expr) => {
                js.push_str(&format!("console.log({});", expression_to_js(&expr)));
            }

            _ => {
                println!("unknown AST node found");
            }
        }
    }

    create_html_boilerplate(config)
        .replace("page-template", &html)
        .replace("@page-css", &css)
        .replace("page-title", &page_title)
        .replace("//js", &js)
}

struct SceneTag {
    tag: Tag,
    properties: String,
    style: String,
}

// Returns a string of the HTML and whether the scene is inside a paragraph
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
                let mut images: Vec<String> = Vec::new();

                for style in styles {
                    match style {
                        Style::TextColor(arg) => {
                            scene_wrap
                                .style
                                .push_str(&format!("color:rgb({});", combine_collection_to_js(&arg)));
                            scene_wrap.tag = Tag::Span;
                        }
                        Style::Size(arg) => {
                            let size = collection_to_vec_of_js(&arg);

                            // TO DO: Add or remove parameters based on number of arguments
                            // And make sure there are no more than 4 arguments
                            scene_wrap
                                .style
                                .push_str(&format!("width:{}px;height:{}px", size[0], size[1]));
                        }
                        Style::Alt(value) => {
                            scene_wrap
                                .properties
                                .push_str(&format!(" alt=\"{}\"", value));
                        }
                        _ => {}
                    }
                }

                let mut img_count = 0;
                let img_default_dir = get_html_config().image_folder_url.clone();

                for tag in &tags {
                    match tag {
                        Tag::Img(src) => {
                            images.push(format!("{img_default_dir}{src}"));
                            img_count += 1;
                        }
                        Tag::Video(src) => {
                            scene_wrap.tag = Tag::Video(format!("{src}"));
                            if img_count > 0 {
                                scene_wrap
                                    .properties
                                    .push_str(&format!(" poster=\"{}\"", images[0]));
                            }

                            continue;
                        }
                        Tag::Audio(src) => {
                            scene_wrap.tag = Tag::Audio(src.to_string());
                        }
                        _ => {}
                    }
                }

                if img_count == 1 && scene_wrap.tag == Tag::None {
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
                            "<img src=\"{image}\" style=\"width:{img_resize}%;height:{img_resize}%;\"/>"
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
                    "<div style=\"{}\" {} >",
                    scene_wrap.style, scene_wrap.properties
                ),
            );
            if *inside_p {
                html.insert_str(0, "</p>");
                *inside_p = false;
            }
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
                    "<img src=\"{}\" style=\"{}\" {} />",
                    src, scene_wrap.style, scene_wrap.properties
                ),
            );
            if *inside_p {
                html.insert_str(0, "</p>");
                *inside_p = false;
            }
        }
        Tag::Video(src) => {
            html.insert_str(
                0,
                &format!(
                    "<video src=\"{}\" style=\"{}\" {} controls />",
                    src, scene_wrap.style, scene_wrap.properties
                ),
            );
            if *inside_p {
                html.insert_str(0, "</p>");
                *inside_p = false;
            }
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
