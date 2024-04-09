use super::{
    ast::AstNode, build_ast::new_collection, parse_expression::create_expression, styles::{Style, Tag}, util::{count_newlines_at_end_of_string, count_newlines_at_start_of_string}
};
use crate::Token;

// Recursive function to parse scenes
pub fn new_scene(scene_head: &Vec<Token>, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    let mut scene_tags: Vec<Tag> = Vec::new();
    let mut scene_styles: Vec<Style> = Vec::new();
    let mut scene_open: bool = true;

    // Look at all the possible properties that can be added to the scene head
    let mut j = 0;

    // CURRENTLY ONLY SUPPORTS COMPILE TIME PROPERTIES
    // NEEDS TO BE ABLE TO LINK INTO GENERATED JS
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
                let href = match &scene_head[j] {
                    Token::StringLiteral(value) => {
                        format!("\"https://{}\"", value)
                    }
                    _ => {
                        scene.push(AstNode::Error(
                            "No string literal provided for a href".to_string(),
                        ));
                        "".to_string()
                    }
                };
                scene_tags.push(Tag::A(href));
            }

            Token::Rgb => {
                let values = parse_scenehead_number_values(scene_head, &mut j);
                
                if values.len() == 3 {
                    scene_styles.push(Style::TextColor(
                        values[0] as u8,
                        values[1] as u8,
                        values[2] as u8,
                    ));
                } else {
                    scene.push(AstNode::Error(
                        "Invalid number of values provided for rgb".to_string(),
                    ));
                }
            }

            Token::Size => {
                let values = parse_scenehead_number_values(scene_head, &mut j);
                
                if values.len() == 2 {
                    scene_styles.push(Style::Size(values[0], values[1]));
                } else if values.len() == 1 {
                    scene_styles.push(Style::Size(values[0], values[0]));
                } else {
                    scene.push(AstNode::Error(
                        "Invalid number of values provided for size parameter".to_string(),
                    ));
                }
            }

            Token::Img => {
                j += 1;
                match &scene_head[j] {
                    Token::StringLiteral(value) => {
                        scene_tags.push(Tag::Img(value.clone()));
                    }
                    _ => {
                        scene.push(AstNode::Error("No src provided for img".to_string()));
                    }
                };
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
                match &scene_head[j] {
                    Token::StringLiteral(value) => {
                        scene_tags.push(Tag::Video(value.clone()));
                    }
                    _ => {
                        scene.push(AstNode::Error("No src provided for video".to_string()));
                    }
                };
            }

            Token::Audio => {
                j += 1;
                match &scene_head[j] {
                    Token::StringLiteral(value) => {
                        scene_tags.push(Tag::Audio(value.clone()));
                    }
                    _ => {
                        scene.push(AstNode::Error("No src provided for audio".to_string()));
                    }
                };
            }

            Token::StringLiteral(string_content) | Token::RawStringLiteral(string_content) => {
                scene.push(AstNode::Element(Token::Span(string_content.to_string())));
            }

            Token::ParentScene => {
                // Useful for wrapping the scene in <main> or unwrapped if it's a component
            }

            Token::Comma | Token::Newline => {}

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
                let nested_scene = new_scene(&new_scenehead, tokens, i);
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
                scene.push(AstNode::Element(Token::BulletPoint(
                    *indentation,
                    content.clone(),
                )));
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

            Token::Empty | Token::Initialise => {}

            _ => {
                scene.push(AstNode::Error(
                    "Invalid Syntax Used Inside scene body".to_string(),
                ));
            }
        }

        *i += 1;
    }

    if !scene_tags.is_empty() || !scene_styles.is_empty() {
        scene.insert(0, AstNode::SceneTag(scene_tags, scene_styles));
    }

    AstNode::Scene(scene)
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
            Token::Initialise | Token::SceneClose(_) => {
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

        Token::A | Token::StringLiteral(_) => true,

        _ => {
            println!("Previous Element: {:?}", previous_element);
            false
        }
    }
}

fn parse_scenehead_number_values(scene_head: &Vec<Token>, i: &mut usize) -> Vec<f64> {
    let mut values = Vec::new();

    *i += 1;
    
    // If Collection, unwrap the values
    if &scene_head[*i] == &Token::OpenCollection {
        let collection = new_collection(&scene_head, i);
        println!("Collection: {:?}", collection);
        match collection {
            AstNode::Collection(elements) => {
                for element in elements {
                    match element {
                        AstNode::Literal(Token::FloatLiteral(value)) => {
                            values.push(value.clone());
                        }
                        AstNode::Literal(Token::IntLiteral(value)) => {
                            values.push(value.clone() as f64);
                        }
                        _ => {
                            AstNode::Error("Invalid value in collection".to_string());
                        }
                    }
                }
            }
            _ => {}
        }

        return values;
    }

    let mut args = Vec::new();
    while *i < scene_head.len() - 1 {
        args.push(create_expression(
            scene_head,
            i,
        ));

        *i += 1;
    }

    for node in args {
        match node {
            AstNode::Literal(Token::FloatLiteral(value)) => {
                values.push(value.clone());
            }
            AstNode::Literal(Token::IntLiteral(value)) => {
                values.push(value as f64);
            }
            _ => {
                values.push(0.0);
            }
        }
    }

    values
}
