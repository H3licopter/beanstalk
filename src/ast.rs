#[allow(dead_code)]
#[derive(Debug)]
pub enum AstNode {
    // Basics
    Comment(String),
    Function(String, Vec<AstNode>),
    VariableDeclaration(String, Box<AstNode>),
    Ref(*const AstNode),

    //Expressions
    UnaryExpression(String, Box<AstNode>),
    BinaryExpression(String, Box<AstNode>, Box<AstNode>),
}