use super::tokens::{Token, TokenizeMode}; // Import the Token enum from the tokens module
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars: Peekable<Chars<'_>> = source_code.chars().peekable();
    let mut tokenize_mode: TokenizeMode = TokenizeMode::Normal;
    let mut scene_nesting_level: &mut i64 = &mut 0;

    let mut token: Token = get_next_token(&mut chars, &mut tokenize_mode, &mut scene_nesting_level);
    while token != Token::EOF {
        tokens.push(token);
        token = get_next_token(&mut chars, &mut tokenize_mode, &mut scene_nesting_level);
    }

    tokens
}

fn get_next_token(chars: &mut Peekable<Chars>, tokenize_mode: &mut TokenizeMode, scene_nesting_level: &mut i64) -> Token {
    let mut current_char = chars.next().unwrap_or('\0');
    if current_char == '\0' { return Token::EOF; }

    if current_char == '}' {
        return Token::SceneClose;
    }

    if tokenize_mode == &TokenizeMode::Markdown {
        return tokenize_markdown(chars, scene_nesting_level, tokenize_mode);
    }

    // Newlines must be tokenized for inline statements and scenes
    if current_char == '\n' { return Token::Newline }
    // Skip whitespace
    while current_char.is_whitespace() {
        current_char = chars.next().unwrap_or('\0');
    }

    // Initialisation
    // Check if going into markdown mode
    if current_char == ':' {
        if tokenize_mode == &TokenizeMode::SceneHead {
            *tokenize_mode = TokenizeMode::Markdown;
        }
        return Token::Initialise
    }

    // SCENES
    // Starts a new Scene Tree Node
    if current_char == '{' {
        *scene_nesting_level += 1;
        *tokenize_mode = TokenizeMode::SceneHead;
        return tokenize_scenehead(chars, tokenize_mode, scene_nesting_level);
    }

    let mut token_value: String = String::new();

    //Meta. Compile time things, configuration and metaprogramming
    if current_char == '#' {
        *tokenize_mode = TokenizeMode::Meta;

        if chars.peek() == Some(&'#') {
            chars.next();
            return Token::Comptime;
        }

        //Get compiler directive token
        return keyword_or_variable(&mut token_value, chars, tokenize_mode);
    }

    // Check for string literals
    if current_char == '"' {
        while let Some(ch) = chars.next() {
            // Check for escape characters
            if ch == '\\' {
                if let Some(next_char) = chars.next() {
                    token_value.push(next_char);
                }
            }
            if ch == '"' {
                return Token::StringLiteral(token_value);
            }
            token_value.push(ch);
        }
    }

    //Check for raw strings (backticks are used for raw string)
    if current_char == '`' {
        while let Some(ch) = chars.next() {
            if ch == '`' {
                return Token::RawStringLiteral(token_value);
            }
            token_value.push(ch);
        }
    }

    // Check for character literals
    if current_char == '\'' {
        let char_token = chars.next();
        if let Some(&char_after_next) = chars.peek() {
            if char_after_next == '\'' && char_token.is_some() {
                return Token::RuneLiteral(char_token.unwrap());
            }
        }
    }

    // Functions and grouping expressions
    if current_char == '(' { return Token::OpenBracket }
    if current_char == ')' { return Token::CloseBracket }

    // All the tokens that are always what they are on their own
    // Independant of the next character
    if current_char == '=' { return Token::Assign }
    if current_char == ',' { return Token::Comma }
    if current_char == '.' { return Token::Dot }
    if current_char == ';' { return Token::Semicolon }

    //Error handling
    if current_char == '!' { return Token::Bang }
    if current_char == '?' { return Token::QuestionMark }

    // Comments / Subtraction
    if current_char == '-' {
        if let Some(&next_char) = chars.peek() {
            
            // Comments
            if next_char == '-' {
                chars.next();

                // Check for multiline
                if let Some(&next_next_char) = chars.peek() {
                   
                    if next_next_char == '\n' {
                        // Mutliline Comment
                        chars.next();

                        // Multiline Comment
                        while let Some(ch) = chars.next() {
                            token_value.push(ch);
                            if token_value.ends_with("--") {
                                return Token::MultilineComment(
                                    token_value
                                    .trim_end_matches("\n--")
                                    .to_string());
                            }
                        }
                    } else if next_next_char == '-' {
                        chars.next();
                        chars.next();
                        // Document comment
                        while let Some(ch) = chars.next() {
                            token_value.push(ch);
                            if token_value.ends_with("---") {
                                return Token::DocComment(
                                    token_value
                                    .trim_end_matches("\n---")
                                    .to_string());
                            }
                        }
                    } else {
                        // Inline Comment
                        while let Some(ch) = chars.next() {
                            if ch == '\n' {
                                return Token::Comment(token_value);
                            }
                            token_value.push(ch);
                        }
                    }
                }

            // Subtraction
            } else {
                if next_char == '=' {
                    chars.next();
                    return Token::SubtractionAssign;
                }
                return Token::Subtraction;
            }
        }
    }

    // Mathematical operators, 
    // must peak ahead to check for exponentiation (**) or roots (//) and assign variations
    if current_char == '+' { 
        if let Some(&next_char) = chars.peek() {
            if next_char == '=' {
                chars.next();
                return Token::AdditionAssign;
            }
        } 
        chars.next();
        return Token::Addition 
    }
    if current_char == '*' {
        if let Some(&next_char) = chars.peek() {
            if next_char == '*' {
                chars.next();
                return Token::Exponentiation;
            }
            if next_char == '=' {
                    chars.next();
                    return Token::MultiplicationAssign;
                }
            return Token::Multiplication;
        }
    }
    if current_char == '/' {
        if let Some(&next_char) = chars.peek() {
            if next_char == '/' {
                chars.next();
                if let Some(&next_next_char) = chars.peek() {
                    if next_next_char == '=' {
                        chars.next();
                        return Token::RootAssign;
                    }
                }
                return Token::Root;
            }
            if next_char == '=' {
                chars.next();
                return Token::DivisionAssign;
            }
            return Token::Division;
        }
    }
    if current_char == '%' {
        if let Some(&next_char) = chars.peek() {
            if next_char == '=' {
                chars.next();
                return Token::ModulusAssign;
            }
            if next_char == '%' {
                chars.next();
                if let Some(&next_next_char) = chars.peek() {
                    if next_next_char == '=' {
                        chars.next();
                        return Token::RemainderAssign;
                    }
                }
                return Token::Remainder;
            }
            return Token::Modulus;
        }
    }
    if current_char == '^' { 
        if let Some(&next_char) = chars.peek() {
            if next_char == '=' {
                chars.next();
                return Token::ExponentiationAssign;
            }
        }
        return Token::Exponentiation 
    }

    // Check for greater than and Less than logic operators
    // must also peak ahead to check it's not also equal to
    if current_char == '>' {
        if let Some(&next_char) = chars.peek() {
            if next_char == '=' {
                chars.next();
                return Token::GreaterThanOrEqual;
            }
            return Token::GreaterThan;
        }
    }
    if current_char == '<' {
        if let Some(&next_char) = chars.peek() {
            if next_char == '=' {
                chars.next();
                return Token::LessThanOrEqual;
            }
            return Token::LessThan;
        }
    }

    // Pointers and Memory Allocation
    if current_char == '&' { return Token::Reference }

    // Arrays
    if current_char == '[' { return Token::OpenArray }
    if current_char == ']' { return Token::CloseArray }

    if current_char == '@' { return Token::Href }

    if current_char == '~' { return Token::Bitwise }

    // Numbers
    if current_char.is_numeric() {
        token_value.push(current_char);

        while let Some(&next_char) = chars.peek() {
            if next_char.is_numeric() {
                token_value.push(chars.next().unwrap());

            // Check for dot to determine if it's a float
            } else {
                if next_char == '.' {
                    token_value.push(chars.next().unwrap());
                    while let Some(&next_char) = chars.peek() {
                    if next_char.is_numeric() {
                        token_value.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                    return Token::FloatLiteral(token_value.parse::<f64>().unwrap());
                }
                break;
            }
        }

        // If no dot, parse as an integer
        return Token::IntLiteral(token_value.parse::<i64>().unwrap());
    }

    if current_char.is_alphabetic() {
        token_value.push(current_char);
        return keyword_or_variable(&mut token_value, chars, tokenize_mode);
    }

    Token::Error("Invalid Token Used".to_string())
}

// Nested function because may need multiple searches for variables
fn keyword_or_variable(token_value: &mut String, chars: &mut Peekable<Chars<'_>>, tokenize_mode: &mut TokenizeMode) -> Token  {

    // Match variables or keywords
    while let Some(&next_char) = chars.peek() {

        if next_char.is_alphanumeric() || next_char == '_' {
            token_value.push(chars.next().unwrap());
        } else {
            // If their is whitespace or some termination
            // First check if there is a match to a keyword
            // Otherwise break out and check it is a valid variable name

            // Control Flow
            if token_value == "if" { return Token::If }
            if token_value == "else" { return Token::Else }
            if token_value == "for" { return Token::For }
            if token_value == "return" { return Token::Return }
            if token_value == "match" { return Token::Match }
            if token_value == "break" { return Token::Break }
            if token_value == "when" { return Token::When }
            if token_value == "defer" { return Token::Defer }
            if token_value == "in" { return Token::In }
            if token_value == "as" { return Token::As }

            // Logical
            if token_value == "is" { return Token::Equal }
            if token_value == "not" { return Token::Not }
            if token_value == "and" { return Token::And }
            if token_value == "or" { return Token::Or }

            // Data Types
            match token_value.as_str() {
                "int" => return Token::TypeInt,
                "float" => return Token::TypeFloat,
                "string" => return Token::TypeString,
                "rune" => return Token::TypeRune,
                "bool" => return Token::TypeBool,
                "decimal" => return Token::TypeDecimal, 
                "scene" => return Token::TypeScene,
                "collection" => return Token::TypeCollection,
                "object" => return Token::TypeObject,
                _ => {}    
            
            }

            // only bother tokenizing / reserving these keywords if inside of a scene head
            if tokenize_mode == &TokenizeMode::SceneHead {
                if token_value == "rgb" { return Token::Rgb }
                if token_value == "img" { return Token::Img }
                if token_value == "code" { return Token::Code }
                if token_value == "video" { return Token::Video }
                if token_value == "slot" { return Token::Slot }
            }
            
            // Compiler directives
            if tokenize_mode == &TokenizeMode::Meta {
                *tokenize_mode = TokenizeMode::Normal;

                if token_value == "import" { return Token::Import }
                if token_value == "export" { return Token::Export }
                if token_value == "exclude" { return Token::Exclude }
                if token_value == "main" { return Token::Main }

                // HTML project settings
                if token_value == "page" { return Token::Page }
                if token_value == "component" { return Token::Component }
                if token_value == "date" { return Token::Date }
                if token_value == "title" { return Token::Title }
            }

            break;
        }
    }

    if is_valid_identifier(&token_value) {
        return Token::Variable(token_value.to_string());
    }
    
    Token::Error(format!("Invalid variable name: {}", token_value))
}

// Checking if the variable name it valid
fn is_valid_identifier(s: &str) -> bool {
    // Check if the string is a valid identifier (variable name)
    s.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_') &&
        s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn tokenize_scenehead(chars: &mut Peekable<Chars>, tokenize_mode: &mut TokenizeMode, scene_nesting_level: &mut i64) -> Token {
    let mut scene_head: Vec<Token> = Vec::new();

    while tokenize_mode == &TokenizeMode::SceneHead {
        scene_head.push(get_next_token(chars, tokenize_mode, scene_nesting_level));
    }

    Token::SceneHead(scene_head)
}

// Create string of markdown content, only escaping when a closed curly brace is found
// Any Beanstalk specific extensions to Markdown will need to be implimented here
fn tokenize_markdown(chars: &mut Peekable<Chars>, scene_nesting_level: &mut i64, tokenize_mode: &mut TokenizeMode) -> Token {
    let mut markdown_content = String::new();

    while let Some(next_char) = chars.peek() {
        if next_char == &'\0' { 
            *tokenize_mode = TokenizeMode::Normal;
            break;
        }

        if next_char == &'}' {
            *scene_nesting_level -= 1;
            if *scene_nesting_level == 0 {
                *tokenize_mode = TokenizeMode::Normal;
            } else {
                *tokenize_mode = TokenizeMode::Markdown;
            }
            break;
        }

        if next_char == &'{' {
            *tokenize_mode = TokenizeMode::SceneHead;
            break;
        }

        markdown_content.push(chars.next().unwrap());
    }

    Token::Markdown(markdown_content.to_owned())
}
