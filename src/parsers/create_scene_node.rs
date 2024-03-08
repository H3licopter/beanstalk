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

    let previous_element = if *i > 1 { &tokens[*i - 2] } else { &Token::Empty };
    
    let is_inline = match previous_element {
        Token::Empty => { false }
        Token::P(content) => {
            if content.ends_with("\n\n") { false } else {
                scene.push(AstNode::Inline("</p>".to_string()));
                true
            }
        }
        Token::Heading(size, content) => {
            if content.ends_with("\n\n") { false } else { 
                let tag = format!("</h{}>", size).to_string();
                scene.push(AstNode::Inline(tag.to_string()));
                true 
            }
        }
        _ => {
            false
        }
    };

    let mut scene_wrapping_tag = if is_inline { 
        "span".to_string()
    } else { 
        "div".to_string() 
    };


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

                scene_wrapping_tag = "img".to_string();
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
                scene.insert(0, AstNode::HTML(format!("<{} {}>", scene_wrapping_tag, properties)));
                scene.push(AstNode::HTML(format!("</{}>", scene_wrapping_tag)));
                break;
            }

            Token::SceneHead(new_scenehead) => {
                let nested_scene = new_scene(&new_scenehead, tokens, i);
                scene.push(nested_scene);
            }

            Token::P(content) => {
                scene.push(AstNode::HTML(
                    if !is_inline {
                        // If the next token is a new inline scene, don't close the tag
                        match &tokens[*i + 1] {
                            Token::SceneHead(_) => {
                                if content.ends_with("\n\n") {
                                    format!("<p>{}</p>", content)
                                } else {
                                    format!("<p>{}", content)
                                }
                            }
                            _ => format!("<p>{}</p>", content)
                        }
                    } else {
                        content.to_string()
                    }
                ));
            }

            Token::Heading(size, content) => {
                scene.push(AstNode::HTML(
                    if !is_inline {
                        // If the next token is a new inline scene, don't close the tag
                        match &tokens[*i + 1] {
                            Token::SceneHead(_) => {
                                if content.ends_with("\n\n") {
                                    format!("<h{}>{}</h{}>", size.to_string(), content, size.to_string())
                                } else {
                                    format!("<h{}>{}", size.to_string(), content)
                                }
                            }
                            _ => format!("<h{}>{}</h{}>", size.to_string(), content, size.to_string())
                        }
                    } else {
                        content.to_string() 
                    }
                ));
            }

            Token::Pre(content) => {
                scene.push(AstNode::HTML(content
                    .replace("{{", r#"\{"#)
                    .replace("}}", r#"\}"#)));
            }

            Token::Empty | Token::Initialise => {}

            _ => {
                scene.push(AstNode::Error("Invalid Syntax Used Inside scene body".to_string()));
            }
        }

        *i += 1;
    }

    // ADDING CLOSING TAG ONLY IF NEEDED
    // If the scene is inline, check if it is inbetween two of the same elements or if it is the last element
    match scene.first() {
        Some(AstNode::Inline(closing_tag)) => {
            let next_element = if *i < tokens.len() { &tokens[*i] } else { &Token::Empty };
            match next_element {
                Token::P(_) => {
                    if closing_tag.contains("p") {
                       return AstNode::Scene(scene)
                    }
                }
                Token::Heading(size, _) => {
                    if closing_tag.contains(&format!("h{}", size)) {
                        return AstNode::Scene(scene)
                    }
                }
                _ => {
                    scene.push(AstNode::HTML(closing_tag.to_string()))
                }
            }
        }
        _ => {}
    }

    AstNode::Scene(scene)
}


// // Add breaks for each newline at the start of the markdown segment
// let mut newlines = 0;
// for c in md_content.chars() {
//     if c == '\n' {
//         newlines += 1;
//     } else { break; }
// }