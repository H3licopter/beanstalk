use crate::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AstNode {
    // Basics
    Error(String),
    Comment(String),
    Function(String, Vec<AstNode>),
    VarDeclaration(String, Box<AstNode>),
    ConstDeclaration(String, Box<AstNode>),

    // References to existing variables
    Ref(String),
    FunctionCall(String, Vec<AstNode>),

    // Literals
    Literal(Token),
    Collection(Vec<AstNode>),
    Scene(Vec<AstNode>),

    // HTML
    Element(Token),   // HTML element content
    Space,            // Add a space at front of element
    SceneTag(String), // Scene wrapping tag

    // SCENE META DATA
    Title(String),
    Date(String),
}