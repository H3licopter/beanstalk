use colour::red_ln;

use crate::{parsers::ast_nodes::AstNode, Token};

pub fn expression_to_js(expr: &AstNode) -> String {
    let mut js = String::new(); //Open the template string

    match expr {
        // CREATE THE JS CODE FOR THE EXPRESSION -> Uses webassembly functions to handle types properly
        AstNode::RuntimeExpression(nodes, _) => {
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
                            red_ln!("unknown literal found in expression");
                        }
                    },

                    AstNode::VarReference(name) | AstNode::ConstReference(name) => {
                        js.push_str(&format!(" ${{v{name}}} "));
                    }

                    // AstNode::FunctionCall(name, arg) => {
                    //     let mut js_args = "".to_string();
                    //     match &**arg {
                    //         AstNode::Tuple(values) => {
                    //             js_args = combine_vec_to_js(values);
                    //         }
                    //         AstNode::EvaluatedExpression(_, _) => {
                    //             js_args = expression_to_js(arg);
                    //         }
                    //         _ => {
                    //             println!("unknown AST node found in function call");
                    //         }
                    //     }
                    //     js.push_str(&format!("f{}({:?})", name, js_args));
                    // }
                    AstNode::Operator(op) => {
                        js.push_str(op);
                    }

                    AstNode::Tuple(values, _) => {
                        js.push_str(&format!("[{}]", combine_vec_to_js(values)));
                    }

                    _ => {
                        red_ln!("unknown AST node found in expression when parsing an expression into JS");
                    }
                }
            }

            // OLD: WRAP IN THE WEBASSEMBLY FUNCTION CALL
            // Need to generate webassembly function that has an exported ID that JS can call
            // Naming Convention: wf<id>() 
            // match expression_type {
            //     DataType::Int => {
            //         js.insert_str(0, "parse_int_expr(`");
            //         js.push_str("`)");
            //     }
            //     DataType::Float => {
            //         js.insert_str(0, "parse_float_expr(`");
            //         js.push_str("`)");
            //     }
            //     DataType::String => {}
            //     DataType::CoerseToString => {
            //         js.insert_str(0, "String(");
            //         js.push_str(")");
            //     }
            //     _ => {
            //         red_ln!("Have not implimented this type yet in expression_to_js");
            //     }
            // }

            // Temporary directly into JS
            js.insert_str(0, "(");
            js.push_str(")");
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
                red_ln!("unknown literal found in expression");
            }
        },

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

/*
                    AstNode::Const(name, value, _) => {
                        match &**value {
                            AstNode::EvaluatedExpression(_, _) => {
                                js.push_str(&format!(
                                    "const cv{} = {}",
                                    name,
                                    expression_to_js(value)
                                ));
                            }
                            AstNode::Tuple(values) => {
                                js.push_str(&format!(
                                    "const tv{} = [{}]",
                                    name,
                                    combine_vec_to_js(values)
                                ));
                            }
                            _ => {
                                println!("unknown AST node found in const declaration");
                            }
                        }
                    }
*/
