use std::string;

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
    StringLiteral(String),
    RawStringLiteral(String),
    RuneLiteral(char),
    IntLiteral(i64),
    FloatLiteral(f64),
    DecLiteral(f64), // Will eventually be some bignum type thing
    BoolLiteral(bool),
    Collection(Vec<AstNode>),
    Scene(Vec<AstNode>), // Scene, Properties
    
    // HTML
    HTML(String), // Content
    Page,
    Title(String),
    Date(String),

    //Expressions
    UnaryExpression(String, Box<AstNode>),
    BinaryExpression(String, Box<AstNode>, Box<AstNode>),
}