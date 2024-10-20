use colour::red_ln;

use crate::{bs_types::DataType, parsers::ast_nodes::AstNode, Token};

// Create everything necissary in JS
// Break out pieces in WASM calls
pub fn expression_to_js(expr: &AstNode) -> String {
    let mut js = String::new(); //Open the template string

    match expr {
        AstNode::RuntimeExpression(nodes, expression_type) => {
            for node in nodes {
                match node {
                    AstNode::Literal(token) => match token {
                        Token::FloatLiteral(value) => {
                            js.push_str(&value.to_string());
                        }
                        Token::StringLiteral(value) => {
                            js.push_str(&format!("\"{}\"", value));
                        }
                        _ => {
                            red_ln!("unknown literal found in expression: {:?}", token);
                        }
                    },

                    AstNode::VarReference(name, data_type)
                    | AstNode::ConstReference(name, data_type) => {
                        // If it's a string, it will just be pure JS, no WASM
                        match data_type {
                            DataType::String | DataType::Scene => js.push_str(&format!(" v{name}")),
                            _ => js.push_str(&format!(" wsx.get_v{name}()")),
                        }
                    }

                    AstNode::BinaryOperator(op, _) => match op {
                        Token::Add => js.push_str(" + "),
                        Token::Subtract => js.push_str(" - "),
                        Token::Multiply => js.push_str(" * "),
                        Token::Divide => js.push_str(" / "),
                        _ => {
                            red_ln!("Unsupported operator found in operator stack when parsing an expression into JS: {:?}", op);
                        }
                    },

                    AstNode::Tuple(values, _) => {
                        js.push_str(&format!("[{}]", combine_vec_to_js(values)));
                    }

                    _ => {
                        red_ln!("unknown AST node found in expression when parsing an expression into JS: {:?}", node);
                    }
                }
            }

            match expression_type {
                DataType::String | DataType::Float => {}
                DataType::CoerseToString => {
                    js.insert_str(0, "String(");
                    js.push_str(")");
                }
                _ => {
                    red_ln!(
                        "Have not implimented this type yet in expression_to_js: {:?}",
                        expression_type
                    );
                }
            }
        }

        AstNode::Literal(token) => match token {
            Token::FloatLiteral(value) => {
                js.push_str(&value.to_string());
            }
            Token::StringLiteral(value) => {
                js.push_str(&format!("\"{}\"", value));
            }
            _ => {
                red_ln!("unknown literal found in expression: {:?}", token);
            }
        },

        AstNode::VarReference(name, data_type) | AstNode::ConstReference(name, data_type) => {
            match data_type {
                DataType::String | DataType::Scene => js.push_str(&format!("`${{v{name}}}`")),
                _ => js.push_str(&format!("`${{wsx.get_v{name}()}}`")),
            }
        }

        // If the expression is just a tuple,
        // then it should automatically destructure into multiple arguments like this
        AstNode::Tuple(values, _) => {
            js.push_str(&format!("[{}]", combine_vec_to_js(values)));
        }

        _ => {
            red_ln!(
                "Non-expression / Literal AST node given to expression_to_js: {:?}",
                expr
            );
        }
    }
    js
}

pub fn combine_vec_to_js(collection: &Vec<AstNode>) -> String {
    let mut js = String::new();

    let mut i: usize = 0;
    for node in collection {
        // Make sure correct commas at end of each element but not last one
        js.push_str(&format!(
            "{}{}",
            expression_to_js(&node),
            if i < collection.len() - 1 { ", " } else { "" }
        ));
        i += 1;
    }

    js
}

pub fn collection_to_js(collection: &AstNode) -> String {
    match collection {
        AstNode::Tuple(nodes, _) => {
            return combine_vec_to_js(nodes);
        }
        _ => {
            return "".to_string();
        }
    }
}

pub fn _collection_to_vec_of_js(collection: &AstNode) -> Vec<String> {
    let mut js = Vec::new();

    match collection {
        AstNode::Tuple(nodes, _) => {
            for node in nodes {
                js.push(expression_to_js(node));
            }
        }
        _ => {
            red_ln!("Non-tuple AST node given to collection_to_vec_of_js");
        }
    }

    js
}
