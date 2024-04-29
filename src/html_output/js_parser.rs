use crate::{parsers::ast::AstNode, Token};

// JS will also need to call into the prebuilt webassembly functions
// Also parses literals
pub fn expression_to_js(expr: &AstNode) -> String {
    let mut js = String::new();

    match expr {
        AstNode::EvaluatedExpression(nodes, _) | AstNode::Expression(nodes) => {
            for node in nodes {
                match node {
                    AstNode::Literal(token) => match token {
                        Token::IntLiteral(value) => {
                            js.push_str(&value.to_string());
                        }
                        Token::FloatLiteral(value) => {
                            js.push_str(&value.to_string());
                        }
                        Token::StringLiteral(value) => {
                            js.push_str(&format!("\"{}\"", value));
                        }
                        _ => {
                            println!("unknown literal found in expression");
                        }
                    },

                    AstNode::VarReference(name) => {
                        js.push_str(&format!("v{}", name));
                    }
                    AstNode::ConstReference(name) => {
                        js.push_str(&format!("c{}", name));
                    }
                    AstNode::FunctionCall(name, arg) => {
                        let mut js_args = "".to_string();
                        match &**arg {
                            AstNode::Tuple(values) => {
                                js_args = combine_vec_to_js(values);
                            }
                            AstNode::EvaluatedExpression(_, _) => {
                                js_args = expression_to_js(arg);
                            }
                            _ => {
                                println!("unknown AST node found in function call");
                            }
                        }
                        js.push_str(&format!("f{}({:?})", name, js_args));
                    }

                    AstNode::Const(name, value, _) => {
                        match &**value {
                            AstNode::EvaluatedExpression(_, _) => {
                                js.push_str(&format!(
                                    "const c{} = {}",
                                    name,
                                    expression_to_js(value)
                                ));
                            }
                            AstNode::Tuple(values) => {
                                js.push_str(&format!(
                                    "const c{} = [{}]",
                                    name,
                                    combine_vec_to_js(values)
                                ));
                            }
                            _ => {
                                println!("unknown AST node found in const declaration");
                            }
                        }
                        js.push_str(&format!("const c{} = {}", name, expression_to_js(value)));
                    }

                    AstNode::BinaryOperator(operator, _) => match operator {
                        Token::Add => {
                            js.push_str(" + ");
                        }
                        Token::Subtract => {
                            js.push_str(" - ");
                        }
                        Token::Multiply => {
                            js.push_str(" * ");
                        }
                        Token::Divide => {
                            js.push_str(" / ");
                        }
                        Token::Modulus => {
                            js.push_str(" % ");
                        }
                        Token::Equal => {
                            js.push_str(" === ");
                        }
                        Token::GreaterThan => {
                            js.push_str(" > ");
                        }
                        Token::LessThan => {
                            js.push_str(" < ");
                        }
                        Token::GreaterThanOrEqual => {
                            js.push_str(" >= ");
                        }
                        Token::LessThanOrEqual => {
                            js.push_str(" <= ");
                        }
                        Token::And => {
                            js.push_str(" && ");
                        }
                        Token::Or => {
                            js.push_str(" || ");
                        }
                        _ => {
                            println!("unknown binary operator found in expression");
                        }
                    },

                    AstNode::UnaryOperator(operator, _) => match operator {
                        Token::Negative => {
                            js.push_str(" -");
                        }
                        Token::Not => {
                            js.push_str(" !");
                        }
                        Token::Exponent => {
                            js.push_str(" ** ");
                        }
                        _ => {
                            println!("unknown unary operator found in expression");
                        }
                    },

                    AstNode::Tuple(values) => {
                        js.push_str(&format!("[{}]", combine_vec_to_js(values)));
                    }

                    _ => {
                        println!("unknown AST node found in expression when parsing an expression into JS");
                    }
                }
            }
        }

        AstNode::Literal(token) => match token {
            Token::IntLiteral(value) => {
                js.push_str(&value.to_string());
            }
            Token::FloatLiteral(value) => {
                js.push_str(&value.to_string());
            }
            Token::StringLiteral(value) => {
                js.push_str(&format!("\"{}\"", value));
            }
            _ => {
                println!("unknown literal found in expression");
            }
        },

        // If the expression is just a tuple,
        // then it should automatically destructure into multiple arguments like this
        AstNode::Tuple(values) => {
            js.push_str(&format!("{}", combine_vec_to_js(values)));
        }

        _ => {
            println!(
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
        AstNode::Tuple(nodes) => {
            return combine_vec_to_js(nodes);
        }
        _ => {
            return "".to_string();
        }
    }
}

pub fn collection_to_vec_of_js(collection: &AstNode) -> Vec<String> {
    let mut js = Vec::new();

    match collection {
        AstNode::Tuple(nodes) => {
            for node in nodes {
                js.push(expression_to_js(node));
            }
        }
        _ => {
            println!("Non-tuple AST node given to collection_to_vec_of_js");
        }
    }

    js
}
