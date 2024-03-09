use crate::{ast::AstNode, Token};

// Recursive function to parse scenes
pub fn new_scene(scene_head: &Vec<Token>, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    /*
        Parse scene head properties to get any styles or other properties
        Produce a string that will be inserted into the HTML tag
        This includes the type of element, styles and other stuff
    */

    let mut properties = String::new();

    // Look at all the possible properties that can be added to the scene head
    let mut j = 0;

    while j < scene_head.len() {
        match scene_head[j] {
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

                    properties += &format!("style=\'color:rgb({},{},{})\' ", red, green, blue);
                    
                    j += 7;
                }
            }

            Token::Img => {
                j += 1;
                let src = match &scene_head[j] {
                    Token::StringLiteral(value) => {
                        value.to_string()
                    }
                    _ => {
                        scene.push(AstNode::Error("No string literal provided for img src".to_string()));
                        "".to_string()
                    }
                };

                properties += &format!("src={}", src);
            }

            // Will escape all characters until the final curly brace
            // Will be formatted as a pre tag but can eventually be formatted with additional styles
            Token::Raw => {}
            
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
                        AstNode::Block(Token::P(content.to_string()))
                    } else {
                        AstNode::Inline(Token::P(content.to_string()))
                    }
                );
            }

            Token::Heading(size, content) => {
                scene.push(
                    if !check_if_inline(tokens, *i) {
                        AstNode::Block(Token::Heading(*size, content.to_string()))
                    } else {
                        AstNode::Inline(Token::Heading(*size, content.to_string()))
                    }
                );
            }

            Token::Pre(content) => {
                scene.push(AstNode::Block(Token::Pre(
                    content
                        .replace("{{", r#"\{"#)
                        .replace("}}", r#"\}"#)
                    )));
            }

            Token::Empty | Token::Initialise => {}

            _ => {
                scene.push(AstNode::Error("Invalid Syntax Used Inside scene body".to_string()));
            }
        }

        *i += 1;
    }

    AstNode::Scene(scene)
}

fn check_if_inline(tokens: &Vec<Token>, i: usize) -> bool {

    let current_element = &tokens[i];

    // Iterate back through tokens to find the last token that isn't Initialise, Scenehead or Sceneclose
    let mut previous_element = &Token::Empty;
    let mut j = i - 1;
    while j > 0 {
        match &tokens[j] {
            Token::Initialise | Token::SceneHead(_) | Token::SceneClose => {
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
            match current_element {
                Token::P(_) => {
                    if content.ends_with("\n\n") { false } else { true }
                }
                _ => {
                    false
                }
            }
        }
        Token::Heading(_, content) => {
            match current_element {
                Token::Heading(_, _) => {
                    if content.ends_with("\n\n") { false } else { true }
                }
                _ => {
                    false
                }
            }
        }
        _ => {
            false
        }
    }
}