#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Inferred, // Type is inferred
    Int,      // 32 bit signed by default, may add more in future
    Float,    // 64 bit
    Decimal,
    Bool,
    True,
    False,
    String, // UTF-8
    Rune,
    Collection,
    Scene,
    Choice,
}
