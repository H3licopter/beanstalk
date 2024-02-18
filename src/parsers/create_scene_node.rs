use crate::{ast::AstNode, Token};

// Recursive function to parse scenes
pub fn new_scene(scene_head: &Vec<Token>, tokens: &Vec<Token>, i: &mut usize) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    /*
    Parse scene head properties to get ElementProperties
    Produce a string that will be inserted into the HTML tag
    This includes the type of element, styles and other stuff
    If scene head is empty, use a div if the scenehead is after two or more newlines,
    Otherwise, use a span. ElementProperties should be at start of scene.
    */
    let mut properties = String::new();
    let mut scene_tag = "span".to_string();

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

                        properties += &format!(" style=\"color:rgb({},{},{})\"", red, green, blue);
                        
                        j += 7;
                    }
                }
                Token::Code => {
                    scene_tag = "code".to_string();
                }
                _ => {
                    scene.push(AstNode::Error(format!("Invalid Token Used inside Scene Head: '{:?}'", &scene_head[j])));
                }
            }
            j += 1;
        }
    }

    scene.push(AstNode::ElementProperties(properties, scene_tag));

    while *i < tokens.len() {
        match &tokens[*i] {
            
            Token::SceneClose => {
                return AstNode::Scene(scene)
            }

            Token::SceneHead(new_scenehead) => {
                let nested_scene = new_scene(new_scenehead, tokens, i);
                scene.push(nested_scene);
            }

            Token::Markdown(md_content) => {
                // Parse markdown to HTML
                let mut tag: String = String::new();

                // // Add breaks for each newline at the start of the markdown segment
                // let mut newlines = 0;
                // for c in md_content.chars() {
                //     if c == '\n' {
                //         newlines += 1;
                //     } else { break; }
                // }

                let mut content = md_content.trim_start().to_string();

                if content.is_empty() {
                    scene.push(AstNode::Gap);
                    *i += 1;
                    continue;
                }

                // Add heading tags if markdown starts with #
                if content.starts_with("#") {
                    let mut hashes = 0;
                    for c in content.chars() {
                        if c == '#' {
                            hashes += 1;
                        } else {
                            content = content.trim_start_matches("#").to_owned();
                            break;
                        }
                    }
                    
                    tag = format!("h{}", hashes);
                }

                let mut formatted_tag = String::new();
                if !tag.is_empty() {
                    formatted_tag = format!("<{}>", tag);
                }


                scene.push(AstNode::HTML(tag, format!("{}{}", formatted_tag, content)));
            }

            _ => {
                scene.push(AstNode::Error("Invalid Syntax Used Inside scene body".to_string()));
            }
        }

        *i += 1;
    }

    AstNode::Scene(scene)
}

fn get_closing_tag(element: &str) -> String {
    let index = element.char_indices().rev().find(|c| c.1 == '/');

    match index {
        Some((i, _)) => {
            let tag = element.split_at(i + 1)
                .1
                .replace(">", "");
            tag
        }
        None => { String::new() }
    }
}