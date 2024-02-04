#[derive(PartialEq)]
pub enum TokenizeMode {
    Normal,
    Markdown,
    SceneHead,
    Meta,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    // For Compiler
    Hash, // Single hash for compiler directives
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
    Url,
    Favicons,

    // Basic syntax
    Comment(String),
    MultilineComment(String),
    DocComment(String),

    // Variables
    Variable(String),
    StringLiteral(String),
    RawStringLiteral(String),
    CharacterLiteral(char),
    IntLiteral(i64),
    FloatLiteral(f64),
    
    // Stucture of Syntax
    Newline,
    CloseScope, // ;

    // Basic Grammar
    Comma,
    Dot,

    // Declarations
    Initialise, // :
    Assign, // =

    // Functions
    OpenBracket,
    CloseBracket,
    As, // For default args in functions

    // Type Declarations
    IntType,
    FloatType,
    StringType,
    Enum,
    RuneType,
    BoolType,
    NoneType, // for "?" data type
    TypeType,

    True,
    False,

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
    CurlyOpen,
    CurlyClose,
    SceneKeyword(String),
    Markdown(String),
    Href,

    // HTTP
    Dollar,

    //HTML element keywords
    Slot, // Injection point for additional template nesting 
    Img,
    Span,

}