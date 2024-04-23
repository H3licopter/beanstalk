use super::ast::AstNode;

#[derive(Debug, PartialEq)]
pub enum Tag {
    None,
    Span,
    Div,
    A(String),     // src
    Img(String),   // src
    Video(String), // src
    Audio(String), // src
}

// Will contain an expression or collection of expressions to be parsed in the target language
#[derive(Debug)]
pub enum Style {
    Padding(AstNode),
    Margin(AstNode),
    Size(AstNode),
    TextColor(AstNode),
    _BackgroundColor(AstNode),
    Alt(String),
}
