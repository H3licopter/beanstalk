use super::ast::AstNode;

#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, Clone)]
pub enum Style {
    Padding(AstNode),
    Margin(AstNode),
    Size(AstNode),
    TextColor(AstNode),
    _BackgroundColor(AstNode),
    Alt(String),
}
