use crate::Token;
use super::{ast::AstNode, build_ast::new_collection, parse_expression::create_expression};

// Must return one element, but that element can be a collection
// Should also return whether the argument is a literal (compile time constant)
pub fn parse_function_args(scene_head: &Vec<Token>, i: &mut usize) -> AstNode {
    *i += 1;

    if &scene_head[*i] == &Token::OpenCollection {
        return new_collection(&scene_head, i);
    } 

    create_expression(scene_head, i)
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
