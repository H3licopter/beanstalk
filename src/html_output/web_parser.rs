use super::{
    dom_hooks::{generate_dom_update_js, DOMUpdate},
    generate_html::create_html_boilerplate,
    js_parser::{collection_to_js, collection_to_vec_of_js, expression_to_js},
};
use crate::{
    bs_css::get_bs_css,
    bs_types::DataType,
    parsers::{
        ast::AstNode,
        styles::{Style, Tag},
        util::{count_newlines_at_end_of_string, count_newlines_at_start_of_string},
    },
    settings::{get_html_config, HTMLMeta},
    Token,
};
use colour::red_ln;

// Parse ast into valid JS, HTML and CSS
pub fn parse(ast: Vec<AstNode>, config: HTMLMeta, release_build: bool) -> String {
    let mut js = generate_dom_update_js(DOMUpdate::InnerHTML).to_string();
    let _wasm = String::new();
    let mut html = String::new();
    let mut css = String::new();
    let mut page_title = String::new();
    let mut exp_id: usize = 0;

    let mut module_references: Vec<AstNode> = Vec::new();

    let mut class_id: usize = 0;

    // Parse HTML
    for node in ast {
        match node {
            // SCENES (HTML)
            AstNode::Scene(scene, scene_tags, scene_styles) => {
                html.push_str(&parse_scene(
                    scene,
                    scene_tags,
                    scene_styles,
                    &mut Tag::None,
                    &mut js,
                    &mut css,
                    &mut module_references,
                    &mut class_id,
                    &mut exp_id,
                    &mut Vec::new(),
                ));
            }
            AstNode::Title(value) => {
                page_title = value;
            }
            AstNode::Date(_value) => {
                // Eventually a way to get date information about the page
            }

            // JAVASCRIPT / WASM
            AstNode::VarDeclaration(id, ref expr, _) => {
                js.push_str(&format!("let v{} = {};", id, expression_to_js(&expr)));
                module_references.push(node);
            }
            AstNode::Const(_, _, _) => {
                module_references.push(node);
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
            AstNode::Comment(_) => {
                // Comments are not added to the final output (Atm). Maybe there will be some documentation thing eventually.
            }

            _ => {
                red_ln!("unknown AST node found: {:?}", node);
            }
        }
    }

    if config.auto_site_title {
        page_title += &(" | ".to_owned() + &config.site_title.clone());
    }

    create_html_boilerplate(config, release_build)
        .replace("page-template", &html)
        .replace("@page-css", &css)
        .replace("page-title", &page_title)
        .replace("//js", &js)
}

struct SceneTag {
    tag: Tag,
    outer_tag: Tag,
    properties: String,
    style: String,
    child_styles: String,
}

// Returns a string of the HTML and the tag the scene is inside of
fn parse_scene(
    scene: Vec<AstNode>,
    scene_tags: Vec<Tag>,
    scene_styles: Vec<Style>,
    parent_tag: &mut Tag,
    js: &mut String,
    css: &mut String,
    module_references: &mut Vec<AstNode>,
    class_id: &mut usize,
    exp_id: &mut usize,
    positions: &mut Vec<i64>,
) -> String {
    let mut html = String::new();
    let mut closing_tags = Vec::new();

    // For tables
    let mut ele_count: u32 = 0;
    let mut columns: u32 = 0;

    let mut images: Vec<&AstNode> = Vec::new();
    let mut scene_wrap = SceneTag {
        tag: Tag::None,
        outer_tag: match parent_tag {
            Tag::List => Tag::List,
            _ => Tag::None,
        },
        properties: String::new(),
        style: String::new(),
        child_styles: String::new(),
    };

    let mut style_assigned = false;
    for style in scene_styles {
        match style {
            Style::Padding(arg) => {
                // If literal, pass it straight in
                // If tuple, spread the values into the padding property
                match arg {
                    AstNode::Literal(Token::IntLiteral(value)) => {
                        scene_wrap.style.push_str(&format!("padding:{}rem;", value));
                    }
                    AstNode::Tuple(values) => {
                        let mut padding = String::new();
                        for value in values {
                            match value {
                                AstNode::Literal(Token::IntLiteral(value)) => {
                                    padding.push_str(&format!("{}rem ", value));
                                }
                                _ => {
                                    red_ln!(
                                        "Error: Padding must be a literal or a tuple of literals"
                                    );
                                }
                            }
                        }
                        scene_wrap.style.push_str(&format!("padding:{};", padding));
                    }
                    _ => {
                        red_ln!("Error: Padding must be a literal or a tuple of literals");
                    }
                }
                style_assigned = true;
            }
            Style::Margin(arg) => {
                scene_wrap
                    .style
                    .push_str(&format!("margin:{}rem;", expression_to_js(&arg)));
                // Only switch to span if there is no tag
                match scene_wrap.tag {
                    Tag::None => {
                        scene_wrap.tag = Tag::Span;
                    }
                    _ => {}
                }
                style_assigned = true;
            }
            Style::BackgroundColor(args) => {
                scene_wrap.child_styles.push_str(&format!(
                    "background-color:rgb({});",
                    collection_to_js(&args)
                ));
                // Only switch to span if there is no tag
                match scene_wrap.tag {
                    Tag::None => {
                        scene_wrap.tag = Tag::Span;
                    }
                    _ => {}
                }
                style_assigned = true;
            }
            Style::TextColor(args, type_of_color) => {
                let color_keyword = match type_of_color {
                    Token::Rgb => "rgb",
                    Token::Hsl => "hsl",
                    _ => {
                        red_ln!("Error: Invalid color type provided for text color");
                        ""
                    }
                };

                scene_wrap.child_styles.push_str(&format!(
                    "color:{}({});",
                    color_keyword,
                    collection_to_js(&args)
                ));
                // Only switch to span if there is no tag
                match scene_wrap.tag {
                    Tag::None => {
                        scene_wrap.tag = Tag::Span;
                    }
                    _ => {}
                }
                style_assigned = true;
            }
            Style::Size(arg) => {
                let size = collection_to_vec_of_js(&arg);

                // TO DO: Add or remove parameters based on number of arguments
                // And make sure there are no more than 4 arguments
                scene_wrap
                    .style
                    .push_str(&format!("width:{}rem;height:{}rem", size[0], size[1]));
                style_assigned = true;
            }
            Style::Alt(value) => {
                scene_wrap
                    .properties
                    .push_str(&format!(" alt=\"{}\"", value));
                style_assigned = true;
            }
            Style::Center(vertical) => {
                scene_wrap.style.push_str(
                    "display:flex;align-items:center;flex-direction:column;text-align:center;",
                );
                if vertical {
                    scene_wrap.style.push_str("justify-content:center;");
                }
                scene_wrap.tag = Tag::Div;
            }
            // Must adapt it's behaviour based on the parent tag and siblings
            Style::Order(node) => {
                let mut order = 0;
                match node {
                    AstNode::Literal(token) => match token {
                        Token::IntLiteral(value) => {
                            order = value;
                        }
                        _ => {
                            red_ln!("Incorrect type arguments passed into order declaration (must be an integer literal)");
                        }
                    },
                    _ => {
                        red_ln!("Incorrect arguments passed into order declaration");
                    }
                };
                match parent_tag {
                    Tag::Nav(_) => {
                        positions.push(order);
                    }
                    _ => {
                        red_ln!("Order not implemented for this tag yet");
                    }
                }
                style_assigned = true;
            }
        }
    }

    let mut img_count = 0;
    let img_default_dir = get_html_config().image_folder_url.to_owned();

    for tag in &scene_tags {
        match tag {
            Tag::Img(value) => {
                images.push(value);
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
                    }
                }
            }
            Tag::Video(value) => {
                scene_wrap.tag = Tag::Video(value.to_owned());

                // TO DO, add poster after images are parsed
                // if img_count > 0 {
                //     let poster = format!("{}{}", img_default_dir, images[0]);
                //     scene_wrap
                //         .properties
                //         .push_str(&format!(" poster=\"{}\"", poster));
                // }

                continue;
            }
            Tag::Audio(src) => {
                scene_wrap.tag = Tag::Audio(src.to_owned());
            }
            Tag::A(node) => {
                scene_wrap.tag = Tag::A(node.to_owned());
            }
            Tag::Nav(style) => {
                match scene_wrap.tag {
                    Tag::Img(_) | Tag::Video(_) | Tag::Audio(_) => {
                        // TO DO: Error handling that passes correctly into the AST for the end user
                    }
                    _ => {
                        scene_wrap.tag = Tag::Nav(style.to_owned());
                    }
                }
            }

            // Structure of the page
            Tag::Main => {
                if scene_wrap.outer_tag == Tag::None {
                    scene_wrap.outer_tag = Tag::Main;
                }
            }
            Tag::Header => {
                if scene_wrap.outer_tag == Tag::None {
                    scene_wrap.outer_tag = Tag::Header;
                }
            }
            Tag::Footer => {
                if scene_wrap.outer_tag == Tag::None {
                    scene_wrap.outer_tag = Tag::Footer;
                }
            }
            Tag::Section => {
                if scene_wrap.outer_tag == Tag::None {
                    scene_wrap.outer_tag = Tag::Section;
                }
            }
            _ => {}
        }
    }

    if img_count == 1 {
        match scene_wrap.tag {
            Tag::None => {
                scene_wrap.tag = Tag::Img(images[0].clone());
            }
            Tag::Video(_) => {
                let poster = format!("{}{}", img_default_dir, get_src(images[0]));
                scene_wrap
                    .properties
                    .push_str(&format!(" poster=\"{}\"", poster));
            }
            Tag::A(_) => {
                let img_src = get_src(images[0]);
                html.push_str(&format!("<img src=\"{img_src}\" />"));
            }
            _ => {}
        }
    }

    if style_assigned && scene_wrap.tag == Tag::None {
        scene_wrap.tag = Tag::Span;
    }

    // If there are multiple images, turn it into a grid of images
    if img_count > 1 {
        scene_wrap.tag = Tag::Div;
        scene_wrap.style.push_str(&format!(
            "display:flex;flex-wrap:wrap;justify-content:center;"
        ));
        let img_resize = 100.0 / f32::sqrt(img_count as f32);
        for node in images {
            let img = get_src(node);
            html.push_str(&format!(
                "<img src=\"{img}\" style=\"width:{img_resize}%;height:{img_resize}%;\"/>"
            ));
        }
    }

    let mut scenehead_literals: Vec<(AstNode, usize)> = Vec::new();
    let mut scenehead_templates: Vec<usize> = Vec::new();

    for node in scene {
        match node {
            AstNode::Element(token) => {
                match token {
                    Token::Span(mut content) => {
                        content = sanitise_content(&mut content);
                        match *parent_tag {
                            Tag::P => {
                                html.push_str(&format!("<span>{}</span>", content));
                                if count_newlines_at_end_of_string(&content) > 0 {
                                    *parent_tag = Tag::None;
                                    html.push_str("</p>");

                                    // Find the last p tag in closing tags and remove it
                                    let mut i = closing_tags.len();
                                    while i > 0 {
                                        i -= 1;
                                        if closing_tags[i] == "</p>" {
                                            closing_tags.remove(i);
                                            break;
                                        }
                                    }
                                }
                            }
                            Tag::Heading | Tag::BulletPoint => {
                                let newlines_at_start = count_newlines_at_start_of_string(&content);
                                if newlines_at_start > 0 {
                                    // If newlines at start, break out of heading and add normal P tag instead
                                    html.push_str(&format!("<p>{}", content));
                                    closing_tags.push("</p>".to_string());
                                    *parent_tag = Tag::None;
                                } else {
                                    html.push_str(&content);
                                    if count_newlines_at_end_of_string(&content) > 0 {
                                        html.push_str(&collect_closing_tags(&mut closing_tags));
                                        *parent_tag = Tag::None;
                                    }
                                }
                            }
                            _ => {
                                html.push_str(&format!("<span>{}</span>", content));
                            }
                        }
                    }

                    Token::P(mut content) => {
                        content = sanitise_content(&mut content);

                        match scene_wrap.tag {
                            Tag::Img(_) | Tag::Video(_) => {
                                scene_wrap
                                    .properties
                                    .push_str(&format!(" alt=\"{}\"", content));
                            }
                            Tag::A(_) => {
                                html.push_str(&collect_closing_tags(&mut closing_tags));
                                html.push_str(&content.to_owned());
                            }
                            _ => {
                                html.push_str(&collect_closing_tags(&mut closing_tags));
                                match *parent_tag {
                                    Tag::P => {
                                        if count_newlines_at_start_of_string(content.as_str()) > 1 {
                                            html.push_str("</p>");
                                            html.push_str(&format!("<p>{}", content));
                                        } else {
                                            html.push_str(&format!("<span>{}</span>", content));
                                        }
                                    }
                                    Tag::Table(_) | Tag::Nav(_) => {
                                        html.push_str(&format!("<span>{}</span>", content));
                                    }
                                    Tag::Heading | Tag::BulletPoint => {
                                        let newlines_at_start =
                                            count_newlines_at_start_of_string(content.as_str());
                                        if newlines_at_start > 0 {
                                            for _ in 1..newlines_at_start {
                                                html.push_str("<br>");
                                            }
                                            html.push_str(&format!("<p>{}", content));
                                            closing_tags.push("</p>".to_string());
                                            *parent_tag = Tag::P;
                                        } else {
                                            html.push_str(&format!("{}", content));
                                            if count_newlines_at_end_of_string(content.as_str()) > 0
                                            {
                                                html.push_str(&collect_closing_tags(
                                                    &mut closing_tags,
                                                ));
                                                *parent_tag = Tag::None;
                                            }
                                        }
                                    }
                                    _ => {
                                        html.push_str(&format!("<p>{}", content));
                                        if count_newlines_at_end_of_string(&content) > 0 {
                                            html.push_str("</p>");
                                            *parent_tag = Tag::None;
                                        } else {
                                            closing_tags.push("</p>".to_string());
                                            *parent_tag = Tag::P;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Token::Pre(content) => {
                        html.push_str(&collect_closing_tags(&mut closing_tags));
                        html.push_str(&format!("<pre>{}", content));
                        closing_tags.push("</pre>".to_string());
                    }

                    Token::Newline => {
                        html.push_str(&collect_closing_tags(&mut closing_tags));
                        if columns == 0 {
                            html.push_str("<br>");
                        }
                    }

                    Token::CodeBlock(content) => {
                        html.push_str(&collect_closing_tags(&mut closing_tags));
                        html.push_str(&format!("<pre><code>{}</code></pre>", content));
                    }
                    _ => {
                        red_ln!("Unknown token found in AST Element node: {:?}", token);
                    }
                };
            }

            AstNode::Scene(new_scene_nodes, new_scene_tags, new_scene_styles) => {
                // Switch scene tag for certain child scenes
                let mut new_scene_tag = match scene_wrap.tag {
                    Tag::Nav(_) => Tag::List,
                    _ => scene_wrap.tag.to_owned(),
                };

                let new_scene = parse_scene(
                    new_scene_nodes,
                    new_scene_tags,
                    new_scene_styles,
                    &mut new_scene_tag,
                    js,
                    css,
                    module_references,
                    class_id,
                    exp_id,
                    &mut Vec::new(),
                );

                // If this is in a list, surroung the scene with list tags
                let list_item_closing_tag = match scene_wrap.outer_tag {
                    Tag::List => {
                        html.push_str("<li>");
                        "</li>"
                    }
                    _ => "",
                };
                // If this is in a table, add correct table tags
                // What happens if columns are 0?
                match scene_wrap.tag {
                    Tag::Table(_) => {
                        insert_into_table(&new_scene, &mut ele_count, columns, &mut html);
                    }
                    Tag::Nav(_) => {
                        html.push_str(&format!("<ul>{}</ul>", new_scene));
                        ele_count += 1;
                    }
                    _ => {
                        html.push_str(&new_scene);
                    }
                }

                html.push_str(list_item_closing_tag);
            }

            // Special Markdown Syntax Elements
            AstNode::Heading(size) => {
                html.push_str(&collect_closing_tags(&mut closing_tags));
                html.push_str(&format!("<h{}>", size));
                closing_tags.push(format!("</h{}>", size));
                *parent_tag = Tag::Heading;
            }
            AstNode::BulletPoint(_strength) => {
                html.push_str(&collect_closing_tags(&mut closing_tags));
                html.push_str(&format!("<li>"));
                closing_tags.push("</li>".to_string());
                *parent_tag = Tag::None;
            }
            AstNode::Em(strength, content) => {
                if *parent_tag != Tag::P {
                    html.push_str(&collect_closing_tags(&mut closing_tags));
                    html.push_str("<p>");
                    closing_tags.push("</p>".to_string());
                    *parent_tag = Tag::P;
                }
                match strength {
                    1 => {
                        html.push_str(&format!("<em>{}</em>", content));
                    }
                    2 => {
                        html.push_str(&format!("<strong>{}</strong>", content));
                    }
                    3 => {
                        html.push_str(&format!("<strong><em>{}</em></strong>", content));
                    }
                    _ => {
                        html.push_str(&format!("<b><strong><em>{}</em></strong></b>", content));
                    }
                }
            }

            AstNode::Superscript(content) => {
                html.push_str(&format!("<sup>{}</sup>", content));
                *parent_tag = Tag::None;
                // TODO
                red_ln!("Superscript not yet supported in HTML output");
            }

            AstNode::Space => {
                html.push_str("&nbsp;");
            }

            // STUFF THAT IS INSIDE SCENE HEAD THAT NEEDS TO BE PASSED INTO SCENE BODY
            AstNode::VarReference(value) => {
                // Create a span in the HTML with a class that can be referenced by JS
                // TO DO: Should be reactive in future
                html.push_str(&format!("<span class=\"c{value}\"></span>"));

                if !module_references.contains(&node) {
                    js.push_str(&format!("uInnerHTML(\"c{value}\", v{value});"));
                    module_references.push(node);
                }
            }

            AstNode::ConstReference(value) => {
                // Make sure the const is in the module references
                let var = module_references.into_iter().rev().find(|n| match n {
                    AstNode::Const(id, _, _) => *id == value,
                    _ => false,
                });

                // Constant folding not yet implemented, so just check if there is a literal rather than expression
                match var {
                    Some(AstNode::Const(_, ref expr, _)) => {
                        let node = *expr.clone();
                        match node {
                            AstNode::Literal(token) => {
                                let value;
                                match token {
                                    Token::StringLiteral(v) | Token::RawStringLiteral(v) => {
                                        value = v;
                                    }
                                    Token::FloatLiteral(v) => {
                                        value = v.to_string();
                                    }
                                    Token::IntLiteral(v) => {
                                        value = v.to_string();
                                    }
                                    _ => {
                                        red_ln!("Unsupported constant type (should be implimented in future: {:?}", token);
                                        value = String::new();
                                    }
                                }

                                html.push_str(&format!("<span>{}</span>", value));
                            }
                            _ => {
                                red_ln!("Const was not folded (was an expression)")
                            }
                        }
                    }
                    _ => red_ln!("Const reference not found in module"),
                }
            }

            // WILL CALL WASM FUNCTIONS
            AstNode::RuntimeExpression(expr, expr_type) => {
                scenehead_literals.push((AstNode::RuntimeExpression(expr, expr_type), html.len()));
            }

            AstNode::Tuple(items) => {
                for item in items {
                    scenehead_literals.push((item, html.len()));
                }
            }

            AstNode::Literal(token) => {
                scenehead_literals.push((AstNode::Literal(token), html.len()));
            }

            AstNode::SceneTemplate => {
                if columns > 0 {
                    scenehead_templates.push(insert_into_table(
                        &String::new(),
                        &mut ele_count,
                        columns,
                        &mut html,
                    ));
                } else {
                    scenehead_templates.push(html.len());
                }
            }

            // ERROR HANDLING
            AstNode::Error(value) => {
                red_ln!("Error: {}", value);
            }
            _ => {
                red_ln!("unknown AST node found in scene: {:?}", node);
            }
        }
    }

    for tag in closing_tags.iter().rev() {
        html.push_str(tag);
    }

    // Take all scenehead variables and add them into any templates inside of the scene body
    // When there are no templates left, create a new span element to hold the literal
    for literal in scenehead_literals.into_iter().rev() {
        let mut js_string = String::new();

        match literal.0 {
            AstNode::RuntimeExpression(expr, expr_type) => {
                js_string = expression_to_js(&AstNode::RuntimeExpression(expr, expr_type));
            }
            AstNode::Literal(token) => match token {
                Token::StringLiteral(value) | Token::RawStringLiteral(value) => {
                    js_string = format!("'{}'", value);
                }
                Token::RuneLiteral(char) => {
                    js_string = format!("'{}'", char);
                }
                Token::IntLiteral(value) => {
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
            },
            _ => {
                red_ln!("Scene Head literal not yet supported in scene body");
            }
        }

        // If there are templates inside the scene, use that index.
        // Otherwise just use the index of where the literal would be inserted.
        let index = scenehead_templates.pop().unwrap_or(literal.1);
        html.insert_str(index, &format!("<span id=\"exp{exp_id}\"></span>"));

        js.push_str(&format!(
            "document.getElementById('exp{exp_id}').innerHTML={js_string};"
        ));

        *exp_id += 1;
    }

    // Create class for all child elements
    if !scene_wrap.child_styles.is_empty() {
        scene_wrap
            .properties
            .push_str(&format!(" class='bs-{class_id}'"));
        css.push_str(&format!(
            ".bs-{class_id} > * {{{}}}",
            scene_wrap.child_styles
        ));
        *class_id += 1;
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
            if match *parent_tag {
                Tag::P => true,
                _ => false,
            } {
                html.insert_str(0, "</p>");
                *parent_tag = Tag::None;
            }
            html.push_str("</div>");
        }
        Tag::A(href) => {
            html.insert_str(
                0,
                &format!(
                    "<a href={} style=\"{}\" {}>",
                    expression_to_js(&href),
                    scene_wrap.style,
                    scene_wrap.properties
                ),
            );
            html.push_str("</a>");
        }
        Tag::Img(src) => {
            html.insert_str(
                0,
                &format!(
                    "<img src={} style=\"{}\" {} />",
                    expression_to_js(&src),
                    scene_wrap.style,
                    scene_wrap.properties
                ),
            );
            if match *parent_tag {
                Tag::P => true,
                _ => false,
            } {
                html.insert_str(0, "</p>");
                *parent_tag = Tag::None;
            }
        }
        Tag::Table(_) => {
            // If not enough elements to fill the table, add empty cells
            let ele_mod = ele_count % columns;
            if ele_mod != 0 {
                for _ in 0..columns - ele_mod {
                    html.push_str("<td></td>");
                }
            }

            collect_closing_tags(&mut closing_tags);
            html.insert_str(
                0,
                &format!(
                    "<table style=\"{}\" {} ><head>",
                    scene_wrap.style, scene_wrap.properties,
                ),
            );
            html.push_str("</tbody></table>");
        }
        Tag::Video(src) => {
            html.insert_str(
                0,
                &format!(
                    "<video src=\"{}\" style=\"{}\" {} controls />",
                    expression_to_js(&src),
                    scene_wrap.style,
                    scene_wrap.properties
                ),
            );
            if match *parent_tag {
                Tag::P => true,
                _ => false,
            } {
                html.insert_str(0, "</p>");
                *parent_tag = Tag::None;
            }
        }
        Tag::Audio(src) => {
            html.insert_str(
                0,
                &format!(
                    "<audio src=\"{}\" style=\"{}\" {} controls />",
                    expression_to_js(&src),
                    scene_wrap.style,
                    scene_wrap.properties
                ),
            );
        }
        Tag::Code(_) => {
            html.insert_str(
                0,
                &format!(
                    "<code style=\"{}\" {} >",
                    scene_wrap.style, scene_wrap.properties,
                ),
            );
            html.push_str("</code>");
        }
        Tag::Nav(nav_style) => {
            let class_id = match nav_style {
                AstNode::Literal(Token::IntLiteral(value)) => value,
                _ => {
                    red_ln!("Error: nav style must be an integer literal, none provided");
                    0
                }
            };

            html.insert_str(
                0,
                &format!(
                    "<nav style=\"{}\" class=\"bs-nav-{}\" {} >",
                    scene_wrap.style, class_id, scene_wrap.properties,
                ),
            );

            css.push_str(get_bs_css(&format!("nav-{}", class_id)));
            html.push_str("</nav>");
        }
        _ => {}
    };

    match scene_wrap.outer_tag {
        Tag::Main => {
            html.insert_str(0, "<main class=\"container\">");
            html.push_str("</main>");
        }
        Tag::Header => {
            html.insert_str(0, "<header>");
            html.push_str("</header>");
        }
        Tag::Footer => {
            html.insert_str(0, "<footer>");
            html.push_str("</footer>");
        }
        Tag::Section => {
            html.insert_str(0, "<section>");
            html.push_str("</section>");
        }
        _ => {}
    };

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

fn get_src(value: &AstNode) -> String {
    let mut src: String = String::new();
    match value {
        AstNode::Literal(Token::StringLiteral(value)) => {
            src = value.clone();
        }
        AstNode::RuntimeExpression(expr, data_type) => {
            if *data_type == DataType::String {
                src = expression_to_js(&AstNode::RuntimeExpression(expr.clone(), DataType::String))
            } else {
                red_ln!("Error: src attribute must be a string literal (Webparser - get src)");
            }
        }
        _ => {
            red_ln!("Error: src attribute must be a string literal (Webparser - get src)");
        }
    }

    if src.starts_with("http") {
        return src;
    } else {
        return format!("{}/{}", get_html_config().image_folder_url, src);
    }
}

// Returns the index it inserted the html at
fn insert_into_table(
    inserted_html: &String,
    ele_count: &mut u32,
    columns: u32,
    html: &mut String,
) -> usize {
    *ele_count += 1;

    let heading = *ele_count < columns || columns < 2;
    let ele_mod = *ele_count % columns;

    if ele_mod == 1 {
        // if this is the first element for this row
        html.push_str("<tr>");
    }

    if heading {
        html.push_str("<th scope='col'>");
    } else {
        html.push_str("<td>");
    }

    // Need to check if need to close some tags before the end of this scene
    html.push_str(inserted_html);
    let idx = html.len();

    if heading {
        html.push_str("</th>");
    } else {
        html.push_str("</td>");
    }

    // If this is the last element for this row
    if ele_mod == 0 {
        html.push_str("</tr>");

        if *ele_count == columns {
            html.push_str("</thead><tbody>");
        }
    }

    idx
}

// Also make sure to escape reserved HTML characters and remove any empty lines
fn sanitise_content(content: &mut String) -> String {
    *content = content.replace('<', "&lt;").replace('>', "&gt;");
    content.trim_start().to_string()
}
