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
    _Padding(f64),
    _Margin(f64),
    Size(f64, f64),
    TextColor(u8, u8, u8),
    _BackgroundColor(u8, u8, u8),
    Alt(String),
}
