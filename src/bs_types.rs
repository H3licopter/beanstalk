#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Inferred, // Type is inferred
    Int,      // 32 bit signed int
    Idx,      // Usize or 32 bit unsigned int
    Float,    // 32 bit
    Decimal,
    Bool,
    True,
    False,
    String,         // UTF-8 (will probably just be utf 16 because js for now)
    Rune,           // UTF-32
    CoerseToString, // Any type can be used in the expression and will be coerced to a string (for scenes only)
    Collection,
    Scene,
    Choice,
    Type,

    // Collections of types
    InferredCollection,

    Tuple, // Mixed types (fixed size)

    IntArray,
    IdxArray,
    FloatArray,
    DecimalArray,
    BoolArray,
    StringArray,
    RuneArray,
    CollectionArray,
    SceneArray,
    ChoiceArray,
}
