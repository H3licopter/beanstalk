use super::tokens::{Declaration, Token, TokenizeMode};
use crate::bs_types::DataType;
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize(source_code: &str, module_name: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars: Peekable<Chars<'_>> = source_code.chars().peekable();
    let mut tokenize_mode: TokenizeMode = TokenizeMode::Normal;
    let mut scene_nesting_level: &mut i64 = &mut 0;

    // For variable optimisation
    let mut var_names: Vec<Declaration> = Vec::new();

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
                                Token::CompileTimeVarReference(id) => {
                                    processed_scenehead.push(Token::CompileTimeVarReference(id));
                                }
                                Token::CompileTimeConstReference(id) => {
                                    processed_scenehead.push(Token::CompileTimeConstReference(id));
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
        token = get_next_token(&mut chars, &mut tokenize_mode, &mut scene_nesting_level);
    }

    // Mark unused variables for removal in AST
    for var_dec in var_names.iter() {
        if !var_dec.has_ref {
            tokens[var_dec.index] = Token::DeadVarible;
        }
    }

    tokens.push(token);
    tokens
}

fn get_next_token(
    chars: &mut Peekable<Chars>,
    tokenize_mode: &mut TokenizeMode,
    scene_nesting_level: &mut i64,
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

    if current_char == '[' {
        *scene_nesting_level += 1;
        match tokenize_mode {
            TokenizeMode::SceneHead => {
                return Token::Error("Cannot have nested scenes inside of a scene head, must be inside the scene body".to_string());
            }
            TokenizeMode::Codeblock => {
                return Token::Error("Cannot have nested scenes inside of a codeblock".to_string());
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
        return tokenize_scenehead(chars, tokenize_mode, scene_nesting_level);
    }

    if tokenize_mode == &TokenizeMode::Markdown && current_char != ']' {
        return tokenize_markdown(chars, &mut current_char);
    }

    // Skip non-newline whitespace
    if current_char == '\n' {
        return Token::Newline;
    }
    while current_char.is_whitespace() {
        current_char = match chars.next() {
            Some(ch) => ch,
            None => return Token::EOF,
        };
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
            return Token::AssignConstant;
        }

        if chars.peek() == Some(&'=') {
            chars.next();
            // :=
            return Token::AssignVariable;
        }

        if tokenize_mode == &TokenizeMode::SceneHead {
            *tokenize_mode = TokenizeMode::Markdown;
        }

        return Token::AssignComptime;
    }

    //Window
    if current_char == '#' {
        *tokenize_mode = TokenizeMode::Window;

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
                        chars.next();

                        // Multiline Comment
                        while let Some(ch) = chars.next() {
                            token_value.push(ch);
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

        while let Some(&next_char) = chars.peek() {
            if next_char == '_' {
                chars.next();
                continue;
            }

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

    if current_char == '_' {}

    Token::Error(format!("Invalid Token Used. Token: {}", current_char))
}

// Nested function because may need multiple searches for variables
fn keyword_or_variable(
    token_value: &mut String,
    chars: &mut Peekable<Chars<'_>>,
    tokenize_mode: &mut TokenizeMode,
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

                // Data Types
                "int" => return Token::TypeKeyword(DataType::Int),
                "idx" => return Token::TypeKeyword(DataType::Int),
                "flt" => return Token::TypeKeyword(DataType::Float),
                "str" => return Token::TypeKeyword(DataType::String),
                "uni" => return Token::TypeKeyword(DataType::Rune),
                "bool" => return Token::TypeKeyword(DataType::Bool),
                "dec" => return Token::TypeKeyword(DataType::Decimal),
                _ => {}
            }

            // only bother tokenizing / reserving these keywords if inside of a scene head
            match tokenize_mode {
                TokenizeMode::SceneHead => match token_value.as_str() {
                    "link" => return Token::A,
                    "m" => return Token::Margin,
                    "p" => return Token::Padding,
                    "img" => return Token::Img,
                    "alt" => return Token::Alt,
                    "rgb" => return Token::Rgb,
                    "center" => return Token::Center,
                    "bg" => return Token::BG,
                    "size" => return Token::Size,
                    "table" => return Token::Table,
                    "video" => return Token::Video,
                    "audio" => return Token::Audio,
                    "slot" => return Token::Slot,
                    "ignore" => return Token::Ignore,
                    "code" => return Token::CodeKeyword,
                    _ => {}
                },

                TokenizeMode::Window => {
                    *tokenize_mode = TokenizeMode::Normal;

                    if token_value == "date" {
                        return Token::Date;
                    }
                    if token_value == "title" {
                        return Token::Title;
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

    Token::Error(format!("Invalid variable name: {}", token_value))
}

// Checking if the variable name it valid
fn is_valid_identifier(s: &str) -> bool {
    // Check if the string is a valid identifier (variable name)
    s.chars()
        .next()
        .map_or(false, |c| c.is_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn tokenize_scenehead(
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
fn tokenize_markdown(chars: &mut Peekable<Chars>, current_char: &mut char) -> Token {
    let mut content = String::new(); // To keep track of current chars being parsed
    let mut previous_newlines = 0;
    let mut current_token = Token::Empty;

    //Ignore starting whitespace (except newlines)
    while current_char.is_whitespace() {
        if current_char == &'\n' {
            if content.ends_with("\n\n") {
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
                    // Break in the hashes
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
                        }, 
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
fn tokenize_codeblock(chars: &mut Peekable<Chars>) -> Token {
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

pub fn new_var_or_ref(
    name: String,
    var_names: &mut Vec<Declaration>,
    tokens: &Vec<Token>,
) -> Token {
    let check_if_ref = var_names.iter().rposition(|n| n.name == name);
    let token_index = tokens.len();
    let previous_token = &tokens[token_index - 1];

    match check_if_ref {
        Some(index) => {
            var_names[index].has_ref = true;

            // POSSIBLE OUT OF BOUNDS ERROR TO SORT OUT??? or will that never happen because of the check_if_ref?
            let token_after = &tokens[var_names[index].next_token_index];

            if token_after == &Token::AssignConstant {
                return Token::ConstReference(var_names[index].index);
            }

            if token_after == &Token::AssignComptimeVariable {
                return Token::CompileTimeVarReference(var_names[index].index);
            }

            if token_after == &Token::AssignComptime {
                return Token::CompileTimeConstReference(var_names[index].index);
            }

            return Token::VarReference(var_names[index].index);
        }
        None => {
            // If the variable is exported, then it counts as having a reference
            // (Does not need to be optimised out by the compiler if no other ref to it in the module)
            let is_public = match previous_token {
                &Token::Export => true,
                _ => false,
            };

            var_names.push(Declaration {
                name,
                index: token_index,
                has_ref: is_public,
                next_token_index: token_index + 1,
            });
            return Token::VarDeclaration(token_index);
        }
    }
}