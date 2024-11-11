use crate::{bs_types::DataType, parsers::ast_nodes::AstNode, settings::BS_VAR_PREFIX, Token};
use colour::red_ln;

pub fn new_wat_var(
    id: &String,
    expr: &AstNode,
    datatype: &DataType,
    wat: &mut String,
    wat_global_initilisation: &mut String,
) {
    match datatype {
        DataType::Float => {
            wat.push_str(&format!(
                "
                \n(global ${BS_VAR_PREFIX}{id} (export \"{BS_VAR_PREFIX}{id}\") (mut f64) (f64.const 0))
                \n(func (export \"get_{BS_VAR_PREFIX}{id}\") (result f64) (global.get ${BS_VAR_PREFIX}{id}))",
            ));

            wat_global_initilisation.push_str(&format!(
                "(global.set ${BS_VAR_PREFIX}{id} {})",
                expression_to_wat(&expr)
            ));
        }
        DataType::Int => {
            wat.push_str(&format!(
                "
                \n(global ${BS_VAR_PREFIX}{id} (export \"{BS_VAR_PREFIX}{id}\") (mut i64) (i64.const 0))
                \n(func (export \"get_{BS_VAR_PREFIX}{id}\") (result i64) (global.get ${BS_VAR_PREFIX}{id}))",
            ));

            wat_global_initilisation.push_str(&format!(
                "(global.set ${BS_VAR_PREFIX}{id} {})",
                expression_to_wat(&expr)
            ));
        }
        _ => {
            red_ln!("Unsupported datatype found in WAT var creation");
        }
    }
}

pub fn expression_to_wat(expr: &AstNode) -> String {
    let mut wat = String::new();

    match expr {
        AstNode::RuntimeExpression(nodes, datatype) => match datatype {
            &DataType::Float => {
                return float_expr_to_wat(nodes);
            }
            _ => {
                red_ln!("Unsupported datatype found in expression sent to WAT parser");
            }
        },

        AstNode::Literal(token) => match token {
            Token::FloatLiteral(value) => {
                wat.push_str(&format!("\n(f64.const {})", value.to_string()));
            }
            Token::IntLiteral(value) => {
                wat.push_str(&format!("\n(i64.const {})", value.to_string()));
            }
            _ => {
                red_ln!("unknown literal found in expression");
            }
        },

        _ => {
            red_ln!(
                "Invalid AST node given to expression_to_wat (wat parser): {:?}",
                expr
            );
        }
    }

    wat
}

fn float_expr_to_wat(nodes: &Vec<AstNode>) -> String {
    let mut wat: String = String::new();

    for node in nodes {
        match node {
            AstNode::Literal(token) => {
                match token {
                    Token::FloatLiteral(value) => {
                        wat.push_str(&format!(" f64.const {}", value));
                    }
                    _ => {
                        red_ln!("Compiler error: Wrong literal type found in expression sent to WAT parser");
                    }
                }
            }

            AstNode::VarReference(name, _) | AstNode::ConstReference(name, _) => {
                wat.push_str(&format!(" global.get ${BS_VAR_PREFIX}{name}"));
            }

            AstNode::BinaryOperator(op, _) => {
                let wat_op = match op {
                    Token::Add => " f64.add",
                    Token::Subtract => " f64.sub",
                    Token::Multiply => " f64.mul",
                    Token::Divide => " f64.div",
                    _ => {
                        red_ln!("Unsupported operator found in operator stack when parsing an expression into WAT");
                        return String::new();
                    }
                };

                wat.push_str(wat_op);
            }

            _ => {
                red_ln!("unknown AST node found in expression when parsing float expression into WAT: {:?}", node);
            }
        }
    }

    wat
}

// if operators_stack.len() > 0 && output_stack.len() > 0 {
//     let operator = match operators_stack.pop() {
//         Some(op) => match op {
//             Token::Add => "f64.add",
//             Token::Subtract => "f64.sub",
//             Token::Multiply => "f64.mul",
//             Token::Divide => "f64.div",
//             _ => {
//                 red_ln!("Unsupported operator found in operator stack when parsing an expression into WAT");
//                 return String::new();
//             }
//         }
//         None => {
//             red_ln!("No operator found in operator stack when parsing an expression into WAT");
//             return String::new();
//         }
//     };

//     // CURRENTLY DOES ZERO VALUE IF SOMETHING WENT WRONG HERE
//     let right_value = format!("f64.const {}", value);
//     wat.push_str(&format!("({} ({}) ({}))", operator, output_stack.pop().unwrap_or(String::from("0")), right_value));
// } else {
