use colour::red_ln;

use super::{
    ast_nodes::AstNode,
    parse_expression::{check_if_arg, create_expression},
    styles::{Style, Tag},
    util::{count_newlines_at_end_of_string, count_newlines_at_start_of_string},
};
use crate::{bs_types::DataType, Token};

// Recursive function to parse scenes
pub fn new_scene(
    scene_head: &Vec<Token>,
    tokens: &Vec<Token>,
    i: &mut usize,
    ast: &Vec<AstNode>,
    token_line_number: &u32,
) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    let mut scene_tags: Vec<Tag> = Vec::new();
    let mut scene_styles: Vec<Style> = Vec::new();
    let mut scene_open: bool = true;
    let mut merge_next_p_line: bool = true;

    // Look at all the possible properties that can be added to the scene head
    let mut j = 0;
    while j < scene_head.len() {
        match &scene_head[j] {
            Token::SceneClose(spaces) => {
                for _ in 0..*spaces {
                    scene.push(AstNode::Space);
                }
                scene_open = false;
                *i -= 1;
                break;
            }

            Token::A => {
                j += 1;
                if !check_if_arg(scene_head, &mut j) {
                    continue;
                }

                let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Inferred);
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::A(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change href at runtime.
                    scene_tags.push(Tag::A(eval_arg));
                }
            }

            Token::Padding => {
                j += 1;
                let eval_arg;
                // TODO: get a default padding value
                if !check_if_arg(scene_head, &mut j) {
                    eval_arg = AstNode::Literal(Token::FloatLiteral(1.5));
                } else {
                    eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Inferred);
                }

                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Padding(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change padding at runtime.
                    scene_styles.push(Style::Padding(eval_arg));
                }
            }

            Token::Margin => {
                j += 1;
                let eval_arg;

                if !check_if_arg(scene_head, &mut j) {
                    eval_arg = AstNode::Literal(Token::FloatLiteral(2.0));
                } else {
                    // Must be inferred as it could be a tuple
                    eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Inferred);
                }

                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Margin(eval_arg));
                } else {
                    scene_styles.push(Style::Margin(eval_arg));
                    // Need to add JS DOM hooks to change margin at runtime.
                }
            }

            // For positioning inside a flex container / grid
            Token::Order => {
                j += 1;
                if !check_if_arg(scene_head, &mut j) {
                    continue;
                }
                let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Float);
                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Order(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change order at runtime.
                }
            }

            Token::BG => {
                j += 1;
                if !check_if_arg(scene_head, &mut j) {
                    continue;
                }
                // TO DO: Accept color names and hex values
                let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Inferred);
                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::BackgroundColor(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change background color at runtime.
                }
            }

            // Colours
            Token::Rgb | Token::Hsl => {
                let color_type = scene_head[j].to_owned();
                j += 1;
                if !check_if_arg(scene_head, &mut j) {
                    continue;
                }
                // TO DO: Accept color names and hex values
                let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Inferred);
                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::TextColor(eval_arg, color_type));
                } else {
                    // Need to add JS DOM hooks to change text color at runtime.
                }
            }
            Token::Red | Token::Green | Token::Blue | Token::Yellow | Token::Cyan | Token::Magenta | Token::White | Token::Black => {
                let color_type = scene_head[j].to_owned();
                j += 1;
                if check_if_arg(scene_head, &mut j) {
                    // TO DO: Accept color names and hex values
                    let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Inferred);
                    if check_if_comptime_value(&eval_arg) {
                        scene_styles.push(Style::TextColor(eval_arg, color_type));
                    } else {
                        // Need to add JS DOM hooks to change text color at runtime.
                    }
                } else {
                    scene_styles.push(Style::TextColor(AstNode::Literal(Token::FloatLiteral(0.0)), color_type));
                }
            }

            Token::Center => {
                scene_styles.push(Style::Center(false));
            }

            Token::Size => {
                j += 1;
                if check_if_arg(scene_head, &mut j) {
                    let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Inferred);
                    if check_if_comptime_value(&eval_arg) {
                        scene_styles.push(Style::Size(eval_arg));
                    } else {
                        // Need to add JS DOM hooks to change text size at runtime.
                    }
                } else {
                    red_ln!("Error: Size must have an argument");
                }
            }

            Token::Blank => {
                scene_styles.push(Style::Blank);
            }

            Token::Hide => {
                scene_styles.push(Style::Hide);
            }

            Token::Table => {
                j += 1;
                let eval_arg;
                // Default to 1 if no argument is provided
                if !check_if_arg(scene_head, &mut j) {
                    eval_arg = AstNode::Literal(Token::FloatLiteral(1.0));
                } else {
                    eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Inferred);
                }

                match eval_arg {
                    AstNode::Literal(literal_token) => match literal_token {
                        Token::FloatLiteral(value) => {
                            scene_tags.push(Tag::Table(value as u32));
                        }
                        _ => {
                            red_ln!("Incorrect arguments passed into table declaration");
                        }
                    },
                    _ => {
                        red_ln!("Incorrect arguments passed into table declaration");
                    }
                }
            }

            Token::Img => {
                j += 1;
                let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::String);
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Img(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    red_ln!("Can't add img src attribute to scene head at runtime (yet)");
                    scene_tags.push(Tag::Img(eval_arg));
                }
            }

            Token::Alt => {
                j += 1;
                let eval_arg: AstNode = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::String);
                if check_if_comptime_value(&eval_arg) {
                    match eval_arg {
                        AstNode::Literal(token) => match token {
                            Token::StringLiteral(value) => {
                                scene_styles.push(Style::Alt(value.clone()));
                            }
                            _ => {
                                scene.push(AstNode::Error(
                                    "Wrong datatype provided for alt".to_string(),
                                    token_line_number.to_owned(),
                                ));
                            }
                        },
                        _ => {
                            scene.push(AstNode::Error("No string provided for alt".to_string(),
                            token_line_number.to_owned()));
                        }
                    }
                } else {
                    // Need to add JS DOM hooks to change href at runtime.
                    red_ln!("Can't add alt attribute to scene head at runtime (yet)");
                }
            }

            Token::Video => {
                j += 1;
                let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::String);
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Video(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Video(eval_arg));
                }
            }

            Token::Audio => {
                j += 1;
                let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::String);
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Audio(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Audio(eval_arg));
                }
            }

            // Expressions to Parse
            Token::FloatLiteral(_) => {
                scene.push(
                    create_expression(scene_head, &mut j, true, &ast, token_line_number, &DataType::CoerseToString),
                );
            }

            Token::VarReference(id) => {
                scene.push(AstNode::VarReference(id.to_string()));
            }
            Token::ConstReference(id) => {
                scene.push(AstNode::ConstReference(id.to_string()));
            }

            Token::StringLiteral(_) | Token::RawStringLiteral(_) => {
                scene.push(
                    create_expression(scene_head, &mut j, true, &ast, token_line_number, &DataType::CoerseToString),
                );
            }

            Token::ParentScene => {
                // Maybe do something if parent scene should be wrapped in something?
            }

            Token::Comma | Token::Newline => {}

            Token::Ignore => {
                // Just create a comment
                // Should also clear any styles or tags in the scene
                scene_styles.clear();
                scene_tags.clear();
                while *i < tokens.len() {
                    match &tokens[*i] {
                        Token::SceneClose(_) | Token::EOF => {
                            break;
                        }
                        _ => {}
                    }
                    *i += 1;
                }
                return AstNode::Comment("Ignored Scene".to_string());
            }

            Token::CodeKeyword => {
                scene_styles.clear();
                scene_tags.clear();
            }

            Token::Nav => {
                j += 1;
                let eval_arg;
                // TODO: get a default margin value
                if !check_if_arg(scene_head, &mut j) {
                    eval_arg = AstNode::Literal(Token::FloatLiteral(0.0));
                } else {
                    eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Inferred);
                }
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Nav(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Nav(eval_arg));
                }
            }

            Token::Title => {
                j += 1;
                let eval_arg;
                // TODO: get a default margin value
                if !check_if_arg(scene_head, &mut j) {
                    eval_arg = AstNode::Literal(Token::FloatLiteral(0.0));
                } else {
                    eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::Float);
                }

                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Title(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Title(eval_arg));
                }
            }

            Token::Main => {
                scene_tags.push(Tag::Main);
            }
            Token::Header => {
                scene_tags.push(Tag::Header);
            }
            Token::Footer => {
                scene_tags.push(Tag::Footer);
            }
            Token::Section => {
                scene_tags.push(Tag::Section);
            }

            Token::Redirect => {
                j += 1;
                let eval_arg = create_expression(scene_head, &mut j, false, ast, token_line_number, &DataType::String);
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Redirect(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change src at runtime.
                    scene_tags.push(Tag::Redirect(eval_arg));
                }
            }

            Token::Whitespace => {
                // Just ignore whitespace for now
            }

            _ => {
                scene.push(AstNode::Error(format!(
                    "Invalid Token Used inside Scene Head: '{:?}'",
                    &scene_head[j]
                ), token_line_number.to_owned()));
                red_ln!(
                    "Invalid Token Used inside Scene Head: '{:?}'",
                    &scene_head[j]
                );
            }
        }

        j += 1;
    }

    while *i < tokens.len() && scene_open {
        match &tokens[*i] {
            Token::EOF => {
                break;
            }

            Token::SceneClose(spaces) => {
                for _ in 0..*spaces {
                    scene.push(AstNode::Space);
                }
                break;
            }

            Token::SceneHead(new_scenehead) => {
                let nested_scene = new_scene(&new_scenehead, tokens, i, ast, token_line_number);
                scene.push(nested_scene);
            }

            Token::P(content) => {
                scene.push(if !check_if_inline(tokens, *i, &mut merge_next_p_line) {
                    AstNode::Element(Token::P(content.clone()))
                } else {
                    AstNode::Element(Token::Span(content.clone()))
                });
            }

            // Special Markdown Syntax Elements
            Token::HeadingStart(size) => {
                merge_next_p_line = false;
                scene.push(AstNode::Heading(*size));
            }
            Token::BulletPointStart(size) => {
                merge_next_p_line = false;
                scene.push(AstNode::BulletPoint(*size));
            }
            Token::Em(size, content) => {
                scene.push(AstNode::Em(*size, content.clone()));
            }
            Token::Superscript(content) => {
                scene.push(AstNode::Superscript(content.clone()));
            }

            Token::RawStringLiteral(content) => {
                scene.push(AstNode::Element(Token::Span(content.to_string())));
            }

            Token::Pre(content) => {
                scene.push(AstNode::Element(Token::Pre(content.to_string())));
            }

            Token::CodeBlock(content) => {
                scene.push(AstNode::Element(Token::CodeBlock(content.to_string())));
            }

            // For templating values in scene heads in the body of scenes
            Token::EmptyScene(spaces) => {
                scene.push(AstNode::SceneTemplate);
                for _ in 0..*spaces {
                    scene.push(AstNode::Space);
                }
            }

            Token::Newline => {
                scene.push(AstNode::Element(Token::Newline));
            }

            Token::Empty | Token::Colon => {}

            Token::DeadVarible(name) => {
                scene.push(AstNode::Error(format!("Dead Variable used in scene. '{}' was never defined", name), token_line_number.to_owned()));
            }

            _ => {
                scene.push(AstNode::Error(format!(
                    "Invalid Syntax Used Inside scene body when creating scene node: {:?}",
                    tokens[*i]
                ), token_line_number.to_owned()));
            }
        }

        *i += 1;
    }

    AstNode::Scene(scene, scene_tags, scene_styles)
}

