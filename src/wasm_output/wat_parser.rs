use colour::red_ln;
use crate::{bs_types::DataType, parsers::ast_nodes::AstNode, Token};

pub fn expression_to_wat(expr: &AstNode) -> String {
    let mut wat = String::new();

    match expr {
        AstNode::RuntimeExpression(nodes, datatype) => {
            match datatype {
                &DataType::Float => {
                    return float_expr_to_wat(nodes);
                }
                _ => {
                    red_ln!("Unsupported datatype found in expression sent to WAT parser");
                }
            }
        }

        AstNode::Literal(token) => match token {
            Token::FloatLiteral(value) => {
                wat.push_str(&format!("\n(f64.const {})", value.to_string()));
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

struct Operator {
    precedence: u8,
    wat: &'static str,
}

// SHUNTING YARD ALGORITHM
fn float_expr_to_wat(nodes: &Vec<AstNode>) -> String {
    let mut wat: String = String::new();
    let mut output_stack: Vec<String> = Vec::new();
    let mut operators_stack: Vec<Operator> = Vec::new();

    'outer: for node in nodes {
        match node {
            AstNode::Literal(token) => match token {
                Token::FloatLiteral(value) => {
                    output_stack.insert(0, format!(" f64.const {}", value));
                }
                _ => {
                    red_ln!("Compiler error: Wrong literal type found in expression sent to WAT parser");
                }
            },

            AstNode::VarReference(name, _) | AstNode::ConstReference(name, _) => {
                output_stack.insert(0, format!(" global.get $v{name}"));
            }

            AstNode::BinaryOperator(op, precedence) => {
                let wat_op = match op {
                    Token::Add => Operator { precedence: *precedence, wat: " f64.add"},
                    Token::Subtract => Operator { precedence: *precedence, wat: " f64.sub"},
                    Token::Multiply => Operator { precedence: *precedence, wat: " f64.mul"},
                    Token::Divide => Operator { precedence: *precedence, wat: " f64.div"},
                    _ => {
                        red_ln!("Unsupported operator found in operator stack when parsing an expression into WAT");
                        return String::new();
                    }
                };

                for i in 0..operators_stack.len() {
                    if &operators_stack[i].precedence < precedence {
                        output_stack.push(operators_stack[i].wat.to_string());
                        operators_stack[i] = wat_op;
                        continue 'outer;
                    }
                }
                
                operators_stack.push(wat_op);
            }

            _ => {
                red_ln!("unknown AST node found in expression when parsing float expression into WAT: {:?}", node);
            }
        }
    };

    for operator in operators_stack {
        output_stack.push(operator.wat.to_string());
    }

    for value in output_stack {
        wat.push_str(&value);
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