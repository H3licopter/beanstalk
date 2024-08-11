use super::styles::{Style, Tag};
use crate::{bs_types::DataType, Token};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AstNode {
    // Config settings
    Settings(Vec<AstNode>),

    // Blocks
    Function(usize, Box<AstNode>, Vec<AstNode>, bool, Vec<DataType>), // Function name, Args, Body, Public
    Expression(Vec<AstNode>, u32),                      // Expression that can contain mixed types, line number
    RuntimeExpression(Vec<AstNode>, DataType),         //Expression, Result type

    // Basics
    Error(String, u32), // Message, line number
    Comment(String),
    VarDeclaration(usize, Box<AstNode>, bool, DataType), // Variable name, Value, Public, Type
    Const(usize, Box<AstNode>, bool, DataType),          // Constant name, Value, Public, Type

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
    Tuple(Vec<AstNode>, u32), // Tuple, line number
    Scene(Vec<AstNode>, Vec<Tag>, Vec<Style>),
    SceneTemplate,
    Empty, // Empty collection

    // Operators
    // Operator, Precedence
    LogicalOperator(Token, u8), // Negative, Not, Exponent
    Operator(String),           // For shunting yard to handle later as a string

    // HTML
    Element(Token), // HTML element content
    Heading(u8),
    BulletPoint(u8),
    Em(u8, String),
    Superscript(String),
    Space, // Add a space at front of element

    // SCENE META DATA
    Title(String),
    Date(String),
}
