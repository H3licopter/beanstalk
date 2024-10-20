use std::path::PathBuf;

use super::styles::{Action, Style, Tag};
use crate::{bs_types::DataType, Token};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AstNode {
    // Config settings
    Settings(Vec<AstNode>),

    // Named import path for the module
    Import(String),

    // Path to a module that will automatically import all styles and scenes
    // into the scope of the current module. Doesn't automatically import variables or functions into the scope
    Use(PathBuf),

    // Blocks
    Function(String, Box<AstNode>, Vec<AstNode>, bool, Vec<DataType>), // Function name, Args, Body, Public
    Expression(Vec<AstNode>, u32), // Expression that can contain mixed types, line number
    RuntimeExpression(Vec<AstNode>, DataType), //Expression, Result type

    // Basics
    Error(String, u32), // Message, line number
    Comment(String),
    VarDeclaration(String, Box<AstNode>, bool, DataType, bool), // Variable name, Value, Public, Type, is_const

    // IO
    Print(Box<AstNode>),

    // References to existing variables
    VarReference(String, DataType),
    ConstReference(String, DataType),
    JSStringReference(String),
    FunctionCall(String, Box<AstNode>), // variable name, arguments

    // Literals
    Literal(Token),
    Collection(Vec<AstNode>, DataType),
    Struct(String, Box<AstNode>, bool), // Name, Fields, Public
    Tuple(Vec<AstNode>, u32),           // Tuple, line number
    Scene(Vec<AstNode>, Vec<Tag>, Vec<Style>, Vec<Action>),
    SceneTemplate,
    Empty, // Empty collection

    // Operators
    // Operator, Precedence
    LogicalOperator(Token, u8), // Negative, Not, Exponent
    BinaryOperator(Token, u8),  // Operator, Precedence
    UnaryOperator(Token, bool), // Operator, is_postfix

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
