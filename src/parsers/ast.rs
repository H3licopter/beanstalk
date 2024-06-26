use super::styles::{Style, Tag};
use crate::{bs_types::DataType, Token};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AstNode {
    // Config settings
    Settings(Vec<AstNode>),

    // Blocks
    Function(usize, Box<AstNode>, Vec<AstNode>, bool), // Function name, Args, Body, Public
    Expression(Vec<AstNode>),                          // Expression that can contain mixed types
    RuntimeExpression(Vec<AstNode>, DataType),         //Expression, Result type

    // Basics
    Error(String),
    Comment(String),
    VarDeclaration(usize, Box<AstNode>, bool), // Variable name, Value, Public
    Const(usize, Box<AstNode>, bool),          // Constant name, Value, Public

    // IO
    Print(Box<AstNode>),

    // References to existing variables
    VarReference(usize),
    ConstReference(usize),
    CompileTimeConstReference(usize),
    CompileTimeVarReference(usize),
    FunctionCall(usize, Box<AstNode>),

    // Literals
    Literal(Token),
    Collection(Vec<AstNode>, DataType),
    Struct(usize, Box<AstNode>, bool), // Name, Fields, Public
    Tuple(Vec<AstNode>),
    Scene(Vec<AstNode>, Vec<Tag>, Vec<Style>),
    SceneTemplate,
    Empty, // Empty collection

    // Operators
    // Operator, Precedence
    LogicalOperator(Token, u8), // Negative, Not, Exponent
    Operator(String),           // For shunting yard to handle later as a string

    // HTML
    Element(Token), // HTML element content
    Space,          // Add a space at front of element

    // SCENE META DATA
    Title(String),
    Date(String),
}
