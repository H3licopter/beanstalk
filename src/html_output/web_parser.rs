use std::path::Path;

use super::{
    colors::get_color,
    js_parser::{collection_to_js, create_reference_in_js, expression_to_js, function_call_to_js},
};
use crate::{
    bs_css::get_bs_css,
    bs_types::DataType,
    build::ExportedJS,
    parsers::{
        ast_nodes::AstNode,
        styles::{Action, Style, Tag},
        util::{count_newlines_at_end_of_string, count_newlines_at_start_of_string},
    },
    settings::{HTMLMeta, BS_VAR_PREFIX},
    wasm_output::wat_parser::new_wat_var,
    Token,
};
use colour::red_ln;

pub struct ParserOutput {
    pub html: String,
    pub js: String,
    pub css: String,
    pub page_title: String,
    pub exported_js: Vec<ExportedJS>,
    pub exported_css: String,
    pub wat: String,
    pub wat_globals: String,
    pub errored: bool,
}

// Parse ast into valid JS, HTML and CSS
pub fn parse(
    ast: Vec<AstNode>,
    config: &HTMLMeta,
    release_build: bool,
    module_path: &str,
    is_global: bool,
    imported_css: &String,
) -> ParserOutput {
    let mut js = String::new();
    let mut wat = String::new();
    let mut wat_global_initilisation = String::new();
    let mut html = String::new();
    let mut css = imported_css.to_owned();
    let mut page_title = String::new();
    let mut exp_id: usize = 0;
    let mut errored = false;

    let mut exported_js: Vec<ExportedJS> = Vec::new();
    let mut exported_css = String::new();
    let mut _exported_wat = String::new();

    // Keeps track of whether a reference has already been used
    // This is to prevent duplicate JS code for updating the same element
    let mut module_references: Vec<AstNode> = Vec::new();

    let mut class_id: usize = 0;

    // Parse HTML
    for node in ast {
        match node {
            // SCENES (HTML)
            AstNode::Scene(scene, scene_tags, scene_styles, scene_actions) => {
                html.push_str(&parse_scene(
                    scene,
                    scene_tags,
                    scene_styles,
                    scene_actions,
                    &mut Tag::None,
                    &mut js,
                    &mut css,
                    &mut module_references,
                    &mut class_id,
                    &mut exp_id,
                    &mut Vec::new(),
                    &mut wat,
                    config,
                ));
            }
            AstNode::Title(value) => {
                page_title = value;
            }
            AstNode::Date(_value) => {
                // Eventually a way to get date information about the page
            }

            // JAVASCRIPT / WASM
            AstNode::VarDeclaration(ref id, ref expr, is_exported, ref data_type, is_const) => {
                let assignment_keyword = if is_const { "const" } else { "let" };
                match data_type {
                    DataType::Float | DataType::Int => {
                        new_wat_var(id, expr, data_type, &mut wat, &mut wat_global_initilisation);
                    }
                    DataType::String => {
                        let var_dec = format!(
                            "{} {BS_VAR_PREFIX}{id} = {};",
                            assignment_keyword,
                            expression_to_js(&expr)
                        );
                        js.push_str(&var_dec);
                        if is_exported {
                            exported_js.push(ExportedJS {
                                js: var_dec,
                                module_path: Path::new(module_path).join(id),
                                global: is_global,
                                data_type: data_type.to_owned(),
                            });
                        }
                    }
                    DataType::Scene => {
                        let unboxed_scene = *expr.clone();

                        match unboxed_scene {
                            AstNode::Scene(scene, scene_tags, scene_styles, scene_actions) => {
                                let mut created_css = String::new();
                                let scene_to_js_string = parse_scene(
                                    scene,
                                    scene_tags,
                                    scene_styles,
                                    scene_actions,
                                    &mut Tag::None,
                                    &mut js,
                                    &mut created_css,
                                    &mut module_references,
                                    &mut class_id,
                                    &mut exp_id,
                                    &mut Vec::new(),
                                    &mut wat,
                                    config,
                                );
                                css.push_str(&created_css);

                                // If this scene is exported, add the CSS it created to the exported CSS
                                if is_exported {
                                    exported_css.push_str(&created_css);
                                }

                                let var_dec = format!(
                                    "{} {BS_VAR_PREFIX}{id} = `{}`;",
                                    assignment_keyword, scene_to_js_string
                                );
                                js.push_str(&var_dec);
                                if is_exported {
                                    exported_js.push(ExportedJS {
                                        js: var_dec,
                                        module_path: Path::new(module_path).join(id),
                                        global: is_global,
                                        data_type: data_type.to_owned(),
                                    });
                                }
                            }
                            _ => {
                                errored = true;
                                red_ln!("Error: Scene expression must be a scene in HTML parser");
                                continue;
                            }
                        };
                    }
                    DataType::Tuple(datatypes) => {
                        // Create struct to represent a tuple in JS
                        let mut tuple_js = String::from("{");
                        let mut index = 0;
                        let tuple = match &**expr {
                            AstNode::Tuple(values, _) => values,
                            _ => {
                                red_ln!("Error: Tuple declaration must be a tuple");
                                break;
                            }
                        };

                        for datatype in &**datatypes {
                            let current_tuple_item = match tuple.get(index) {
                                Some(item) => item,
                                None => {
                                    red_ln!("Error: Tuple datatype count does not match the number of items in the tuple. Tuple index: {}", index);
                                    break;
                                }
                            };

                            match datatype {
                                &DataType::Float | &DataType::Int => {
                                    new_wat_var(
                                        &format!("{id}_{index}"),
                                        current_tuple_item,
                                        datatype,
                                        &mut wat,
                                        &mut wat_global_initilisation,
                                    );
                                    tuple_js.push_str(&format!(
                                        "{}: wsx.get_{BS_VAR_PREFIX}{id}_{index}(),",
                                        index,
                                    ));
                                }
                                &DataType::String | &DataType::CoerseToString => {
                                    tuple_js.push_str(&format!(
                                        "{}: {},",
                                        index,
                                        expression_to_js(current_tuple_item)
                                    ));
                                }
                                _ => {
                                    errored = true;
                                    red_ln!(
                                        "Unsupported datatype found in tuple declaration: {:?}",
                                        datatype
                                    );
                                }
                            }
                            index += 1;
                        }

                        tuple_js.push_str("}");
                        js.push_str(&format!(
                            "{} {BS_VAR_PREFIX}{id} = {};",
                            assignment_keyword, tuple_js
                        ));
                    }
                    _ => {
                        js.push_str(&format!(
                            "{} {BS_VAR_PREFIX}{id} = {};",
                            assignment_keyword,
                            expression_to_js(&expr)
                        ));
                    }
                };

                module_references.push(node);
            }

            AstNode::Function(name, args, body, is_exported, return_type) => {
                let mut arg_names = String::new();
                for arg in &args {
                    let unboxed_default = match &arg.default_value {
                        Some(ref boxed_value) => {
                            &**boxed_value
                        }
                        _ => &AstNode::Empty,
                    };

                    let default_arg = match unboxed_default {
                        AstNode::Literal(token) => {
                            match token {
                                Token::StringLiteral(value) 
                                | Token::RawStringLiteral(value) 
                                | Token::PathLiteral(value) => {
                                    &format!("=\"{value}\"")
                                }
                                Token::IntLiteral(value) => {
                                    &format!("={value}")
                                }
                                Token::FloatLiteral(value) => {
                                    &format!("={value}")
                                }
                                Token::BoolLiteral(value) => {
                                    &format!("={value}")
                                }
                                _=> {
                                    errored = true;
                                    red_ln!("Compiler Error: invalid literal given as a default value");
                                    ""
                                }
                            }
                        }
                        _ => {
                            ""
                        }
                    };

                    arg_names.push_str(&format!("{name}{default_arg},"));

                }

                let func_body = parse(body, config, release_build, module_path, false, imported_css);
                let func = format!(
                    "{}function {BS_VAR_PREFIX}{name}({arg_names}){{{}}}",
                    if is_exported { "export " } else { "" },
                    func_body.js
                );

                if is_exported {
                    exported_js.push(ExportedJS {
                        js: func.to_owned(),
                        module_path: Path::new(module_path).join(name),
                        global: is_global,
                        data_type: DataType::Function(Box::new(args), Box::new(return_type)),
                    });
                }
                js.push_str(&func);
                wat.push_str(&func_body.wat);
                wat_global_initilisation.push_str(&func_body.wat_globals);
            }

            AstNode::FunctionCall(name, arguments, _) => {
                js.push_str(&function_call_to_js(&name, *arguments.to_owned()));
            }

            AstNode::Return(ref expr) => {
                js.push_str(&format!("return {};", expression_to_js(&expr)));
            }
            AstNode::Print(ref expr) => {
                js.push_str(&format!("console.log({});", expression_to_js(&expr)));
            }


            // DIRECT INSERTION OF JS / CSS / HTML into page
            AstNode::JS(js_string) => {
                js.push_str(&js_string);
            }
            AstNode::CSS(css_string) => {
                css.push_str(&css_string);
            }

            // Ignored
            AstNode::Comment(_) => {}

            AstNode::Error(err, line_number) => {
                errored = true;
                red_ln!("Error on Line {}: - {}", line_number, err);
            }
            _ => {
                errored = true;
                red_ln!(
                    "unknown AST node found when parsing AST in web parser: {:?}",
                    node
                );
            }
        }
    }

    if config.auto_site_title {
        page_title += &(" | ".to_owned() + &config.site_title.clone());
    }

    ParserOutput {
        html,
        js,
        css,
        page_title,
        exported_js,
        exported_css,
        wat,
        wat_globals: wat_global_initilisation,
        errored,
    }
}

