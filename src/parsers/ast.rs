use super::styles::{Style, Tag};
use crate::{bs_types::DataType, Token};

#[derive(Debug)]
#[allow(dead_code)]
pub enum AstNode {
    // Config settings
    Config(Vec<AstNode>),
    Project(Vec<AstNode>),

    // Blocks
    Function(usize, Box<AstNode>, Vec<AstNode>),
    Expression(Vec<AstNode>, DataType), //Expression, Result type

    // Basics
    Error(String),
    Comment(String),
    VarDeclaration(usize, Box<AstNode>),
    Const(usize, Box<AstNode>),
    
    // IO
    Print(Box<AstNode>),

    // References to existing variables
    VarReference(usize),
    ConstReference(usize),
    FunctionCall(usize, Vec<AstNode>),

    // Literals
    Literal(Token),
    Collection(Vec<AstNode>),
    Scene(Vec<AstNode>),

    // Operators
    UnaryOperator(Token, u8),  // Operator, Precedence
    BinaryOperator(Token, u8), // Operator, Precedence

    // HTML
    Element(Token),                 // HTML element content
    Space,                          // Add a space at front of element
    SceneTag(Vec<Tag>, Vec<Style>), // Scene wrapping tag / Styles

    // SCENE META DATA
    Title(String),
    Date(String),
}
