use crate::Token;

#[derive(Debug)]
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
    Scene(Vec<AstNode>), // Scene, Properties
    
    // HTML
    Block(Token), // Block HTML element content
    Inline(Token), // Inline HTML content

    // SCENE META DATA
    Title(String),
    Date(String),
}