struct SceneTag {
    tag: Tag,
    outer_tag: Tag,
    properties: String,
    style: String,
    classes: String,
    child_styles: String,
}

// Returns a string of the HTML and the tag the scene is inside of
pub fn parse_scene(
    scene: Vec<AstNode>,
    scene_tags: Vec<Tag>,
    scene_styles: Vec<Style>,
    scene_actions: Vec<Action>,
    parent_tag: &mut Tag,
    js: &mut String,
    css: &mut String,
    module_references: &mut Vec<AstNode>,
    class_id: &mut usize,
    exp_id: &mut usize,
    positions: &mut Vec<f64>,
    wasm_module: &mut String,
    config: &HTMLMeta,
) -> String {
    let mut html = String::new();
    let mut closing_tags = Vec::new();
    let mut codeblock_css_added = false;
    let mut content_size = 1.0;
    let mut text_is_size = true;

    let mut spaces_after_closing_tag = 0;

    // For tables
    let mut ele_count: u32 = 0;
    let mut columns: u32 = 0;

    let mut images: Vec<&AstNode> = Vec::new();
    let mut scene_wrap = SceneTag {
        tag: match parent_tag {
            Tag::List => Tag::List,
            _ => Tag::None,
        },
        outer_tag: match parent_tag {
            Tag::List => Tag::List,
            _ => Tag::None,
        },
        properties: String::new(),
        classes: String::new(),
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
                    AstNode::Literal(Token::FloatLiteral(value)) => {
                        scene_wrap.style.push_str(&format!("padding:{}rem;", value));
                    }
                    AstNode::Literal(Token::IntLiteral(value)) => {
                        scene_wrap.style.push_str(&format!("padding:{}rem;", value));
                    }
                    AstNode::Tuple(values, line_number) => {
                        let mut padding = String::new();
                        for value in values {
                            match value {
                                AstNode::Literal(Token::FloatLiteral(value)) => {
                                    padding.push_str(&format!("{}rem ", value));
                                }
                                AstNode::Literal(Token::IntLiteral(value)) => {
                                    padding.push_str(&format!("{}rem ", value));
                                }
                                _ => {
                                    red_ln!(
                                        "Error at line {}: Padding must be a literal or a tuple of literals",
                                        line_number
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
                style_assigned = true;
            }
            Style::BackgroundColor(args) => {
                scene_wrap.style.push_str(&format!(
                    "background-color:rgba({});",
                    collection_to_js(&args)
                ));
                style_assigned = true;
            }
            Style::TextColor(args, type_of_color) => {
                let color = match type_of_color {
                    Token::Rgb => format!("rgba({})", collection_to_js(&args)),
                    Token::Hsl => format!("hsla({})", collection_to_js(&args)),

                    Token::Red
                    | Token::Green
                    | Token::Blue
                    | Token::Yellow
                    | Token::Cyan
                    | Token::Magenta
                    | Token::White
                    | Token::Black
                    | Token::Orange
                    | Token::Pink
                    | Token::Purple
                    | Token::Grey => {
                        format!("hsla({})", get_color(&type_of_color, &args))
                    }
                    _ => {
                        red_ln!(
                            "Error: Invalid color type provided for text color: {:?}",
                            type_of_color
                        );
                        continue;
                    }
                };

                scene_wrap.style.push_str(&format!("color:{};", color,));
                scene_wrap.child_styles.push_str("color:inherit;");
                style_assigned = true;
            }
            Style::Size(node) => {
                content_size = match node {
                    AstNode::Literal(token) => match token {
                        Token::FloatLiteral(value) => value,
                        Token::IntLiteral(value) => value as f64,
                        _ => {
                            red_ln!("Error: Size argument was not numeric");
                            continue;
                        }
                    },
                    _ => {
                        red_ln!("Error: Size must be a literal");
                        continue;
                    }
                };
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
                let mut order = 0.0;
                match node {
                    AstNode::Literal(token) => match token {
                        Token::FloatLiteral(value) => {
                            order = value;
                        }
                        Token::IntLiteral(value) => {
                            order = value as f64;
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
            Style::Blank => {
                scene_wrap.style.push_str("all:unset;");
                style_assigned = true;
            }
            Style::Hide => {
                scene_wrap.style.push_str("display:none;");
                style_assigned = true;
            }
        }
    }

    let mut img_count = 0;

    // Scene tags usually override each other, only the last one will actually be used
    // There may be some exceptions
    for tag in &scene_tags {
        match tag {
            Tag::Img(value) => {
                text_is_size = false;
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
                text_is_size = false;
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
                text_is_size = false;
                scene_wrap.tag = Tag::Audio(src.to_owned());
            }
            Tag::A(node) => {
                scene_wrap.tag = Tag::A(node.to_owned());
            }
            Tag::Title(node) => {
                scene_wrap.tag = Tag::Title(node.to_owned());
            }
            Tag::Nav(style) => {
                scene_wrap.tag = Tag::Nav(style.to_owned());
            }

            // Interactive
            Tag::Button(node) => {
                text_is_size = false;
                scene_wrap.tag = Tag::Button(node.to_owned());
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

            // Scripts
            Tag::Redirect(node) => {
                let src = match node {
                    AstNode::Literal(Token::StringLiteral(value)) => value,
                    AstNode::RuntimeExpression(expr, data_type) => {
                        if *data_type == DataType::String {
                            &expression_to_js(&AstNode::RuntimeExpression(
                                expr.clone(),
                                DataType::String,
                            ))
                        } else {
                            red_ln!("Error: src attribute must be a string literal (Webparser - get src)");
                            continue;
                        }
                    }
                    _ => {
                        red_ln!(
                            "Error: src attribute must be a string literal (Webparser - get src)"
                        );
                        continue;
                    }
                };
                js.push_str(&format!("window.location.href='{}';", src));
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
                let poster = get_src(images[0], config);
                scene_wrap
                    .properties
                    .push_str(&format!(" poster=\"{}\"", poster));
            }
            Tag::A(_) => {
                let img_src = get_src(images[0], config);
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
        let img_resize = (content_size * 100.0) / f64::sqrt(img_count as f64);
        for node in images {
            let img = get_src(node, config);
            html.push_str(&format!(
                "<img src=\"{img}\" style=\"width:{img_resize}%;height:{img_resize}%;\"/>"
            ));
        }
    }

    // Add any actions that need to be added to the scene
    for action in scene_actions {
        match action {
            Action::Click(node) => {
                // Should accept a function as an argument
                scene_wrap
                    .properties
                    .push_str(&format!(" onclick=\"{}\"", expression_to_js(&node)));
            }
            Action::_Swap => {}
        }
    }

    if content_size != 1.0 {
        if text_is_size {
            scene_wrap
                .style
                .push_str(&format!("font-size:{}rem;", content_size));
        } else {
            let size = content_size * 100.0;
            scene_wrap.style.push_str(&format!("width:{size}%;"));
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

                        // Specical tags
                        match scene_wrap.tag {
                            Tag::Title(_) | Tag::List | Tag::A(_) | Tag::Button(_) => {
                                html.push_str(&content.to_owned());
                                continue;
                            }
                            _ => {}
                        }

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
                            Tag::Table(_) | Tag::List | Tag::A(_) | Tag::Button(_) => {
                                html.push_str(&content.to_owned());
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
                            Tag::Title(_) | Tag::List => {
                                html.push_str(&content.to_owned());
                                continue;
                            }
                            Tag::A(_) | Tag::Button(_) => {
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
                                    Tag::Table(_) | Tag::Nav(_) | Tag::List | Tag::Button(_) => {
                                        html.push_str(&content.to_owned());
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
                                            html.push_str(&content.to_owned());
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
                        match *parent_tag {
                            Tag::Table(_) | Tag::Nav(_) => {}
                            _ => {
                                html.push_str(&collect_closing_tags(&mut closing_tags));
                                // if columns == 0 {
                                //     html.push_str("<br>");
                                // }
                            }
                        };
                    }

                    Token::CodeBlock(content) => {
                        // Add the CSS for code highlighting
                        if !codeblock_css_added {
                            // css.push_str(&get_highlight_css());
                            codeblock_css_added = true;
                        }

                        html.push_str(&collect_closing_tags(&mut closing_tags));
                        html.push_str(&format!("<pre><code>{}</code></pre>", content));
                    }
                    _ => {
                        red_ln!("Unknown token found in AST Element node: {:?}", token);
                    }
                };
            }

            AstNode::Scene(
                new_scene_nodes,
                new_scene_tags,
                new_scene_styles,
                new_scene_actions,
            ) => {
                // Switch scene tag for certain child scenes
                let mut new_scene_tag = match scene_wrap.tag {
                    Tag::Nav(_) => Tag::List,
                    Tag::List => Tag::None,
                    _ => scene_wrap.tag.to_owned(),
                };

                let new_scene = parse_scene(
                    new_scene_nodes,
                    new_scene_tags,
                    new_scene_styles,
                    new_scene_actions,
                    &mut new_scene_tag,
                    js,
                    css,
                    module_references,
                    class_id,
                    exp_id,
                    &mut Vec::new(),
                    wasm_module,
                    config,
                );

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
            }

            // Special Markdown Syntax Elements
            AstNode::Heading(size) => {
                match *parent_tag {
                    Tag::Table(_) | Tag::Nav(_) => {}
                    _ => {
                        html.push_str(&collect_closing_tags(&mut closing_tags));
                        html.push_str(&format!("<h{}>", size));
                        closing_tags.push(format!("</h{}>", size));
                        *parent_tag = Tag::Heading;
                    }
                };
            }
            AstNode::BulletPoint(_strength) => {
                match *parent_tag {
                    Tag::Table(_) | Tag::Nav(_) => {}
                    _ => {
                        html.push_str(&collect_closing_tags(&mut closing_tags));
                        html.push_str(&format!("<li>"));
                        closing_tags.push("</li>".to_string());
                        *parent_tag = Tag::None;
                    }
                };
            }
            AstNode::Em(strength, content) => {
                match *parent_tag {
                    Tag::Table(_) | Tag::Nav(_) | Tag::P => {}
                    _ => {
                        html.push_str(&collect_closing_tags(&mut closing_tags));
                        html.push_str("<p>");
                        closing_tags.push("</p>".to_string());
                        *parent_tag = Tag::P;
                    }
                };

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
                spaces_after_closing_tag += 1;
            }

            // STUFF THAT IS INSIDE SCENE HEAD THAT NEEDS TO BE PASSED INTO SCENE BODY
            AstNode::FunctionCall(ref name, ref arguments, _) => {
                html.push_str(&format!("<span class=\"{name}\"></span>"));
                if !module_references.contains(&node) {
                    module_references.push(node.to_owned());
                    js.push_str(&format!(
                        "uInnerHTML(\"{name}\",{});",
                        &function_call_to_js(name, *arguments.to_owned()
                    )));
                }
            }
            AstNode::VarReference(ref name, ref data_type)
            | AstNode::ConstReference(ref name, ref data_type) => {
                // Create a span in the HTML with a class that can be referenced by JS
                // TO DO: Should be reactive in future -> this can change at runtime
                html.push_str(&format!("<span class=\"{name}\"></span>"));

                if !module_references.contains(&node) {
                    module_references.push(node.to_owned());
                    match &data_type {
                        DataType::Tuple(items) => {
                            // Automatically unpack all items in the tuple into the scene
                            let mut elements = String::new();
                            let mut index = 0;
                            for _ in &**items {
                                elements.push_str(&format!("{BS_VAR_PREFIX}{name}[{index}],"));
                                index += 1;
                            }

                            js.push_str(&format!("uInnerHTML(\"{name}\", [{elements}]);"));
                        }
                        _ => {
                            js.push_str(&create_reference_in_js(name, data_type));
                        }
                    }
                }
            }

            AstNode::CollectionAccess(ref name, ref index, ref data_type)
            | AstNode::TupleAccess(ref name, ref index, ref data_type) => {
                html.push_str(&format!("<span class=\"{name}\"></span>"));

                if !module_references.contains(&node) {
                    module_references.push(node.to_owned());

                    match &data_type {
                        DataType::Tuple(items) => {
                            // Automatically unpack all items in the tuple into the scene
                            let mut elements = String::new();
                            let mut idx = 0;
                            for _ in &**items {
                                elements
                                    .push_str(&format!("{BS_VAR_PREFIX}{name}[{index}][{idx}],"));
                                idx += 1;
                            }

                            js.push_str(&format!("uInnerHTML(\"{name}\", [{elements}]);"));
                        }
                        _ => {
                            js.push_str(&create_reference_in_js(name, data_type));
                        }
                    }
                }
            }

            AstNode::RuntimeExpression(expr, expr_type) => {
                scenehead_literals.push((AstNode::RuntimeExpression(expr, expr_type), html.len()));
            }

            AstNode::Tuple(items, _) => {
                for item in items {
                    scenehead_literals.push((item, html.len()));
                }
            }

            AstNode::Literal(token) => {
                // Check if this is accessing a tuple
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
            AstNode::Error(value, line_number) => {
                red_ln!("Error at line {} in scene: {} ", line_number, value);
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
                    js_string = format!("\"{}\"", value);
                }
                Token::FloatLiteral(value) => {
                    js_string = value.to_string();
                }
                Token::IntLiteral(value) => {
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
        scene_wrap.classes.push_str(&format!(" bs-{class_id}"));
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
                    "<span style=\"{}\" {} class=\"{}\" >",
                    scene_wrap.style, scene_wrap.properties, scene_wrap.classes
                ),
            );
            html.push_str("</span>");
        }
        Tag::Div => {
            html.insert_str(
                0,
                &format!(
                    "<div style=\"{}\" {} class=\"{}\" >",
                    scene_wrap.style, scene_wrap.properties, scene_wrap.classes
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
                    "<a href={} style=\"{}\" class=\"{}\" {}>",
                    expression_to_js(&href),
                    scene_wrap.style,
                    scene_wrap.classes,
                    scene_wrap.properties
                ),
            );
            html.push_str("</a>");
        }
        Tag::Button(button) => {
            html.insert_str(
                0,
                &format!(
                    "<button onclick=\"{}\" style=\"{}\" class=\"{}\" {}>",
                    expression_to_js(&button),
                    scene_wrap.style,
                    scene_wrap.classes,
                    scene_wrap.properties
                ),
            );
            html.push_str("</button>");
        }
        Tag::Img(src) => {
            let img_src = get_src(&src, config);
            html.insert_str(
                0,
                &format!(
                    "<img src={} style=\"{}\" class=\"{}\" {} />",
                    img_src, scene_wrap.style, scene_wrap.classes, scene_wrap.properties
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
                    "<table style=\"{}\" {} class=\"{}\" ><thead>",
                    scene_wrap.style, scene_wrap.properties, scene_wrap.classes,
                ),
            );
            html.push_str("</tbody></table>");
        }
        Tag::Video(src) => {
            html.insert_str(
                0,
                &format!(
                    "<video src=\"{}\" style=\"{}\" {} class=\"{}\" controls />",
                    expression_to_js(&src),
                    scene_wrap.style,
                    scene_wrap.properties,
                    scene_wrap.classes
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
                    "<audio src=\"{}\" style=\"{}\" {} class=\"{}\" controls />",
                    expression_to_js(&src),
                    scene_wrap.style,
                    scene_wrap.properties,
                    scene_wrap.classes
                ),
            );
        }
        Tag::Code(_) => {
            html.insert_str(
                0,
                &format!(
                    "<code style=\"{}\" {} class=\"{}\" >",
                    scene_wrap.style, scene_wrap.properties, scene_wrap.classes,
                ),
            );
            html.push_str("</code>");
        }
        Tag::Nav(nav_style) => {
            let class_id = match nav_style {
                AstNode::Literal(Token::FloatLiteral(value)) => value,
                AstNode::Literal(Token::IntLiteral(value)) => value as f64,
                _ => {
                    red_ln!("Error: nav style must be an integer literal, none provided");
                    0.0
                }
            };

            html.insert_str(
                0,
                &format!(
                    "<nav style=\"{}\" class=\"bs-nav-{} {}\" {} >",
                    scene_wrap.style, class_id, scene_wrap.classes, scene_wrap.properties,
                ),
            );

            css.push_str(get_bs_css(&format!("nav-{}", class_id)));
            html.push_str("</nav>");
        }
        Tag::Title(size) => {
            let class_id = match size {
                AstNode::Literal(Token::FloatLiteral(value)) => value,
                AstNode::Literal(Token::IntLiteral(value)) => value as f64,
                _ => {
                    red_ln!("Error: title size must be an integer literal, none provided");
                    0.0
                }
            };
            html.insert_str(
                0,
                &format!(
                    "<b class=\"bs-title-{} {}\" style=\"{}\" {} >",
                    class_id, scene_wrap.classes, scene_wrap.style, scene_wrap.properties,
                ),
            );
            css.push_str(get_bs_css(&format!("title-{}", class_id)));
            html.push_str("</b>");
        }
        _ => {}
    };

    for _ in 0..spaces_after_closing_tag {
        html.push_str("&nbsp;");
    }

    match scene_wrap.outer_tag {
        Tag::Main => {
            html.insert_str(0, "<main class=\"container\">");
            html.push_str("</main>");
        }
        Tag::Header => {
            html.insert_str(0, "<header class=\"container\">");
            html.push_str("</header>");
        }
        Tag::Footer => {
            html.insert_str(0, "<footer class=\"container\">");
            html.push_str("</footer>");
        }
        Tag::Section => {
            html.insert_str(0, "<section>");
            html.push_str("</section>");
        }
        Tag::List => {
            html.insert_str(0, "<li>");
            html.push_str("</li>");
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

fn get_src(value: &AstNode, config: &HTMLMeta) -> String {
    let mut src: String = String::new();
    match value {
        AstNode::Literal(literal) => {
            match literal {
                Token::StringLiteral(value) => {
                    src = value.clone();
                }
                Token::PathLiteral(value) => {
                    // Replace slashes with correct platform OS
                    // for now just make an http link
                    return format!("https://{}", value);
                }
                _ => {
                    red_ln!("Error: src attribute must be a string literal (Webparser - get src)");
                }
            }
        }
        AstNode::RuntimeExpression(expr, data_type) => {
            if *data_type == DataType::String || *data_type == DataType::CoerseToString {
                src = expression_to_js(&AstNode::RuntimeExpression(
                    expr.clone(),
                    data_type.to_owned(),
                ))
            } else {
                red_ln!("Error: src attribute must be a string literal (Webparser - get src)");
            }
        }
        _ => {
            red_ln!("Error: src attribute must be a string literal (Webparser - get src)");
        }
    }

    if src.starts_with("http") || src.starts_with("/") {
        return src;
    } else {
        return format!(
            "{}{}/{}",
            config.page_root_url, config.image_folder_url, src
        );
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

    let heading = *ele_count <= columns || columns < 2;
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