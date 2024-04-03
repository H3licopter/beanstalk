#[derive(Debug, Clone)]
pub enum DataType {
    Inffered, // Type is inferred
    Int,      // 32 bit signed by default, may add more in future
    Float,    // 64 bit
    Decimal,
    Bool,
    String, // UTF-8
    Rune,
    Collection,
}
