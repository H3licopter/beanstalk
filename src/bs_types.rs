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
    String,   // UTF-8
    Rune,     // UTF-32
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
