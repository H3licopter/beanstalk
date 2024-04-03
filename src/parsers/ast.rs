use super::styles::{Style, Tag};
use crate::{bs_types::DataType, Token};

#[derive(Debug)]
#[allow(dead_code)]
pub enum AstNode {
    // Blocks
    Function(String, Vec<AstNode>, Vec<AstNode>),
    Expression(Vec<AstNode>, DataType), //Expression, Result type

    // Basics
    Error(String),
    Comment(String),
    VarDeclaration(String, Box<AstNode>),
    ConstDeclaration(String, Box<AstNode>),

    // IO
    Print(Box<AstNode>),

    // References to existing variables
    Ref(String),
    FunctionCall(String, Vec<AstNode>),

    // Literals
    Literal(Token),
    Collection(Vec<AstNode>),
    Scene(Vec<AstNode>),

    // Operators
    UnaryOperator(Token),
    BinaryOperator(Token),

    // HTML
    Element(Token),                 // HTML element content
    Space,                          // Add a space at front of element
    SceneTag(Vec<Tag>, Vec<Style>), // Scene wrapping tag / Styles

    // SCENE META DATA
    Title(String),
    Date(String),
}
