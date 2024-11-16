use colour::red_ln;

use super::{
    ast_nodes::{AstNode, Reference},
    expressions::parse_expression::{get_args, create_expression},
    styles::{Action, Style, Tag},
    util::{count_newlines_at_end_of_string, count_newlines_at_start_of_string},
};
use crate::{bs_types::DataType, Token};

// Recursive function to parse scenes
pub fn new_scene(
    tokens: &Vec<Token>,
    i: &mut usize,
    ast: &Vec<AstNode>,
    token_line_number: &u32,
    variable_declarations: &Vec<Reference>,
) -> AstNode {
    let mut scene = Vec::new();
    *i += 1;

    let mut scene_tags: Vec<Tag> = Vec::new();
    let mut scene_styles: Vec<Style> = Vec::new();
    let scene_actions: Vec<Action> = Vec::new();
    let mut merge_next_p_line: bool = true;

    // Look at all the possible properties that can be added to the scene head
    while *i < tokens.len() {
        let token = &tokens[*i];
        let inside_brackets = token == &Token::OpenParenthesis;
        *i += 1;

        match token {
            Token::Colon => {
                break;
            }
            Token::SceneClose(spaces) => {
                for _ in 0..*spaces {
                    scene.push(AstNode::Space);
                }
                *i -= 1;
                return AstNode::Scene(scene, scene_tags, scene_styles, scene_actions);
            }

            Token::Id => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "id".to_string(),
                    data_type: DataType::String,
                    default_value: None,
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        continue;
                    }
                };

                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::A(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change strings at runtime dynamically
                    scene_tags.push(Tag::A(eval_arg));
                }
            }

            Token::A => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "href".to_string(),
                    data_type: DataType::String,
                    default_value: None,
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        continue;
                    }
                };
                
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::A(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change href at runtime.
                    scene_tags.push(Tag::A(eval_arg));
                }
            }

            Token::Padding => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "padding".to_string(),
                    data_type: DataType::Float,
                    default_value: Some(Box::new(AstNode::Literal(Token::FloatLiteral(1.5)))),
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        AstNode::Literal(Token::FloatLiteral(1.5))
                    }
                };

                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Padding(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change padding at runtime.
                    scene_styles.push(Style::Padding(eval_arg));
                }
            }

            Token::Margin => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "margin".to_string(),
                    data_type: DataType::Float,
                    default_value: Some(Box::new(AstNode::Literal(Token::FloatLiteral(2.0)))),
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        AstNode::Literal(Token::FloatLiteral(2.0))
                    }
                };

                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Margin(eval_arg));
                } else {
                    scene_styles.push(Style::Margin(eval_arg));
                    // Need to add JS DOM hooks to change margin at runtime.
                }
            }

            // For positioning inside a flex container / grid
            Token::Order => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "order".to_string(),
                    data_type: DataType::Float,
                    default_value: None,
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        continue;
                    }
                };

                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Order(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change order at runtime.
                }
            }

            Token::BG => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "style".to_string(),
                    data_type: DataType::Style,
                    default_value: None,
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        continue;
                    }
                };

                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::BackgroundColor(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change background color at runtime.
                }
            }

            // Colours
            Token::Rgb | Token::Hsl => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "color".to_string(),
                    data_type: DataType::String,
                    default_value: None,
                }];
                let color_type = token.to_owned();
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        continue;
                    }
                };

                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::TextColor(eval_arg, color_type));
                } else {
                    // Need to add JS DOM hooks to change text color at runtime.
                }
            }
            Token::Red
            | Token::Green
            | Token::Blue
            | Token::Yellow
            | Token::Cyan
            | Token::Magenta
            | Token::White
            | Token::Black => {
                let color_type = token.to_owned();
                let required_args: Vec<Reference> = vec![Reference {
                    name: "shade".to_string(),
                    data_type: DataType::Float,
                    default_value: None,
                }];
                match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => {
                        if check_if_comptime_value(&arg) {
                            scene_styles.push(Style::TextColor(arg, color_type));
                        } else {
                            // Need to add JS DOM hooks to change text color at runtime.
                        }
                    },
                    None => {
                        scene_styles.push(Style::TextColor(
                            AstNode::Literal(Token::FloatLiteral(0.0)),
                            color_type,
                        ));
                        continue;
                    }
                };
            }

            Token::Center => {
                scene_styles.push(Style::Center(false));
            }

            Token::Size => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "size".to_string(),
                    data_type: DataType::Float,
                    default_value: None,
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        red_ln!("Error: Size must have an argument");
                        continue;
                    }
                };
                if check_if_comptime_value(&eval_arg) {
                    scene_styles.push(Style::Size(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change text size at runtime.
                }
            }

            Token::Blank => {
                scene_styles.push(Style::Blank);
            }

            Token::Hide => {
                scene_styles.push(Style::Hide);
            }

            Token::Table => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "columns".to_string(),
                    data_type: DataType::Float,
                    default_value: Some(Box::new(AstNode::Literal(Token::FloatLiteral(1.0)))),
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        AstNode::Literal(Token::FloatLiteral(1.0))
                    }
                };

                match eval_arg {
                    AstNode::Literal(literal_token) => match literal_token {
                        Token::FloatLiteral(value) => {
                            scene_tags.push(Tag::Table(value as u32));
                        }
                        Token::IntLiteral(value) => {
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
                let required_args: Vec<Reference> = vec![Reference {
                    name: "src".to_string(),
                    data_type: DataType::String,
                    default_value: None,
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        continue;
                    }
                };

                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Img(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    red_ln!("Can't add img src attribute to scene head at runtime (yet)");
                    scene_tags.push(Tag::Img(eval_arg));
                }
            }

            Token::Video => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "src".to_string(),
                    data_type: DataType::String,
                    default_value: None,
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        continue;
                    }
                };
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Video(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Video(eval_arg));
                }
            }

            Token::Audio => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "src".to_string(),
                    data_type: DataType::String,
                    default_value: None,
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        continue;
                    }
                };
                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Audio(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Audio(eval_arg));
                }
            }

            // Expressions to Parse
            Token::Variable(_)
            | Token::FloatLiteral(_)
            | Token::BoolLiteral(_)
            | Token::IntLiteral(_)
            | Token::StringLiteral(_)
            | Token::RawStringLiteral(_) => {
                *i -= 1;
                scene.push(create_expression(
                    tokens,
                    &mut *i,
                    true,
                    &ast,
                    token_line_number,
                    &DataType::CoerseToString,
                    inside_brackets,
                    variable_declarations,
                ));
            }

            Token::Comma | Token::Newline | Token::Empty => {}

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
                scene_tags.clear();
            }
            Token::CodeBlock(content) => {
                scene.push(AstNode::Element(Token::CodeBlock(content.to_owned())));
            }

            Token::Nav => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "style".to_string(),
                    data_type: DataType::String,
                    default_value: Some(Box::new(AstNode::Literal(Token::FloatLiteral(0.0)))),
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        AstNode::Literal(Token::FloatLiteral(0.0))
                    }
                };

                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Nav(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change img src at runtime.
                    scene_tags.push(Tag::Nav(eval_arg));
                }
            }

            Token::Title => {
                let required_args: Vec<Reference> = vec![Reference {
                    name: "title".to_string(),
                    data_type: DataType::String,
                    default_value: Some(Box::new(AstNode::Literal(Token::FloatLiteral(0.0)))),
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        AstNode::Literal(Token::FloatLiteral(0.0))
                    }
                };

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
                let required_args: Vec<Reference> = vec![Reference {
                    name: "href".to_string(),
                    data_type: DataType::String,
                    default_value: None,
                }];
                let eval_arg = match get_args(tokens, &mut *i, ast, token_line_number, variable_declarations, &required_args) {
                    Some(arg) => arg,
                    None => {
                        red_ln!("Error: Redirect must have an argument");
                        continue;
                    }
                };

                if check_if_comptime_value(&eval_arg) {
                    scene_tags.push(Tag::Redirect(eval_arg));
                } else {
                    // Need to add JS DOM hooks to change src at runtime.
                    scene_tags.push(Tag::Redirect(eval_arg));
                }
            }

            _ => {
                red_ln!("Invalid Token Used inside Scene Head: '{:?}'", token);
            }
        }
    }

    //look through everything that can be added to the scene body
    while *i < tokens.len() {
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

            Token::SceneHead => {
                let nested_scene =
                    new_scene(tokens, i, ast, token_line_number, variable_declarations);
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
                scene.push(AstNode::Error(
                    format!("Dead Variable used in scene. '{}' was never defined", name),
                    token_line_number.to_owned(),
                ));
            }

            _ => {
                scene.push(AstNode::Error(
                    format!(
                        "Invalid Syntax Used Inside scene body when creating scene node: {:?}",
                        tokens[*i]
                    ),
                    token_line_number.to_owned(),
                ));
            }
        }

        *i += 1;
    }

    AstNode::Scene(scene, scene_tags, scene_styles, scene_actions)
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
            // Ignore these tokens, keep searching back
            Token::Colon | Token::SceneClose(_) | Token::SceneHead => {
                j -= 1;
            }

            // Can't go any further back
            Token::ParentScene => {
                return false;
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
        Token::Empty | Token::Newline | Token::Pre(_) => false,

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

        _ => true,
    }
}

fn check_if_comptime_value(node: &AstNode) -> bool {
    match node {
        AstNode::Literal(_) | AstNode::ConstReference(_, _) => true,
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
