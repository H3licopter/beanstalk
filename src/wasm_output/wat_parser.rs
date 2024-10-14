use colour::red_ln;
use crate::{parsers::ast_nodes::AstNode, Token};

pub fn expression_to_wat(expr: &AstNode) -> String {
    let mut wat = String::new();

    let mut values_stack: Vec<String> = Vec::new();
    let mut operators_stack: Vec<&Token> = Vec::new();

    match expr {
        AstNode::RuntimeExpression(nodes, _) => {
            for node in nodes {
                match node {
                    AstNode::Literal(token) => match token {
                        Token::FloatLiteral(value) => {
                            if operators_stack.len() > 0 && values_stack.len() > 0 {
                                let operator = match operators_stack.pop() {
                                    Some(op) => match op {
                                        Token::Add => "f64.add",
                                        Token::Subtract => "f64.sub",
                                        Token::Multiply => "f64.mul",
                                        Token::Divide => "f64.div",
                                        _ => {
                                            red_ln!("Unsupported operator found in operator stack when parsing an expression into WAT");
                                            return String::new();
                                        }
                                    }
                                    None => {
                                        red_ln!("No operator found in operator stack when parsing an expression into WAT");
                                        return String::new();
                                    }
                                };

                                // CURRENTLY DOES ZERO VALUE IF SOMETHING WENT WRONG HERE
                                let right_value = format!("f64.const {}", value);
                                wat.push_str(&format!("({} ({}) ({}))", operator, values_stack.pop().unwrap_or(String::from("0")), right_value));
                            } else {
                                values_stack.push(format!("f64.const {}", value));
                            }
                        }
                        _ => {
                            red_ln!("Unsupported literal found in expression sent to WAT parser");
                        }
                    },

                    AstNode::VarReference(name) | AstNode::ConstReference(name) => {
                        values_stack.push(format!("global.get $v{name}"));
                    }

                    AstNode::BinaryOperator(op) => {
                        if operators_stack.len() > 0 {
                            red_ln!("Operator stack already has an operator in it when parsing an expression into WAT");
                        }
                        operators_stack.push(op);
                    }

                    _ => {
                        red_ln!("unknown AST node found in expression when parsing an expression into JS");
                    }
                }
            };
        }

        AstNode::Literal(token) => match token {
            Token::FloatLiteral(value) => {
                wat.push_str(&format!("\nf64.const {} ", value.to_string()));
            }
            _ => {
                red_ln!("unknown literal found in expression");
            }
        },

        _ => {
            red_ln!(
                "Non-expression / Literal AST node given to expression_to_js: {:?}",
                expr
            );
        }
    }

    wat
}