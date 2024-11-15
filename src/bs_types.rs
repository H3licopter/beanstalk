use crate::{
    parsers::ast_nodes::{AstNode, Node, Reference},
    Token,
};

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Inferred, // Type is inferred, this only gets to the emitter stage if it will definitely be JS rather than WASM
    Float,
    Int,
    Bool,
    True,
    False,
    String, // UTF-8 (will probably just be utf 16 because js for now)

    // Any type can be used in the expression and will be coerced to a string (for scenes only)
    // Mathematical operations will still work and take priority, but strings can be used in these expressions
    // And all types will finally be coerced to strings after everything is evaluated
    CoerseToString,

    Collection(Box<DataType>), // Collection of a single type, dynamically sized
    Struct,
    Scene,
    Choice,
    Type,

    Style,

    Function(Box<Vec<Reference>>, Box<DataType>), // Arguments, Return type

    Tuple(Box<Vec<DataType>>), // Mixed types (fixed size)

    None, // Maybe only for function returns?
}

pub fn return_datatype(node: &AstNode) -> DataType {
    match node {
        AstNode::RuntimeExpression(_, datatype) => datatype.clone(),
        AstNode::Literal(token) => match token {
            Token::FloatLiteral(_) => DataType::Float,
            Token::IntLiteral(_) => DataType::Int,
            Token::StringLiteral(_) => DataType::String,
            Token::BoolLiteral(value) => {
                if *value {
                    DataType::True
                } else {
                    DataType::False
                }
            }
            _ => DataType::Inferred,
        },
        AstNode::VarReference(_, datatype) | AstNode::ConstReference(_, datatype) => {
            datatype.clone()
        }
        AstNode::Tuple(nodes, _) => {
            let mut types: Vec<DataType> = Vec::new();
            for node in nodes {
                types.push(node.get_type());
            }
            DataType::Tuple(Box::new(types))
        }
        _ => DataType::Inferred,
    }
}
