#[derive(Debug, Clone)]
pub enum DataType {
    Inffered,
    Int,
    Float,
    Decimal,
    Bool,
    String,
    Rune,
    Collection,
}