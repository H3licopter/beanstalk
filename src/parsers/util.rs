use crate::Token;

pub fn _op_precedence(operator: Token) -> i8 {
    match operator {
        Token::Negative => 10,
        Token::Not => 10,

        Token::Root => 8,
        Token::RootAssign => 8,
        Token::Exponent => 8,
        Token::ExponentAssign => 8,

        Token::Multiply => 7,
        Token::Divide => 7,
        Token::Modulus => 7,
        Token::Remainder => 7,

        Token::MultiplyAssign => 7,
        Token::DivideAssign => 7,
        Token::ModulusAssign => 7,
        Token::RemainderAssign => 7,

        Token::Add => 6,
        Token::Subtract => 6,
        Token::AddAssign => 6,
        Token::SubtractAssign => 6,

        Token::Equal => 5,
        Token::LessThan => 5,
        Token::LessThanOrEqual => 5,
        Token::GreaterThan => 5,
        Token::GreaterThanOrEqual => 5,

        Token::And => 4,
        Token::Or => 3,

        _ => 0,
    }
}

pub fn count_newlines_at_end_of_string(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars().rev() {
        if c == '\n' {
            count += 1;
            continue;
        }

        if c.is_whitespace() {
            continue;
        }

        break;
    }

    count
}

pub fn count_newlines_at_start_of_string(s: &str) -> usize {
    let mut count = 0;

    for c in s.chars() {
        if c == '\n' {
            count += 1;
            continue;
        }
        break;
    }

    count
}
