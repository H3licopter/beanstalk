#[allow(dead_code)]
#[derive(Debug)]
pub enum AstNode {
    // Basics
    Error(String),
    Comment(String),
    Function(String, Vec<AstNode>),
    VariableDeclaration(String, Box<AstNode>),
    Ref(String),

    //Expressions
    UnaryExpression(String, Box<AstNode>),
    BinaryExpression(String, Box<AstNode>, Box<AstNode>),
}