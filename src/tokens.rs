#[derive(Debug, PartialEq)]
pub enum TokenizeMode {
    Normal,
    Markdown,
    RawMarkdown,
    SceneHead, //Inline?
    Meta,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    // For Compiler
    Directive(String), // Single hash
    Comptime, // Double hash for comptime
    Meta(Vec<Token>), // Everthing to be executed at compile time
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

    // Basic syntax
    Comment(String),
    MultilineComment(String),
    DocComment(String),

    // Variables
    Variable(String),

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
    Assign, // =

    // Scope
    OpenBracket,
    CloseBracket,
    CollectionOpen,
    CollectionClose,
    SceneOpen,
    SceneClose,

    //Functions
    As, // For default args in functions

    // Type Declarations
    TypeInt,
    TypeFloat,
    TypeDecimal,
    TypeString,
    TypeRune,
    TypeBool,
    True,
    False,
    TypeScene,
    TypeCollection,
    TypeObject,

    // Errors
    Bang,
    QuestionMark,

    //Mathematical Operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Remainder,
    Exponentiation,
    Root,
    MultiplicationAssign,
    DivisionAssign,
    ModulusAssign,
    AdditionAssign,
    SubtractionAssign,
    ExponentiationAssign,
    RootAssign,
    RemainderAssign,
    
    // Logical Operators
    And,
    Or,
    Not,

    //Memory and Pointers
    Pointer,
    Reference,
    Allocate,
    Free,

    // Bitwise Operators
    Bitwise,

    // Comparison Operators
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

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

    // Arrays
    OpenArray,
    CloseArray,

    // Scenes
    ParentScene,
    SceneHead(Vec<Token>), // Scene head properties, inline?
    SceneBody(Vec<Token>),
    Href,

    // HTTP
    Dollar,

    //HTML element stuff
    //markdown inferred elements
    Span(String),
    P(String),
    Heading(u8, String), // Max heading size should be 10 or something
    Empty,
    Pre(String), // Content inside raw elements. Might change to not be a format tag in the future

    // named tags
    A, // href, content 
    Img, // src, alt
    Video,
    Rgb,
    Raw,
    Code,

    Slot, // Injection point for additional template nesting 


}