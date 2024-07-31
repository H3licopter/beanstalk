use super::tokens::{Token, TokenizeMode};
use crate::tokenizer::get_next_token;
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize_scenehead(
    chars: &mut Peekable<Chars>,
    tokenize_mode: &mut TokenizeMode,
    scene_nesting_level: &mut i64,
) -> Token {
    let mut scene_head: Vec<Token> = Vec::new();
    let mut code_block: bool = false;

    if scene_nesting_level == &1 {
        scene_head.push(Token::ParentScene);
    }

    while tokenize_mode == &TokenizeMode::SceneHead {
        let next_token = get_next_token(chars, tokenize_mode, scene_nesting_level);
        match next_token {
            Token::EOF | Token::AssignComptime => {
                break;
            }
            Token::SceneClose(_) => {
                scene_head.push(next_token);
                break;
            }
            Token::CodeKeyword => {
                code_block = true;
            }
            _ => {
                scene_head.push(next_token);
            }
        }
    }

    if code_block {
        *tokenize_mode = TokenizeMode::Codeblock
    }

    Token::SceneHead(scene_head)
}

// Create string of markdown content, only escaping when a closed curly brace is found
// Any Beanstalk specific extensions to Markdown will need to be implimented here
pub fn tokenize_markdown(chars: &mut Peekable<Chars>, current_char: &mut char) -> Token {
    let mut content = String::new(); // To keep track of current chars being parsed
    let mut previous_newlines = 0;
    let mut current_token = Token::Empty;

    //Ignore starting whitespace (except newlines)
    while current_char.is_whitespace() {
        if current_char == &'\n' {
            if content.ends_with("\n") {
                return Token::Newline;
            }
            previous_newlines += 1;
            content.push('\n');
        }

        match chars.peek() {
            Some(ch) => match ch {
                '[' | ']' | '-' | '#' | '*' => {
                    break;
                }
                _ => {
                    *current_char = *ch;
                    chars.next();
                }
            },
            None => return Token::EOF,
        };
    }

    // HEADINGS
    if current_char == &'#' {
        let mut strength = 1;
        previous_newlines = 0;

        loop {
            match chars.peek() {
                Some(ch) => match ch {
                    '#' => {
                        strength += 1;
                        *current_char = *ch;
                        chars.next();
                        continue;
                    }
                    // Break in the hashes
                    ' ' => {
                        *current_char = *ch;
                        chars.next();
                        return Token::HeadingStart(strength);
                    }
                    // Cancel the heading, just normal hashes
                    _ => {
                        *current_char = *ch;
                        chars.next();
                        for _ in 0..strength {
                            content.push('#');
                            content.push(current_char.clone());
                        }
                        break;
                    }
                },
                None => return Token::EOF,
            };
        }
    // BULLET POINTS
    } else if current_char == &'-' {
        let mut strength: u8 = 1;
        previous_newlines = 0;

        loop {
            match chars.peek() {
                Some(ch) => match ch {
                    '-' => {
                        strength += 1;
                        *current_char = *ch;
                        chars.next();
                        continue;
                    }
                    // Break in the dashes
                    ' ' => {
                        *current_char = *ch;
                        chars.next();
                        return Token::BulletPointStart(strength);
                    }
                    // Cancel the heading, just normal hashes
                    _ => {
                        *current_char = *ch;
                        chars.next();
                        for _ in 0..strength {
                            content.push('-');
                            content.push(current_char.clone());
                        }
                        break;
                    }
                },
                None => return Token::EOF,
            };
        }
    // EM TAGS
    } else if current_char == &'*' {
        let mut strength: u8 = 1;
        previous_newlines = 0;

        loop {
            match chars.peek() {
                Some(ch) => match ch {
                    '*' => {
                        strength += 1;
                        *current_char = *ch;
                        chars.next();
                        continue;
                    }
                    // If there is a space or newline after the asterisk, cancel the em tag
                    ' ' | '\n' => {
                        *current_char = *ch;
                        chars.next();
                        for _ in 0..strength {
                            content.push('*');
                            content.push(current_char.clone());
                        }
                        break;
                    }
                    _ => {
                        *current_char = *ch;
                        chars.next();
                        current_token = Token::Em(strength, String::new());
                        break;
                    }
                },
                None => return Token::EOF,
            };
        }
    }

    // Loop through the elements content until hitting a condition that
    // breaks out of the element
    let mut parse_raw = false;
    loop {
        // Parsing Raw String inside of Markdown
        if parse_raw {
            *current_char = chars.next().unwrap();
            match current_char {
                // Escape character for backticks in raw strings
                '\\' => {
                    match chars.next() {
                        Some('`') => {
                            content.push('`');
                        }
                        _ => {
                            content.push('\\');
                        }
                    };
                }
                '`' => {
                    break;
                }
                _ => {
                    content.push(current_char.clone());
                }
            }
            continue;
        }

        // Raw Strings
        if current_char == &'`' {
            parse_raw = true;
            previous_newlines = 0;
            continue;
        }

        if current_char == &'\n' {
            content.push('\n');
            break;
        } else if !current_char.is_whitespace() {
            previous_newlines = 0;
        }

        if current_char == &' ' {
            if chars.peek() == Some(&' ') {
                content.push_str("&nbsp;");
                chars.next();
                continue;
            }
        }

        content.push(current_char.clone());

        match chars.peek() {
            Some(&ch) => match ch {
                ']' => {
                    content = content.trim_end().to_string();
                    break;
                }
                '[' => {
                    break;
                }
                '-' | '#' => {
                    if previous_newlines > 0 {
                        break;
                    } else {
                        *current_char = ch;
                        chars.next();
                    }
                }
                '*' => {
                    match current_token {
                        // Breaking out of current em tag
                        Token::Em(strength, _) => {
                            // Count strength of em tag and make sure it's the same
                            // Once it hits the same number of asterisks, return the em tag
                            let mut asterisks = 1;
                            loop {
                                if strength == asterisks {
                                    chars.next();
                                    // Check for any spaces after the asterisks and add them to the end of the content
                                    while let Some(&next_char) = chars.peek() {
                                        if next_char == ' ' {
                                            content.push_str("&nbsp;");
                                            chars.next();
                                        } else {
                                            break;
                                        }
                                    }
                                    return Token::Em(strength, content);
                                }

                                chars.next();
                                if let Some(&next_char) = chars.peek() {
                                    if next_char == '*' {
                                        asterisks += 1;
                                        continue;
                                    }
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                        // New em tag?
                        _ => {
                            if content.ends_with(' ') || previous_newlines > 0 {
                                // This could be an Em tag
                                break;
                            }
                            *current_char = ch;
                            chars.next();
                        }
                    }
                }
                _ => {
                    *current_char = ch;
                    chars.next();
                }
            },
            None => {
                break;
            }
        }
    }

    // Return relevant token
    if !content.trim().is_empty() {
        match current_token {
            Token::Empty => return Token::P(content),
            Token::Em(size, _) => return Token::Em(size, content),
            Token::Superscript(_) => return Token::Superscript(content),
            _ => return current_token,
        }
    } else {
        return Token::Empty;
    }
}

// Ignores everything except for the closing brackets
// If there is a greater number of closing brackets than opening brackets,
// Close the codeblock and return the token
pub fn tokenize_codeblock(chars: &mut Peekable<Chars>) -> Token {
    let mut codeblock = String::new();
    let mut brackets = 1;
    let mut raw_mode = false;

    while let Some(ch) = chars.peek() {
        match ch {
            &'[' => {
                if !raw_mode {
                    brackets += 1;
                }
            }
            &']' => {
                if !raw_mode {
                    brackets -= 1;
                }
            }
            &'`' => {
                raw_mode = !raw_mode;
            }
            _ => {}
        }
        if brackets == 0 {
            break;
        }
        codeblock.push(*ch);
        chars.next();
    }

    Token::CodeBlock(codeblock)
}
