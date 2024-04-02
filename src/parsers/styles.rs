#[derive(Debug, PartialEq)]
pub enum Tag {
    None,
    Span,
    Div,
    A(String),     // src
    Img(String),   // src
    Video(String), // src
    Audio(String), // src
}

#[derive(Debug)]
pub enum Style {
    Padding(f64),
    Margin(f64),
    Size(f64, f64),
    TextColor(u8, u8, u8),
    BackgroundColor(u8, u8, u8),
}
