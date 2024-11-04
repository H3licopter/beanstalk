use crate::bs_types::DataType;

#[derive(Debug, PartialEq)]
pub enum TokenizeMode {
    Normal,
    Markdown,
    Codeblock,
    SceneHead,
    CompilerDirective, // #
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    // For Compiler
    ModuleStart(String),
    Comptime,
    Error(String, u32),  // Error message, line number
    DeadVarible(String), // Name. Variable that is never used, to be removed in the AST
    EOF,                 // End of file

    // Module Import/Export
    Import,
    Export,

    // HTML project compiler directives
    Page,
    Component,
    Title,
    Date,
    JS(String),  // JS codeblock
    CSS(String), // CSS codeblock

    // Basics
    Settings, // bs keyword
    Print,    // io keyword
    Comment(String),
    MultilineComment(String),
    DocComment(String),

    // Variables / Functions
    Arrow,
    Variable(String),

    // Literals
    StringLiteral(String),
    PathLiteral(String),
    FloatLiteral(f64),
    RawStringLiteral(String),
    BoolLiteral(bool),

    // Collections
    OpenCurly,  // {
    CloseCurly, // }

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
    Colon,  // :
    Assign, // =

    // Scope
    OpenParenthesis,  // (
    CloseParenthesis, // )
    SceneOpen,        // [
    SceneClose(u32),  // Keeps track of the spaces following the scene

    As, // Type casting

    // Type Declarations
    TypeKeyword(DataType),

    FunctionKeyword,

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

    // Control Flow
    If,
    Else,
    ElseIf,
    For,
    In,
    Break,
    Continue, // Might also operate as a fallthrough operator
    Return,
    End,
    When,
    Defer,

    // Scenes
    ParentScene,
    EmptyScene(u32), // Used for templating values in scene heads in the body of scenes, value is numnber of spaces after the scene template

    SceneHead,
    SceneBody,
    Signal(String),

    // HTTP
    Dollar,

    //HTML element stuff
    //markdown inferred elements
    Id,
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
