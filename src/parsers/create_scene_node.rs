use crate::{ast::AstNode, Token};

use super::util::count_newlines_at_end_of_string;

struct Element {
    tag: String,
    properties: String,
}

// Recursive function to parse scenes
pub fn new_scene(scene_head: &Vec<Token>, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    /*
        Parse scene head properties to get any styles or other properties
        Produce a string that will be inserted into the HTML tag
        This includes the type of element, styles and other stuff
    */

    let mut scene_wrapping_tags: Vec<Element> = Vec::new();

    // Look at all the possible properties that can be added to the scene head
    let mut j = 0;

    while j < scene_head.len() {
        match scene_head[j] {

            Token::A => {
                j += 1;
                let href = match &scene_head[j] {
                    Token::StringLiteral(value) => {
                        format!("\"https://{}\"", value)
                    }
                    _ => {
                        scene.push(AstNode::Error("No string literal provided for a href".to_string()));
                        "".to_string()
                    }
                };
                scene_wrapping_tags.push(Element {
                    tag: "a".to_string(),
                    properties: format!("href={}", href)
                });
            }
            
            Token::Rgb => {
                if j + 7 >= scene_head.len() {
                    scene.push(AstNode::Error("RGB values not formatted correctly. \nShould look like: rgb(0, 0, 0)".to_string()));
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
                        properties: format!("style=\'color:rgb({},{},{})\' ", red, green, blue)
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
                        scene.push(AstNode::Error("No string literal provided for img src".to_string()));
                        "".to_string()
                    }
                };

                if !check_if_inline(tokens, *i) {
                    scene_wrapping_tags.push(Element {
                        tag: "div".to_string(),
                        properties: format!("src={}", src)
                    });
                }

                scene_wrapping_tags.push(Element {
                    tag: "img".to_string(),
                    properties: format!("src={}", src)
                });
            }

            // Will escape all characters until the final curly brace
            // Will be formatted as a pre tag but can eventually be formatted with additional styles
            Token::Raw => {                
                j += 1;

                let mut arg = "".to_string();
                if scene_head.len() > j {
                    arg = match &scene_head[j] {
                        Token::StringLiteral(value) => {
                            format!("\"{}\"", value)
                        }
                        _ => {
                            "".to_string()
                        }
                    };
                }

                scene_wrapping_tags.push(Element {
                    tag: "div".to_string(),
                    properties: format!("{}{}", "class=bs-raw", if !arg.is_empty() {format!("-{}", arg)} else {"".to_string()})
                });
            }
            
            _ => {
                scene.push(AstNode::Error(
                    format!("Invalid Token Used inside Scene Head: '{:?}'", &scene_head[j])
                ));
            }
        }

        j += 1;
    }

    while *i < tokens.len() {
        match &tokens[*i] {

            Token::SceneClose | Token::EOF => {
                break;
            }

            Token::SceneHead(new_scenehead) => {
                let nested_scene = new_scene(&new_scenehead, tokens, i);
                scene.push(nested_scene);
            }

            Token::P(content) => {
                scene.push(
                    if !check_if_inline(tokens, *i) {
                        AstNode::Element(Token::P(content.to_string()))
                    } else {
                        AstNode::Element(Token::Span(content.to_string()))
                    }
                );
            }

            Token::Heading(size, content) => {
                scene.push(
                    if !check_if_inline(tokens, *i) {
                        AstNode::Element(Token::Heading(*size, content.to_string()))
                    } else {
                        AstNode::Element(Token::Span(content.to_string()))
                    }
                );
            }

            Token::Em(strength, content) => {
                scene.push(AstNode::Element(Token::Em(*strength, content.to_string())));
            }

            Token::Pre(content) => {
                scene.push(AstNode::Element(Token::Pre(content.to_string())));
            }

            Token::Empty | Token::Initialise => {}

            _ => {
                scene.push(AstNode::Error("Invalid Syntax Used Inside scene body".to_string()));
            }
        }

        *i += 1;
    }

    if !scene_wrapping_tags.is_empty() {
        for element in scene_wrapping_tags.iter().rev() {
            scene.insert(0, AstNode::SceneTag(format!("<{} {}>", element.tag, element.properties)));
            scene.push(AstNode::SceneTag(format!("</{}>", element.tag)));
        }
    }

    AstNode::Scene(scene)
}

fn check_if_inline(tokens: &Vec<Token>, i: usize) -> bool {
    // Iterate back through tokens to find the last token that isn't Initialise, Scenehead or Sceneclose
    let mut previous_element = &Token::Empty;
    let mut j = i - 1;
    while j > 0 {
        match &tokens[j] {

            // Ignore these tokens
            Token::Initialise | Token::SceneClose => {
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
        Token::Empty => { false }
        Token::P(content) => {
            if count_newlines_at_end_of_string(content) > 1 { false } else { true }
        }
        Token::Heading(_, content) => {
            if count_newlines_at_end_of_string(content) > 0 { false } else { true }
        }
        Token::Em(_, content) => {
            if count_newlines_at_end_of_string(content) > 0 { false } else { true }
        }
        Token::A => { true }
        _ => {
            false
        }
    }
}