use crate::Token;

use super::ast::AstNode;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Tag {
    None,

    // Structure of the page
    Main,
    Header,
    Footer,
    Section,

    // HTML tags
    Span,
    Div,
    P, // To check whether scene is already inside a P tag
    Heading,
    BulletPoint,
    Em,
    Superscript,
    A(AstNode),     // src
    Img(AstNode),   // src
    Video(AstNode), // src
    Audio(AstNode), // src
    Table(u32),
    Code(String), // Language

    // TO BE IMPLIMENTED
    Nav(AstNode),  // Will be an enum with different types of navs
    List,
    Button,
}

// Will contain an expression or collection of expressions to be parsed in the target language
#[derive(Debug, Clone, PartialEq)]
pub enum Style {
    Padding(AstNode),
    Margin(AstNode),
    Size(AstNode),
    TextColor(AstNode, Token), // Value, type (rgb, hsl)
    BackgroundColor(AstNode),
    Alt(String),
    Center(bool), // true = also center vertically
    Order(AstNode),    // For positioning elements inside a grid/flex container/nav etc
}
