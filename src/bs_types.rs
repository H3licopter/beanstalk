#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Inferred, // Type is inferred, this only gets to the emitter stage if it will definitely be JS rather than WASM
    Float,    // 32 bit
    Bool,
    True,
    False,
    String, // UTF-8 (will probably just be utf 16 because js for now)

    // Any type can be used in the expression and will be coerced to a string (for scenes only)
    // Mathematical operations will still work and take priority, but strings can be used in these expressions
    // And all types will finally be coerced to strings after everything is evaluated
    CoerseToString,

    Collection(Box<DataType>), // Collection of a single type, dynamically sized
    Struct,
    Scene,
    Choice,
    Type,

    Function(Box<DataType>), // Return types

    Tuple(Box<Vec<DataType>>), // Mixed types (fixed size)

    None, // Maybe only for function returns?
}
