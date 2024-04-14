use crate::bs_types::DataType;

#[derive(Debug, PartialEq)]
pub enum TokenizeMode {
    Normal,
    Markdown,
    SceneHead,
    Meta,
}
// Stores variable name, it's index and whether it has a reference in the token array
pub struct Declaration {
    pub name: String,
    pub index: usize,
    pub has_ref: bool,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    // For Compiler
    ModuleStart(String),
    Directive(String), // Single hash
    Comptime,          // Double hash for comptime
    Meta(Vec<Token>),  // Compiler Directive
    Error(String),
    EOF, // End of file

    // Compiler Directives
    Import,
    Export,
    Exclude, // Exclude specific standard library modules

    // HTML project compiler directives
    Main,
    Page,
    Component,
    Title,
    Date,

    // Basics
    Print,
    Comment(String),
    MultilineComment(String),
    DocComment(String),

    // Variables / Functions
    Arrow,
    Variable(String),

    // Optimised Variables (Happens just before AST creation)
    VarDeclaration(usize),
    Reference(usize),

    // Literals
    TypeInference,
    StringLiteral(String),
    RawStringLiteral(String),
    RuneLiteral(char),
    IntLiteral(i64),
    FloatLiteral(f64),
    DecLiteral(f64), // Will eventually be some bignum type thing
    BoolLiteral(bool),

    // Stucture of Syntax
    Newline,
    Semicolon,

    // Basic Grammar
    Comma,
    Dot,

    // Declarations
    Initialise, // :
    Assign,     // =

    // Scope
    OpenParenthesis,
    CloseParenthesis,
    OpenCollection,
    CloseCollection,
    SceneOpen,
    SceneClose(u32), // Keeps track of the spaces following the scene

    As, // Type casting

    // Type Declarations
    TypeKeyword(DataType),

    // Errors
    Bang,
    QuestionMark,

    //Mathematical Operators in order of precedence
    Negative,

    Exponent,
    Multiply,
    Divide,
    Modulus,
    Remainder,
    Root,

    ExponentAssign,
    MultiplyAssign,
    DivideAssign,
    ModulusAssign,
    RootAssign,
    RemainderAssign,

    Add,
    Subtract,
    AddAssign,
    SubtractAssign,

    // Logical Operators in order of precedence
    Not,
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    And,
    Or,

    //Memory and Pointers
    Pointer,
    Allocate,
    Free,

    // Bitwise Operators
    Bitwise,

    // Control Flow
    If,
    Else,
    ElseIf,
    For,
    In,
    Break,
    Continue, // Might also operate as a fallthrough operator
    Return,
    Match,
    When,
    Defer,

    // Scenes
    ParentScene,
    SceneHead(Vec<Token>), // Scene head properties, inline?
    SceneBody(Vec<Token>),
    Href,
    Signal(String),

    // HTTP
    Dollar,

    //HTML element stuff
    //markdown inferred elements
    Span(String),
    P(String),
    Heading(u8, String), // Max heading size should be 10 or something
    BulletPoint(u8, String),
    Superscript(String),
    Empty,       // ALSO USED FOR REMOVED TOKENS
    Pre(String), // Content inside raw elements. Might change to not be a format tag in the future

    // named tags
    A,   // href, content
    Img, // src, alt
    Video,
    Audio,
    Raw,

    Slot, // Injection point for additional template nesting
    Alt,

    // Styles
    Padding,
    Margin,
    Size,
    Rgb,
}
