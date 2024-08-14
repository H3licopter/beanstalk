use colour::red_ln;

use super::tokens::{Declaration, Token, TokenizeMode};
use crate::bs_types::DataType;
use crate::tokenize_scene::{tokenize_codeblock, tokenize_markdown, tokenize_scenehead};
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize(source_code: &str, module_name: &str, globals: Vec<Declaration>) -> (Vec<Token>, Vec<u32>) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line_number: u32 = 1;
    let mut token_line_numbers: Vec<u32> = Vec::new();
    let mut chars: Peekable<Chars<'_>> = source_code.chars().peekable();
    let mut tokenize_mode: TokenizeMode = TokenizeMode::Normal;
    let mut scene_nesting_level: &mut i64 = &mut 0;

    // For variable optimisation
    let mut var_names: Vec<Declaration> = globals;
    let mut token: Token = Token::ModuleStart(module_name.to_string());

    loop {
        match token {
            Token::Variable(name) => {
                token = new_var_or_ref(name, &mut var_names, &tokens);
            }

            // Check for variables used inside of scenehead
            // Replace with reference if it's been declared, otherwise remove it as dead code
            Token::SceneHead(content) => {
                let mut processed_scenehead: Vec<Token> = Vec::new();
                for t in content {
                    match t {
                        Token::Variable(name) => {
                            let var = new_var_or_ref(name, &mut var_names, &tokens);
                            match var {
                                Token::VarReference(id) => {
                                    processed_scenehead.push(Token::VarReference(id));
                                }
                                Token::ConstReference(id) => {
                                    processed_scenehead.push(Token::ConstReference(id));
                                }
                                _ => {
                                    processed_scenehead.push(Token::DeadVarible);
                                }
                            }
                        }
                        _ => {
                            processed_scenehead.push(t);
                        }
                    }
                }
                token = Token::SceneHead(processed_scenehead);
            }

            Token::EOF => {
                break;
            }
            _ => {}
        }

        tokens.push(token);
        token_line_numbers.push(line_number);
        token = get_next_token(&mut chars, &mut tokenize_mode, &mut scene_nesting_level, &mut line_number);
    }

    // Mark unused variables for removal in AST
    for var_dec in var_names.iter() {
        if !var_dec.has_ref && !var_dec.is_exported {
            tokens[var_dec.index] = Token::DeadVarible;
        }
    }

    tokens.push(token);
    (tokens, token_line_numbers)
}

