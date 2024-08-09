use colour::red_ln;

use crate::{bs_types::DataType, parsers::ast_nodes::AstNode, Token};

// Create everything necissary in JS
// Break out pieces in WASM calls
pub fn expression_to_js(expr: &AstNode, id: &mut usize, wasm_module: &mut String) -> String {
    let mut js = String::new(); //Open the template string

    match expr {
        // CREATE THE JS CODE FOR THE EXPRESSION -> Uses webassembly functions to handle types properly
        AstNode::RuntimeExpression(nodes, expression_type) => {
            // If numerical type, create a function call to WASM to parse the expression
            match expression_type {
                // NOT SUPPORTING WASM FUNCTIONS YET
                // DataType::Int => {
                //     match wasm_math_expr_fn(nodes, id, expression_type, &mut js) {
                //         Ok(wasm_fn) => {wasm_module.push_str(&wasm_fn)}
                //         Err(err) => {
                //             red_ln!("Error creating WASM function for expression: {:?}", err);
                //             return "".to_string();
                //         }
                //     };
                   
                // }
                DataType::String | DataType::Float | DataType::Int => {
                    js.push_str(&format!("("));
                },
                DataType::CoerseToString => {
                    js.push_str(&format!("String("));
                },
                _ => {
                    red_ln!("Have not implimented this type yet in expression_to_js: {:?}", expression_type);
                }
            }

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

                    AstNode::Operator(op) => {
                        js.push_str(op);
                    }

                    AstNode::Tuple(values, _) => {
                        js.push_str(&format!("[{}]", combine_vec_to_js(values, id, wasm_module)));
                    }

                    _ => {
                        red_ln!("unknown AST node found in expression when parsing an expression into JS");
                    }
                }
            };
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
            js.push_str(&format!("[{}]", combine_vec_to_js(values, id, wasm_module)));
        }

        _ => {
            red_ln!(
                "Non-expression / Literal AST node given to expression_to_js: {:?}",
                expr
            );
        }
    }
    js.push_str(")");
    js
}

pub fn combine_vec_to_js(collection: &Vec<AstNode>, id: &mut usize, wasm_module: &mut String) -> String {
    let mut js = String::new();

    let mut i: usize = 0;
    for node in collection {
        // Make sure correct commas at end of each element but not last one
        js.push_str(&format!(
            "{}{}",
            expression_to_js(&node, id, wasm_module),
            if i < collection.len() - 1 { ", " } else { "" }
        ));
        i += 1;
    }

    js
}

pub fn collection_to_js(collection: &AstNode, id: &mut usize, wasm_module: &mut String) -> String {
    match collection {
        AstNode::Tuple(nodes, _) => {
            return combine_vec_to_js(nodes, id, wasm_module);
        }
        _ => {
            return "".to_string();
        }
    }
}

pub fn _collection_to_vec_of_js(collection: &AstNode, id: &mut usize, wasm_module: &mut String) -> Vec<String> {
    let mut js = Vec::new();

    match collection {
        AstNode::Tuple(nodes, _) => {
            for node in nodes {
                js.push(expression_to_js(node, id, wasm_module));
            }
        }
        _ => {
            red_ln!("Non-tuple AST node given to collection_to_vec_of_js");
        }
    }

    js
}
// BS function created in WASM that can be called from JS
// Naming Convention: wf<id>()
pub fn wasm_math_expr_fn(nodes: &Vec<AstNode>, id: &mut usize, data_type: &DataType, js: &mut String) -> Result<String, &'static str>  {
    js.push_str(&format!("wsx.wf{id}("));
    
    let type_sig = match data_type {
        DataType::Int => { "i64" }
        DataType::Float => { "f64" }

        // TO BE IMPLEMENTED, needs to be growing bit array
        DataType::Decimal => { "f64" }
        _=>{
            red_ln!("Have not implimented this type yet in wasm_math_expr_fn: {:?}", data_type);
            return Err("Have not implimented this type yet in wasm_math_expr_fn")
        }
    };

    let mut wasm_fn = String::from(format!("(func $wf{id} (result {type_sig})"));


    for node in nodes {
        match node {
            AstNode::Literal(token) => match token {
                Token::IntLiteral(value) => {
                    wasm_fn.push_str(&format!("{value}"));
                }
                Token::FloatLiteral(value) => {
                    wasm_fn.push_str(&format!("{value}"));
                }
                _ => {
                    red_ln!("Unsupposed WASM literal found in expression: {:?}", token);
                }
            },

            AstNode::VarReference(name) | AstNode::ConstReference(name) => {
                // Add variable value to WASM function call
                js.push_str(&format!("v{name},"));
                wasm_fn.push_str(&format!(""));
            }

            AstNode::Operator(op) => {
                match op.as_str() {
                    "+" => wasm_fn.push_str("+"),
                    _ => {
                        red_ln!("Unsupported WASM operator found in expression: {:?}", op);
                    }
                }
            }

            _ => {
                red_ln!("unknown AST node found in expression when parsing an expression into JS");
            }
        }
    };

    if js.ends_with(",") {
        js.pop();
    }
    js.push_str(")");

    *id += 1;
    Ok(wasm_fn)
}