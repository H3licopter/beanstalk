use crate::{ast::AstNode, Token};

// Recursive function to parse scenes
pub fn new_scene(scene_head: &Vec<Token>, tokens: &Vec<Token>, i: &mut usize, inline: &bool) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    /*
        Parse scene head properties to get any styles or other properties
        Produce a string that will be inserted into the HTML tag
        This includes the type of element, styles and other stuff
    */

    let mut properties = String::new();
    let scene_wrapping_tag = if *inline { 
        "span".to_string()
    } else { 
        "div".to_string() 
    };


    // Look at all the possible properties that can be added to the scene head
    if !scene_head.is_empty() {
        let mut j = 0;
        while j < scene_head.len() {
            match scene_head[j] {
                Token::Initialise => { break; }

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

                        properties += &format!(" style=\'color:rgb({},{},{})\' ", red, green, blue);
                        
                        j += 7;
                    }
                }

                // Will escape all characters until the final curly brace
                // Will be formatted as a pre tag but can eventually be formatted with additional styles
                Token::Raw => {}
                
                _ => {
                    scene.push(AstNode::Error(
                        format!("Invalid Token Used inside Scene Head: '{:?}'", &scene_head[j])));
                }
            }

            j += 1;
        }
    }

    while *i < tokens.len() {
        match &tokens[*i] {

            Token::SceneClose => {
                scene.insert(0, AstNode::HTML(format!("<{} {}>", scene_wrapping_tag, properties)));
                scene.push(AstNode::HTML(format!("</{}>", scene_wrapping_tag)));
                return AstNode::Scene(scene);
            }

            Token::SceneHead(new_scenehead, inline) => {
                let nested_scene = new_scene(new_scenehead, tokens, i, &inline);
                scene.push(nested_scene);
            }

            Token::P(content) => {
                scene.push(AstNode::HTML(
                    format!("<p>{}</p>", content)
                ));
            }

            Token::Heading(size, content) => {
                scene.push(AstNode::HTML(
                    format!("<h{}>{}</h{}>", size.to_string(), content, size.to_string())
                ));
            }

            Token::Pre(content) => {
                scene.push(AstNode::HTML(content
                    .replace("{{", r#"\{"#)
                    .replace("}}", r#"\}"#)));
            }

            Token::Empty => {}

            _ => {
                scene.push(AstNode::Error("Invalid Syntax Used Inside scene body".to_string()));
            }
        }

        *i += 1;
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