pub fn get_next_token(
    chars: &mut Peekable<Chars>,
    tokenize_mode: &mut TokenizeMode,
    scene_nesting_level: &mut i64,
    line_number: &mut u32,
) -> Token {
    let mut current_char = match chars.next() {
        Some(ch) => ch,
        None => return Token::EOF,
    };

    let mut token_value: String = String::new();

    // Check for raw strings (backticks)
    // Also used in scenes for raw outputs
    if current_char == '`' {
        while let Some(ch) = chars.next() {
            if ch == '`' {
                return Token::RawStringLiteral(token_value);
            }
            token_value.push(ch);
        }
    }

    if tokenize_mode == &TokenizeMode::Codeblock {
        if scene_nesting_level == &0 {
            *tokenize_mode = TokenizeMode::Normal;
        } else {
            *tokenize_mode = TokenizeMode::Markdown;
        }

        return tokenize_codeblock(chars);
    }

    // Whitespace 
    if current_char == '\n' {
        *line_number += 1;
        return Token::Newline;
    }
    while current_char.is_whitespace() {
        current_char = match chars.next() {
            Some(ch) => ch,
            None => return Token::EOF,
        };
    }

    if current_char == '[' {
        *scene_nesting_level += 1;
        match tokenize_mode {
            TokenizeMode::SceneHead => {
                return Token::Error("Cannot have nested scenes inside of a scene head, must be inside the scene body".to_string(), *line_number);
            }
            TokenizeMode::Codeblock => {
                return Token::Error("Cannot have nested scenes inside of a codeblock".to_string(), *line_number);
            }
            _ => {
                // [] is an empty scene
                if chars.peek() == Some(&']') {
                    chars.next();
                    let mut spaces_after_scene = 0;
                    while let Some(ch) = chars.peek() {
                        if !ch.is_whitespace() || ch == &'\n' {
                            break;
                        }
                        spaces_after_scene += 1;
                        chars.next();
                    }
                    return Token::EmptyScene(spaces_after_scene);
                }

                *tokenize_mode = TokenizeMode::SceneHead;
            }
        }
        return tokenize_scenehead(chars, tokenize_mode, scene_nesting_level, line_number);
    }

    if tokenize_mode == &TokenizeMode::Markdown && current_char != ']' {
        return tokenize_markdown(chars, &mut current_char, line_number);
    }

    if current_char == ']' {
        *scene_nesting_level -= 1;
        if *scene_nesting_level == 0 {
            *tokenize_mode = TokenizeMode::Normal;
            return Token::SceneClose(0);
        }

        *tokenize_mode = TokenizeMode::Markdown;

        // Track spaces after the scene close
        let mut spaces_after_scene = 0;
        while let Some(ch) = chars.peek() {
            if !ch.is_whitespace() || ch == &'\n' {
                break;
            }
            spaces_after_scene += 1;
            chars.next();
        }
        return Token::SceneClose(spaces_after_scene);
    }

    // Initialisation
    // Check if going into markdown mode
    if current_char == ':' {
        if chars.peek() == Some(&':') {
            // ::
            chars.next();
            return Token::Initialise(true);
        }

        if chars.peek() == Some(&'=') {
            chars.next();
            // :=
            return Token::Initialise(false);
        }

        if tokenize_mode == &TokenizeMode::SceneHead {
            *tokenize_mode = TokenizeMode::Markdown;
        }

        return Token::Colon;
    }

    //Window
    if current_char == '#' {
        *tokenize_mode = TokenizeMode::Window;

        //Get compiler directive token
        return keyword_or_variable(&mut token_value, chars, tokenize_mode, line_number);
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
    if current_char == '(' {
        return Token::OpenParenthesis;
    }
    if current_char == ')' {
        return Token::CloseParenthesis;
    }

    // Context Free Grammars
    if current_char == '=' {
        return Token::Assign;
    }
    if current_char == ',' {
        return Token::Comma;
    }
    if current_char == '.' {
        return Token::Dot;
    }
    if current_char == ';' {
        return Token::Semicolon;
    }
    if current_char == '&' {
        return Token::Ref;
    }

    if current_char == '$' {
        // Create new signal variable
        while let Some(&next_char) = chars.peek() {
            if next_char.is_alphanumeric() || next_char == '_' {
                token_value.push(chars.next().unwrap());
            } else {
                return Token::Signal(token_value);
            }
        }
    }

    // Collections
    if current_char == '{' {
        return Token::OpenScope;
    }
    if current_char == '}' {
        return Token::CloseScope;
    }

    //Error handling
    if current_char == '!' {
        return Token::Bang;
    }
    if current_char == '?' {
        return Token::QuestionMark;
    }

    // Comments / Subtraction / Negative / Scene Head / Arrow
    if current_char == '-' {
        if let Some(&next_char) = chars.peek() {
            // Comments
            if next_char == '-' {
                chars.next();

                // Check for multiline
                if let Some(&next_next_char) = chars.peek() {
                    if next_next_char == '\n' {
                        // Mutliline Comment
                        *line_number += 1;
                        chars.next();

                        // Multiline Comment
                        while let Some(ch) = chars.next() {
                            token_value.push(ch);
                            if ch == '\n' {
                                *line_number += 1;
                            }
                            if token_value.ends_with("--") {
                                return Token::MultilineComment(
                                    token_value.trim_end_matches("\n--").to_string(),
                                );
                            }
                        }
                    }

                    // Inline Comment
                    while let Some(ch) = chars.next() {
                        if ch == '\n' {
                            *line_number += 1;
                            return Token::Comment(token_value);
                        }
                        token_value.push(ch);
                    }
                }
            // Subtraction / Negative / Return / Subtract Assign
            } else {
                if next_char == '=' {
                    chars.next();
                    return Token::SubtractAssign;
                }
                if next_char == '>' {
                    chars.next();
                    return Token::Arrow;
                }
                if next_char.is_numeric() {
                    return Token::Negative;
                }
                return Token::Subtract;
            }
        }
    }

    // Mathematical operators,
    // must peak ahead to check for exponentiation (**) or roots (//) and assign variations
    if current_char == '+' {
        if let Some(&next_char) = chars.peek() {
            if next_char == '=' {
                chars.next();
                return Token::AddAssign;
            }
        }
        return Token::Add;
    }
    if current_char == '*' {
        if let Some(&next_char) = chars.peek() {
            if next_char == '*' {
                chars.next();
                return Token::Exponent;
            }
            if next_char == '=' {
                chars.next();
                return Token::MultiplyAssign;
            }
            return Token::Multiply;
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
                return Token::DivideAssign;
            }
            return Token::Divide;
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
                return Token::ExponentAssign;
            }
        }
        return Token::Exponent;
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

    // Exporting variables outside of the module or scope (public declaration)
    if current_char == '@' {
        return Token::Export;
    }

    if current_char == '~' {
        return Token::Bitwise;
    }

    // Numbers
    if current_char.is_numeric() {
        token_value.push(current_char);
        let mut dot_count = 0;

        while let Some(&next_char) = chars.peek() {
            if next_char == '_' {
                chars.next();
                continue;
            }

            if next_char == '.' {
                dot_count += 1;
                // Stop if too many dots           
                if dot_count > 1 {
                    return Token::Error("Cannot have more than one decimal point in a number".to_string(), *line_number);
                }
                token_value.push(chars.next().unwrap());
                continue;
            }

            if next_char.is_numeric() {     
                token_value.push(chars.next().unwrap());
            } else {
                break;
            }
        }

        // ALWAYS PARSE AS FLOAT FOR NOW
        return Token::FloatLiteral(token_value.parse::<f64>().unwrap());
    }

    if current_char.is_alphabetic() {
        token_value.push(current_char);
        return keyword_or_variable(&mut token_value, chars, tokenize_mode, line_number);
    }

    if current_char == '_' {}

    Token::Error(format!("Invalid Token Used (tokenizer). Token: {}", current_char), *line_number)
}

