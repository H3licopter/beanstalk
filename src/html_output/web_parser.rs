use super::{
    dom_hooks::{generate_dom_update_js, DOMUpdate},
    generate_html::create_html_boilerplate,
    js_parser::{collection_to_js, collection_to_vec_of_js, expression_to_js},
    markdown_parser::add_markdown_tags,
};
use crate::{
    parsers::{
        ast::AstNode, styles::{Style, Tag}, util::count_newlines_at_end_of_string
    }, settings::{get_html_config, HTMLMeta}, Token
};

// Parse ast into valid JS, HTML and CSS
pub fn parse(ast: Vec<AstNode>, config: HTMLMeta) -> String {
    let mut js = generate_dom_update_js(DOMUpdate::InnerHTML).to_string();
    let _wasm = String::new();
    let mut html = String::new();
    let css = String::new();
    let mut page_title = String::new();

    let mut module_references: Vec<usize> = Vec::new();

    // Parse HTML
    for node in ast {
        match node {
            // SCENES (HTML)
            AstNode::Scene(scene, scene_tags, scene_styles) => {
                html.push_str(&parse_scene(scene, scene_tags, scene_styles, &mut Tag::None, &mut js, &mut module_references).0);
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
            AstNode::VarDeclaration(id, expr, _) => {
                js.push_str(&format!("let v{} = {};", id, expression_to_js(&expr)));
            }
            AstNode::Const(id, expr, _) => {
                js.push_str(&format!("const cv{} = {};", id, expression_to_js(&expr)));
            }
            AstNode::Function(name, args, body, is_exported) => {
                js.push_str(&format!(
                    "{}function f{}({:?}){{\n{:?}\n}}",
                    if is_exported { "export " } else { "" },
                    name,
                    args, // NEED TO PARSE ARGUMENTS
                    body
                ));
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
fn parse_scene(scene: Vec<AstNode>, scene_tags: Vec<Tag>, scene_styles: Vec<Style>, parent_tag: &mut Tag, js: &mut String, module_references: &mut Vec<usize>) -> (String, Tag) {
    let mut html = String::new();
    let mut closing_tags = Vec::new();

    let mut template_ids: u32 = 0;
    
    // For tables
    let mut ele_count: u32 = 0;
    let mut columns: u32 = 0;

    let mut images: Vec<String> = Vec::new();
    let mut scene_wrap = SceneTag {
        tag: Tag::None,
        properties: String::new(),
        style: String::new(),
    };

    for style in scene_styles {
        match style {
            Style::Padding(arg) => {
                scene_wrap
                    .style
                    .push_str(&format!("padding:{}rem;", expression_to_js(&arg)));
                scene_wrap.tag = Tag::Span;
            }
            Style::Margin(arg) => {
                scene_wrap
                    .style
                    .push_str(&format!("margin:{}rem;", expression_to_js(&arg)));
                scene_wrap.tag = Tag::Span;
            }
            Style::BackgroundColor(args) => {
                scene_wrap
                    .style
                    .push_str(&format!("background-color:rgb({});", collection_to_js(&args)));
                scene_wrap.tag = Tag::Span;
            }
            Style::TextColor(args) => {
                scene_wrap
                    .style
                    .push_str(&format!("color:rgb({});", collection_to_js(&args)));
                scene_wrap.tag = Tag::Span;
            }
            Style::Size(arg) => {
                let size = collection_to_vec_of_js(&arg);

                // TO DO: Add or remove parameters based on number of arguments
                // And make sure there are no more than 4 arguments
                scene_wrap
                    .style
                    .push_str(&format!("width:{}rem;height:{}rem", size[0], size[1]));
            }
            Style::Alt(value) => {
                scene_wrap
                    .properties
                    .push_str(&format!(" alt=\"{}\"", value));
            }
        }
    }

    let mut img_count = 0;
    let img_default_dir = get_html_config().image_folder_url.clone();

    for tag in &scene_tags {
        match tag {
            Tag::Img(src) => {
                images.push(format!("{img_default_dir}{src}"));
                img_count += 1;
            }
            Tag::Table(c) => {
                match scene_wrap.tag {
                    Tag::Img(_) | Tag::Video(_) | Tag::Audio(_) => {
                       // TO DO: Error handling that passes correctly into the AST for the end user
                    }
                    _ => {
                        scene_wrap.tag = Tag::Table(columns);
                        columns = *c;
                        ele_count = 0;
                        html.push_str("<table><thead>");
                    }
                }
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

    let mut scenehead_literals: Vec<AstNode> = Vec::new();

    for node in scene {
        match node {
            AstNode::Element(token) => {
                match token {
                    Token::Span(content) => {
                        html.push_str(&format!(
                            "<span>{}",
                            add_markdown_tags(&mut content.clone())
                        ));
                        
                        match *parent_tag {
                            Tag::P => {
                                if count_newlines_at_end_of_string(&content) > 1 {
                                    html.push_str("</p>");
                                    *parent_tag = Tag::None;
                                }
                            }
                            Tag::Th => {
                                closing_tags.push("</th>".to_string());
                            }
                            Tag::Td => {
                                closing_tags.push("</td>".to_string());
                            }
                            _ => {
                                *parent_tag = Tag::Span;
                            }
                        }

                        closing_tags.push("</span>".to_string());
                    }

                    Token::P(content) => {
                        match scene_wrap.tag {
                            Tag::Img(_) | Tag::Video(_) => {
                                scene_wrap
                                    .properties
                                    .push_str(&format!(" alt=\"{}\"", content));
                            }
                            Tag::Table(_) => {
                                // basically just don't add any wrapping tags inside tables
                                html.push_str(collect_closing_tags(&mut closing_tags).as_str());
                                html.push_str(&add_markdown_tags(&mut content.clone()));
                            }
                            _ => {
                                html.push_str(collect_closing_tags(&mut closing_tags).as_str());

                                let parsed_content = add_markdown_tags(&mut content.clone());
                                match *parent_tag {
                                    Tag::P => {
                                        html.push_str(&parsed_content);
                                        closing_tags.push("</p>".to_string());

                                        if count_newlines_at_end_of_string(&content) > 1 {
                                            html.push_str("</p>");
                                            *parent_tag = Tag::None;
                                        } else {
                                            *parent_tag = Tag::P;
                                        }
                                    }
                                    Tag::Table(_) => {
                                        html.push_str(&parsed_content);
                                    }
                                    Tag::Th => {
                                        html.push_str(&format!("<th scope='col'>{}", parsed_content));
                                        closing_tags.push("</th>".to_string());
                                    }
                                    Tag::Td => {
                                        html.push_str(&format!("<td>{}", parsed_content));
                                        closing_tags.push("</td>".to_string());
                                    }
                                    _=> {
                                        match scene_wrap.tag {
                                            Tag::Table(_) => {
                                                html.push_str(&format!("<th>{}</th>", parsed_content));
                                            }
                                            Tag::Span => {
                                                html.push_str(&format!("<span style=\"{}\">{}</span>", scene_wrap.style, parsed_content));
                                            }
                                            _ => {
                                                html.push_str(&format!("<p>{}", parsed_content));
                                                if count_newlines_at_end_of_string(&content) > 1 {
                                                    html.push_str("</p>");
                                                    *parent_tag = Tag::None;
                                                } else {
                                                    *parent_tag = Tag::P;
                                                }
                                            }
                                        }
                                    }
                                }
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

            AstNode::Scene(scene, scene_tags, scene_styles) => {
                add_table_open_tags(&mut html, columns, &mut ele_count, parent_tag);
                let new_scene = parse_scene(scene, scene_tags, scene_styles, parent_tag, js, module_references);
                html.push_str(&new_scene.0);
                add_table_closing_tags(&mut html, columns, &mut ele_count, parent_tag);
            }

            AstNode::Space => {
                html.push_str("&nbsp;");
            }

            // STUFF THAT IS INSIDE SCENE HEAD THAT NEEDS TO BE PASSED INTO SCENE BODY
            AstNode::VarReference(value) => {
                // Create a span in the HTML with a class that can be referenced by JS
                // TO DO: Should be reactive in future
                html.push_str(&format!("<span class=\"c{value}\"></span>"));

                if !module_references.contains(&value) {
                    js.push_str(&format!("uInnerHTML(\"c{value}\", v{value});"));
                    module_references.push(value);
                }
            }
            AstNode::ConstReference(value) => {
                // Create a span in the HTML with a class that can be referenced by JS
                html.push_str(&format!("<span class=\"c{value}\"></span>"));

                if !module_references.contains(&value) {
                    js.push_str(&format!("uInnerHTML(\"c{value}\", cv{value});"));
                    module_references.push(value);
                }
            }
            
            // WILL CALL WASM FUNCTIONS
            AstNode::RuntimeExpression(expr, expr_type) => {
                scenehead_literals.push(
                    AstNode::RuntimeExpression(expr, expr_type)
                );
            }

            AstNode::Tuple(items) => {
                for item in items {
                    scenehead_literals.push(item);
                }
            }

            AstNode::Literal(token) => {
                scenehead_literals.push(AstNode::Literal(token));
            }

            AstNode::SceneTemplate => {
                html.push_str(&format!("<span id=\"exp{template_ids}\"></span>"));
                template_ids += 1;
            }

            // ERROR HANDLING
            AstNode::Error(value) => {
                println!("Error: {}", value);
            }
            _ => {
                println!("unknown AST node found in scene: {:?}", node);
            }
        }
    }


    // Take all scenehead variables and add them into any templates inside of the scene body
    // When there are no templates left, create a new span element to hold the literal
    let mut id: u32 = 0;
    for literal in scenehead_literals {        
        let number_of_templates = template_ids;
        let mut js_string = String::new();

        match literal {
            AstNode::RuntimeExpression(expr, expr_type) => {
                js_string = expression_to_js(&AstNode::RuntimeExpression(expr, expr_type));
            }
            AstNode::Literal(token) => {
                match token {
                    Token::StringLiteral(value) | Token::RawStringLiteral(value) => {
                        js_string = format!("'{}'", value);
                    }
                    Token::RuneLiteral(char) => {
                        js_string = format!("'{}'", char);
                    }
                    Token::IntLiteral(value) =>{
                        js_string = value.to_string();
                    }
                    Token::FloatLiteral(value) => {
                        js_string = value.to_string();
                    }
                    Token::DecLiteral(value) => {
                        js_string = value.to_string();
                    }
                    Token::BoolLiteral(value) => {
                        js_string = value.to_string();
                    }
                    _ => {}
                }
            }
            _=> {
                println!("Scene Head literal not yet supported in scene body");
            }
        }


        // If there are no more templates inside the scene
        if id > number_of_templates {
            html.push_str(&format!("<span id=\"exp{id}\"></span>"));
        }

        js.push_str(&format!("document.getElementById('exp{id}').innerHTML={js_string};"));

        id += 1;
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
            if *parent_tag == Tag::P {
                html.insert_str(0, "</p>");
                *parent_tag = Tag::None;
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
            if *parent_tag == Tag::P {
                html.insert_str(0, "</p>");
                *parent_tag = Tag::None;
            }
        }
        Tag::Table(_) => {
            html.push_str("</tbody></table>");
        }
        Tag::Video(src) => {
            html.insert_str(
                0,
                &format!(
                    "<video src=\"{}\" style=\"{}\" {} controls />",
                    src, scene_wrap.style, scene_wrap.properties
                ),
            );
            if *parent_tag == Tag::P {
                html.insert_str(0, "</p>");
                *parent_tag = Tag::None;
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

    (html, parent_tag.to_owned())
}

fn add_table_open_tags(html: &mut String, columns: u32, ele_count: &mut u32, parent_tag: &mut Tag) {
    // If this is a table scene, add appropriate table element tags
    if columns > 0 {
        *ele_count += 1;

        match *ele_count % columns {
            // If this is the last element for this column
            0 => {
                // Open the table element, should be a td unless there is just one column
                if columns > 1 && *ele_count > columns {
                    *parent_tag = Tag::Td;
                } else {
                    *parent_tag = Tag::Th;
                }
            },
            // if this is the first element for this column
            1 => {
                // If this is the first element for this column, open the table column and first element
                html.push_str("<tr>");
                *parent_tag = Tag::Th;
            },
            _=> {
                // If first row, make sure all the table elements are headings
                if *ele_count < columns {
                    *parent_tag = Tag::Th;
                } else {
                    *parent_tag = Tag::Td;
                }
            }
        }
    }
}

fn add_table_closing_tags(html: &mut String, columns: u32, ele_count: &mut u32, parent_tag: &mut Tag) {
    // If this is a table scene, close the table row (and possibly column)
    if columns > 0 && *ele_count > 0 {
        
        // If this is the last element for this column
        if *ele_count % columns == 0 {
            // Close the table element and column, should be a td unless there is just one column
            if columns > 1 && *ele_count > columns {
                html.push_str("</tr>");
            } else {
                html.push_str("</tr>");
            }

            // If this is the first row of elements, close the heading tag and open the body tag too
            if *ele_count == columns {
                html.push_str("</thead><tbody>");
                *parent_tag = Tag::None;
            }
        }
    }
}

fn collect_closing_tags(closing_tags: &mut Vec<String>) -> String {
    let mut tags = String::new();

    closing_tags.reverse();
    while let Some(tag) = closing_tags.pop() {
        tags.push_str(&tag);
    }

    tags
}
