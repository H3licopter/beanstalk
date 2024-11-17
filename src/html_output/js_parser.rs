use colour::red_ln;

use crate::{bs_types::DataType, parsers::ast_nodes::AstNode, settings::BS_VAR_PREFIX, Token};

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
                        Token::IntLiteral(value) => {
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
                            DataType::String | DataType::Scene => {
                                js.push_str(&format!(" {BS_VAR_PREFIX}{name}"))
                            }
                            _ => js.push_str(&format!(" wsx.get_{BS_VAR_PREFIX}{name}()")),
                        }
                    }

                    AstNode::CollectionAccess(name, index, data_type)
                    | AstNode::TupleAccess(name, index, data_type) => {
                        // If it's a string, it will just be pure JS, no WASM

                        // CURRENTLY WORKS AS TUPLES DON'T SUPPORT NAMED ARGS YET,
                        // so it's all just accessing an array or struct by number
                        match data_type {
                            DataType::String | DataType::Scene => {
                                js.push_str(&format!(" {BS_VAR_PREFIX}{name}_{index}"))
                            }
                            _ => js.push_str(&format!(" wsx.get_{BS_VAR_PREFIX}{name}_{index}")),
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
                DataType::String | DataType::Float | DataType::Int => {}
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
            Token::IntLiteral(value) => {
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
                DataType::String | DataType::Scene => {
                    js.push_str(&format!("`${{{BS_VAR_PREFIX}{name}}}`"))
                }
                _ => js.push_str(&format!("`${{wsx.get_{BS_VAR_PREFIX}{name}()}}`")),
            }
        }

        // If the expression is just a tuple,
        // then it should automatically destructure into multiple arguments like this
        AstNode::Tuple(values, _) => {
            js.push_str(&format!("[{}]", combine_vec_to_js(values)));
        }

        AstNode::FunctionCall(name, arguments, _) => {
            js.push_str(&function_call_to_js(name, *arguments.to_owned()));
        }

        _ => {
            red_ln!("Invalid AST node given to expression_to_js: {:?}", expr);
        }
    }

    js
}

pub fn create_reference_in_js(name: &String, data_type: &DataType) -> String {
    match data_type {
        DataType::String | DataType::Scene | DataType::Inferred | DataType::CoerseToString => {
            format!("uInnerHTML(\"{name}\", {BS_VAR_PREFIX}{name});")
        }
        _ => {
            format!("uInnerHTML(\"{name}\", wsx.get_{BS_VAR_PREFIX}{name}());")
        }
    }
}

pub fn function_call_to_js(name: &String, argument: AstNode) -> String {
    let mut js = format!("{BS_VAR_PREFIX}{name}(");

    match argument {
        AstNode::Empty => {}
        AstNode::Literal(token) => match token {
            Token::StringLiteral(value) => {
                js.push_str(&format!("\"{}\",", value));
            }
            Token::FloatLiteral(value) => {
                js.push_str(&format!("{},", value));
            }
            Token::IntLiteral(value) => {
                js.push_str(&format!("{},", value));
            }
            Token::BoolLiteral(value) => {
                js.push_str(&format!("{},", value));
            }
            _ => {}
        },
        AstNode::CollectionAccess(collection_name, index_accessed, _)
        | AstNode::TupleAccess(collection_name, index_accessed, _) => {
            js.push_str(&format!("{collection_name}[{index_accessed}],"));
        }
        AstNode::RuntimeExpression(expr, data_type) => {
            js.push_str(&format!(
                "{},",
                expression_to_js(&AstNode::RuntimeExpression(
                    expr.clone(),
                    data_type.to_owned(),
                ))
            ));
        }
        AstNode::VarReference(name, _) | AstNode::ConstReference(name, _) => {
            js.push_str(&format!("{},", name));
        }
        AstNode::FunctionCall(function_name, args, _) => {
            js.push_str(&function_call_to_js(&function_name, *args));
        }
        _ => {
            red_ln!(
                "Web Parser Error: Invalid argument type for function call: {:?}",
                argument
            );
        }
    }

    js.push_str(") ");
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
            if i < collection.len() - 1 { "," } else { "" }
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
            red_ln!("Non-tuple AST node given to collection_to_js");
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
