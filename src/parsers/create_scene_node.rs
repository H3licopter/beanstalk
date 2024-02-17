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
    let mut properties = "span".to_string();
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
                    }
                }
                _ => {
                    scene.push(AstNode::Error("Invalid Token Used".to_string()));
                }
            }
            j += 1;
        }
    }

    scene.push(AstNode::ElementProperties(properties));

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
                // Skip token if empty markdown
                if !md_content.is_empty() {
                    let html = markdown::to_html(md_content);
                    scene.push(AstNode::HTML(html));

                    // GFM Style markdown parsing?
                    // let parsed_markdown = markdown::to_html_with_options(md_content, &markdown::Options::gfm());
                    // match parsed_markdown {
                    //     Ok(html) => {
                    //         scene.push(AstNode::HTML(html));
                    //     }
                    //     Err(_) => {
                    //         scene.push(AstNode::Error("Error parsing markdown".to_string()));
                    //     }
                    // }
                }
            }

            // Scene head keywords and expressions

            _ => {
                scene.push(AstNode::Error("Invalid Token Used".to_string()));
            }
        }

        *i += 1;
    }

    AstNode::Scene(scene)
}