use crate::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AstNode {
    // Blocks
    Function(String, Vec<AstNode>, Vec<AstNode>),
    Expression(Vec<AstNode>),

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
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponent,

    // HTML
    Element(Token),   // HTML element content
    Space,            // Add a space at front of element
    SceneTag(String), // Scene wrapping tag

    // SCENE META DATA
    Title(String),
    Date(String),
}
