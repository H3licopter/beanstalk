use super::{
    ast::AstNode,
    parse_expression::{create_expression, eval_expression},
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
) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    let mut scene_tags: Vec<Tag> = Vec::new();
    let mut scene_styles: Vec<Style> = Vec::new();
    let mut scene_open: bool = true;

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
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::String, ast);
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::A(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change href at runtime.
                    scene_tags.push(Tag::A(eval_arg));
                }
            }

            Token::Padding => {
                j += 1;
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::Inferred, ast);

                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Padding(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change padding at runtime.
                    scene_styles.push(Style::Padding(eval_arg));
                }
            }

            Token::Margin => {
                j += 1;
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::Inferred, ast);
                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Margin(eval_arg));
                } else {
                    scene_styles.push(Style::Margin(eval_arg));
                    // Need to add JS DOM hooks to change margin at runtime.
                }
            }

            Token::BG => {
                j += 1;
                // TO DO: Accept color names and hex values
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::Inferred, ast);
                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::BackgroundColor(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change background color at runtime.
                }
            }

            Token::Rgb => {
                j += 1;
                // TO DO: Accept color names and hex values
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::Inferred, ast);
                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::TextColor(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change text color at runtime.
                }
            }

            Token::Center => {
                scene_styles.push(Style::Center(false));
            }

            Token::Size => {
                j += 1;
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::Inferred, ast);
                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Size(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change text size at runtime.
                }
            }

            Token::Table => {
                j += 1;
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::Inferred, ast);

                match eval_arg {
                    AstNode::Literal(literal_token) => match literal_token {
                        Token::IntLiteral(value) => {
                            scene_tags.push(Tag::Table(value as u32));
                        }
                        Token::FloatLiteral(value) => {
                            scene_tags.push(Tag::Table(value as u32));
                        }
                        _ => {
                            println!("Incorrect arguments passed into table declaration");
                        }
                    },
                    _ => {
                        println!("Incorrect arguments passed into table declaration");
                    }
                }
            }

            Token::Img => {
                j += 1;
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::String, ast);
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Img(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Img(eval_arg));
                }
            }

            Token::Alt => {
                j += 1;
                match &scene_head[j] {
                    Token::StringLiteral(value) => {
                        scene_styles.push(Style::Alt(value.clone()));
                    }
                    _ => {
                        scene.push(AstNode::Error("No string provided for alt".to_string()));
                    }
                };
            }

            Token::Video => {
                j += 1;
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::String, ast);
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Video(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Video(eval_arg));
                }
            }

            Token::Audio => {
                j += 1;
                let arg = create_expression(scene_head, &mut j, false, ast);
                let eval_arg = eval_expression(arg, &DataType::String, ast);
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Audio(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Audio(eval_arg));
                }
            }

            // Expressions to Parse
            Token::FloatLiteral(_) | Token::IntLiteral(_) | Token::DecLiteral(_) => {
                scene.push(eval_expression(
                    create_expression(scene_head, &mut j, true, &ast),
                    &DataType::CoerseToString,
                    &ast,
                ));
            }

            Token::VarReference(name) => {
                scene.push(AstNode::VarReference(*name));
            }
            Token::ConstReference(name) => {
                scene.push(AstNode::ConstReference(*name));
            }

            Token::StringLiteral(_) | Token::RawStringLiteral(_) => {
                scene.push(eval_expression(
                    create_expression(scene_head, &mut j, true, &ast),
                    &DataType::CoerseToString,
                    &ast,
                ));
            }

            Token::ParentScene => {
                // Useful for wrapping the scene in <main> or unwrapped if it's a component
            }

            Token::Comma | Token::Newline => {}

            Token::Ignore => {
                // Just create a comment
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

            _ => {
                scene.push(AstNode::Error(format!(
                    "Invalid Token Used inside Scene Head: '{:?}'",
                    &scene_head[j]
                )));
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
                let nested_scene = new_scene(&new_scenehead, tokens, i, ast);
                scene.push(nested_scene);
            }

            Token::P(content) => {
                scene.push(if !check_if_inline(tokens, *i) {
                    AstNode::Element(Token::P(content.clone()))
                } else {
                    AstNode::Element(Token::Span(content.clone()))
                });
            }

            Token::Heading(size, content) => {
                scene.push(if !check_if_inline(tokens, *i) {
                    AstNode::Element(Token::Heading(*size, content.clone()))
                } else {
                    AstNode::Element(Token::Span(content.clone()))
                });
            }

            Token::BulletPoint(indentation, content) => {
                scene.push(if !check_if_inline(tokens, *i) {
                    AstNode::Element(Token::BulletPoint(
                        *indentation,
                        content.clone(),
                    ))
                } else {
                    AstNode::Element(Token::Span(content.clone()))
                });
            }

            Token::Superscript(content) => {
                scene.push(AstNode::Element(Token::Superscript(content.clone())));
            }

            Token::RawStringLiteral(content) => {
                scene.push(AstNode::Element(Token::Span(content.to_string())));
            }

            Token::Pre(content) => {
                scene.push(AstNode::Element(Token::Pre(content.to_string())));
            }

            // For templating values in scene heads in the body of scenes
            Token::EmptyScene(spaces) => {
                scene.push(AstNode::SceneTemplate);
                for _ in 0..*spaces {
                    scene.push(AstNode::Space);
                }
            }

            Token::Empty | Token::Newline | Token::AssignComptime | Token::DeadVarible => {}

            _ => {
                scene.push(AstNode::Error(
                    format!("Invalid Syntax Used Inside scene body when creating scene node: {:?}", tokens[*i]),
                ));
            }
        }

        *i += 1;
    }

    AstNode::Scene(scene, scene_tags, scene_styles)
}

fn check_if_inline(tokens: &Vec<Token>, i: usize) -> bool {
    // If the element itself starts with Newlines, it should not be inlined
    match &tokens[i] {
        Token::P(content) => {
            if count_newlines_at_start_of_string(content) > 0 {
                return false;
            }
        }
        Token::Heading(_, content) => {
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
            Token::AssignComptime | Token::SceneClose(_) => {
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
        Token::Empty => false,

        Token::P(content) | Token::Span(content) => {
            if count_newlines_at_end_of_string(content) > 1 {
                false
            } else {
                true
            }
        }

        Token::Heading(_, content) => {
            if count_newlines_at_end_of_string(content) > 0 {
                false
            } else {
                true
            }
        }

        Token::BulletPoint(_, content) => {
            if count_newlines_at_end_of_string(content) > 0 {
                false
            } else {
                true
            }
        }

        Token::Pre(_) => false,

        Token::A | Token::StringLiteral(_) | Token::EmptyScene(_) => true,

        _ => false,
    }
}

fn check_if_comptime_value(node: &AstNode) -> bool {
    match node {
        AstNode::Literal(_) | AstNode::ConstReference(_) => true,
        AstNode::Tuple(values) => {
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