fn check_if_inline(tokens: &Vec<Token>, i: usize, merge_next_p_line: &mut bool) -> bool {
    // If the element itself starts with Newlines, it should not be inlined
    let current_element = &tokens[i];
    let p_newlines_to_seperate: usize = if *merge_next_p_line { 2 } else { 1 };
    match current_element {
        Token::P(content) => {
            if count_newlines_at_start_of_string(content) > 0 {
                return false;
            }
        }
        _ => {}
    }

    // Iterate back through tokens to find the last token that isn't Initialise, Scenehead or Sceneclose
    let mut previous_element = &Token::Empty;
    let mut j = i - 1;
    while j > 0 {
        match &tokens[j] {
            // Ignore these tokens
            Token::Colon | Token::SceneClose(_) => {
                j -= 1;
            }

            // Check if the previous scenehead has any tags that can be inlined
            Token::SceneHead(tags) => {
                if tags.len() > 0 {
                    match tags[0] {
                        Token::A => {
                            previous_element = &tags[0];
                            break;
                        }
                        Token::StringLiteral(_) => {
                            previous_element = &tags[0];
                            break;
                        }
                        Token::ParentScene => {
                            return false;
                        }
                        _ => {}
                    }
                }
                j -= 1;
            }

            _ => {
                previous_element = &tokens[j];
                break;
            }
        }
    }

    // If the current element is the same as the previous element
    // It doesn't have 2 newlines ending it and it can be inlined
    // Then return true
    match previous_element {
        Token::Empty | Token::Newline => false,

        Token::P(content)
        | Token::Span(content)
        | Token::Em(_, content)
        | Token::Superscript(content) => {
            if count_newlines_at_end_of_string(content) >= p_newlines_to_seperate {
                *merge_next_p_line = true;
                false
            } else {
                true
            }
        }

        Token::Pre(_) => false,

        Token::A
        | Token::StringLiteral(_)
        | Token::EmptyScene(_)
        | Token::HeadingStart(_)
        | Token::BulletPointStart(_) => true,

        _ => false,
    }
}

fn check_if_comptime_value(node: &AstNode) -> bool {
    match node {
        AstNode::Literal(_) | AstNode::ConstReference(_) => true,
        AstNode::Tuple(values, _) => {
            for value in values {
                if !check_if_comptime_value(value) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}
