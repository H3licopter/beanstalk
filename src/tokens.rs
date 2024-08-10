use crate::bs_types::DataType;

#[derive(Debug, PartialEq)]
pub enum TokenizeMode {
    Normal,
    Markdown,
    Codeblock,
    SceneHead,
    Window,
}
// Stores variable name, it's index and whether it has a reference in the token array
pub struct Declaration {
    pub name: String,
    pub index: usize,
    pub has_ref: bool,
    pub next_token_index: usize,
    pub is_exported: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    // For Compiler
    ModuleStart(String),
    Comptime,
    Error(String),
    DeadVarible, // Variable that is never used, to be removed in the AST
    EOF,         // End of file

    // To later calculate where a token was in the source code
    Whitespace,

    // Module Import/Export
    Import,
    Export,

    // HTML project compiler directives
    Page,
    Component,
    Title,
    Date,

    // Basics
    Settings, // bs keyword
    Print,    // io keyword
    Comment(String),
    MultilineComment(String),
    DocComment(String),

    // Variables / Functions
    Arrow,
    Variable(String),

    // Optimised Variables (Happens just before AST creation)
    VarDeclaration(usize),
    VarReference(usize),
    ConstReference(usize),
    CompileTimeVarReference(usize),
    CompileTimeConstReference(usize),
    Ref, // & operator

    // Literals
    TypeInference,
    StringLiteral(String),
    FloatLiteral(f64),
    RawStringLiteral(String),
    BoolLiteral(bool),

    // Not yet supported
    IntLiteral(i64), 
    RuneLiteral(char),
    DecLiteral(f64), // Will eventually be some bignum type thing

    // Stucture of Syntax
    Newline,
    Semicolon,

    // Basic Grammar
    Comma,
    Dot,
    Colon, // :

    // Declarations
    Assign,         // =
    AssignVariable, // :=
    AssignConstant, // ::

    // Scope
    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenScope,        // {
    CloseScope,       // }
    SceneOpen,        // [
    SceneClose(u32),  // Keeps track of the spaces following the scene

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
    EmptyScene(u32), // Used for templating values in scene heads in the body of scenes, value is numnber of spaces after the scene template
    
    // THIS SHOULD BE CHANGED TO SCENEHEAD OPEN, SCENEBODY OPEN, nest tokens in ast only
    SceneHead(Vec<Token>), // Scene head properties, inline?
    SceneBody(Vec<Token>),
    Signal(String),

    // HTTP
    Dollar,

    //HTML element stuff
    //markdown inferred elements
    Span(String),
    P(String),
    Em(u8, String), // Forms the start and the end of an Em tag
    Superscript(String),
    HeadingStart(u8), // Max heading size should be 10 or something
    BulletPointStart(u8),
    Empty,
    Pre(String), // Content inside raw elements. Might change to not be a format tag in the future

    Ignore, // for commenting out an entire scene

    // named tags
    A,   // href, content
    Img, // src, alt
    Video,
    Audio,
    Raw,

    Alt,

    // Styles
    Padding,
    Margin,
    Size,
    Rgb,
    Hsl,
    BG,
    Table,
    Center,
    CodeKeyword,
    CodeBlock(String),
    Order,
    Blank,
    Hide,

    // Colours
    ThemeColor,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
    Black,
    Orange,
    Pink,
    Purple,
    Grey,

    // Structure of the page
    Main,
    Header,
    Footer,
    Section,
    Gap,

    Nav,
    Button,
    Canvas,
    Click,
    Form,
    Option,
    Dropdown,
    Input,
    Redirect,
}