// Nested function because may need multiple searches for variables
fn keyword_or_variable(
    token_value: &mut String,
    chars: &mut Peekable<Chars<'_>>,
    tokenize_mode: &mut TokenizeMode,
    line_number: &u32,
) -> Token {
    // Match variables or keywords
    while let Some(&next_char) = chars.peek() {
        if next_char.is_alphanumeric() || next_char == '_' {
            token_value.push(chars.next().unwrap());
        } else {
            // If their is whitespace or some termination
            // First check if there is a match to a keyword
            // Otherwise break out and check it is a valid variable name
            match token_value.as_str() {
                // Control Flow
                "return" => return Token::Return,
                "if" => return Token::If,
                "else" => return Token::Else,
                "for" => return Token::For,
                "import" => return Token::Import,
                "match" => return Token::Match,
                "break" => return Token::Break,
                "when" => return Token::When,
                "defer" => return Token::Defer,
                "in" => return Token::In,
                "as" => return Token::As,

                // Logical
                "is" => return Token::Equal,
                "not" => return Token::Not,
                "and" => return Token::And,
                "or" => return Token::Or,

                // Keywords
                "io" => return Token::Print,

                
                _ => {}
            }

            if let Some(token) = check_if_datatype_keyword(&token_value) {
                return token;
            }

            // only bother tokenizing / reserving these keywords if inside of a scene head
            match tokenize_mode {
                TokenizeMode::SceneHead => match token_value.as_str() {
                    // Style
                    "rgb" => return Token::Rgb,
                    "hsl" => return Token::Hsl,
                    "code" => return Token::CodeKeyword,
                    "blank" => return Token::Blank,
                    "bg" => return Token::BG,

                    // Theme stuff
                    "clr" => return Token::ThemeColor,

                    // Colour keywords
                    "red" => return Token::Red,
                    "green" => return Token::Green,
                    "blue" => return Token::Blue,
                    "yellow" => return Token::Yellow,
                    "cyan" => return Token::Cyan,
                    "magenta" => return Token::Magenta,
                    "white" => return Token::White,
                    "black" => return Token::Black,
                    "orange" => return Token::Orange,
                    "pink" => return Token::Pink,
                    "purple" => return Token::Purple,
                    "grey" => return Token::Grey,

                    // Layout
                    "pad" => return Token::Padding,
                    "space" => return Token::Margin,
                    "center" => return Token::Center,
                    "size" => return Token::Size,
                    "hide" => return Token::Hide,
                    "nav" => return Token::Nav,
                    "table" => return Token::Table,

                    // Interactive
                    "link" => return Token::A,
                    "button" => return Token::Button,
                    "input" => return Token::Input,
                    "click" => return Token::Click,
                    "form" => return Token::Form,
                    "option" => return Token::Option,
                    "dropdown" => return Token::Dropdown,

                    // Media
                    "img" => return Token::Img,
                    "alt" => return Token::Alt,
                    "video" => return Token::Video,
                    "audio" => return Token::Audio,

                    "order" => return Token::Order,
                    "title" => return Token::Title,

                    // Structure of the page
                    "main" => return Token::Main,
                    "header" => return Token::Header,
                    "footer" => return Token::Footer,
                    "section" => return Token::Section,

                    // Other
                    "ignore" => return Token::Ignore,
                    "canvas" => return Token::Canvas,
                    "redirect" => return Token::Redirect,
                    _ => {}
                },

                TokenizeMode::Window => {
                    *tokenize_mode = TokenizeMode::Normal;

                    if token_value == "date" {
                        return Token::Date;
                    }
                }

                _ => {}
            }

            break;
        }
    }

    if is_valid_identifier(&token_value) {
        return Token::Variable(token_value.to_string());
    }

    Token::Error(format!("Invalid variable name: {}", token_value), *line_number)
}

