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
    EvaluatedExpression(Vec<AstNode>, DataType),       //Expression, Result type

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
    FunctionCall(usize, Box<AstNode>),

    // Literals
    Literal(Token),
    Collection(Vec<AstNode>, DataType),
    Struct(usize, Box<AstNode>, bool), // Name, Fields, Public
    Tuple(Vec<AstNode>),
    Scene(Vec<AstNode>),
    Empty, // Empty collection

    // Operators
    // Operator, Precedence
    UnaryOperator(Token, u8), // Negative, Not, Exponent
    BinaryOperator(Token, u8),

    // HTML
    Element(Token),                 // HTML element content
    Space,                          // Add a space at front of element
    SceneTag(Vec<Tag>, Vec<Style>), // Scene wrapping tag / Styles

    // SCENE META DATA
    Title(String),
    Date(String),
}
