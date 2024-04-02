#[derive(Debug, PartialEq)]
pub enum Tag {
    None,
    Span,
    Div,
    P,
    A(String),     // src
    Img(String),   // src
    Video(String), // src
    Audio(String), // src
}

#[derive(Debug)]
pub enum Style {
    Width(f64),
    Height(f64),
    TextColor(u8, u8, u8),
    BackgroundColor(u8, u8, u8),
}