// Checking if the variable name it valid
fn is_valid_identifier(s: &str) -> bool {
    // Check if the string is a valid identifier (variable name)
    s.chars()
        .next()
        .map_or(false, |c| c.is_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn new_var_or_ref(
    name: String,
    var_names: &mut Vec<Declaration>,
    tokens: &Vec<Token>,
) -> Token {
    let check_if_ref = var_names.into_iter().find(|n| n.name == name);
    let token_index = tokens.len();
    let previous_token = &tokens[token_index - 1];

    match check_if_ref {
        Some(declaration) => {
            declaration.has_ref = true;
            if declaration.is_imported {
                return Token::ConstReference(declaration.name.to_string())
            }

            if tokens.len() <= declaration.index + 1 {
                red_ln!("Error: Something weird when checking variable reference. No proceeding token found after declaration");
                return Token::DeadVarible;
            }
            let token_after = &tokens[declaration.index + 1];

            match token_after {
                Token::Initialise(is_const) => {
                    if *is_const {
                        return Token::ConstReference(declaration.name.to_string())
                    }
                }
                // Probably should have some error handling here if the declaration is weird
                // For now it should just always be a var if not a const
                _ => {}
            }
            Token::VarReference(declaration.name.to_string())
        }
        None => {
            // If the variable is exported, then it counts as having a reference
            // (Does not need to be optimised out by the compiler if no other ref to it in the module)
            let is_public = match previous_token {
                &Token::Export => true,
                _ => false,
            };

            var_names.push(Declaration {
                name: name.to_string(),
                index: token_index,
                has_ref: false,
                is_exported: is_public,
                is_imported: false,
            });
            return Token::VarDeclaration(name);
        }
    }
}

fn check_if_datatype_keyword(token_value: &String) -> Option<Token> {
    match token_value.as_str() {
        "float" => return Some(Token::TypeKeyword(DataType::Float)),
        "string" => return Some(Token::TypeKeyword(DataType::String)),
        "bool" => return Some(Token::TypeKeyword(DataType::Bool)),
        _ => {}
    }
    None
}