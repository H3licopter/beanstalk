use super::util::{count_newlines_at_end_of_string, count_newlines_at_start_of_string};
use crate::{ast::AstNode, Token};

struct Element {
    tag: String,
    properties: String,
}

// Recursive function to parse scenes
pub fn new_scene(scene_head: &Vec<Token>, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    let mut scene_wrapping_tags: Vec<Element> = Vec::new();
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
                scene_wrapping_tags.push(Element {
                    tag: "a".to_string(),
                    properties: format!("href={}", href),
                });
            }

            Token::Rgb => {
                if j + 7 >= scene_head.len() {
                    scene.push(AstNode::Error(
                        "RGB values not formatted correctly. \nShould look like: rgb(0, 0, 0)"
                            .to_string(),
                    ));
                } else {
                    let mut red = 0;
                    match &scene_head[j + 2] {
                        Token::IntLiteral(value) => {
                            red = *value as i32;
                        }
                        _ => {
                            scene.push(AstNode::Error("Invalid RGB value for red".to_string()));
                        }
                    }
                    let mut green = 0;
                    match &scene_head[j + 4] {
                        Token::IntLiteral(value) => {
                            green = *value as i32;
                        }
                        _ => {
                            scene.push(AstNode::Error("Invalid RGB value for green".to_string()));
                        }
                    }
                    let mut blue = 0;
                    match &scene_head[j + 6] {
                        Token::IntLiteral(value) => {
                            blue = *value as i32;
                        }
                        _ => {
                            scene.push(AstNode::Error("Invalid RGB value for blue".to_string()));
                        }
                    }

                    scene_wrapping_tags.push(Element {
                        tag: "span".to_string(),
                        properties: format!("style=\'color:rgb({},{},{})\' ", red, green, blue),
                    });

                    j += 7;
                }
            }

            Token::Img => {
                j += 1;
                let src = match &scene_head[j] {
                    Token::StringLiteral(value) => {
                        format!("\"{}\"", value)
                    }
                    _ => {
                        scene.push(AstNode::Error(
                            "No string literal provided for img src".to_string(),
                        ));
                        "".to_string()
                    }
                };

                scene_wrapping_tags.push(Element {
                    tag: "img".to_string(),
                    properties: format!("src={}", src),
                });
            }

            Token::StringLiteral(string_content) | Token::RawStringLiteral(string_content) => {
                scene.push(AstNode::Element(Token::Span(string_content.to_string())));
            }

            Token::ParentScene => {
                // Useful for wrapping the scene in <main> or unwrapped if it's a component
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
                scene.push(AstNode::Element(Token::BulletPoint(*indentation, content.clone())));
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

    if !scene_wrapping_tags.is_empty() {
        for element in scene_wrapping_tags.iter().rev() {
            scene.insert(
                0,
                AstNode::SceneTag(format!("<{} {}>", element.tag, element.properties)),
            );
            scene.push(AstNode::SceneTag(format!("</{}>", element.tag)));
        }
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
        
        Token::Pre(_) => { false }

        Token::A | Token::StringLiteral(_) => true,

        _ => {
            println!("Previous Element: {:?}", previous_element);
            false
        }
    }
}
