WebAssembly.instantiateStreaming(fetch("bs.wasm"), importObject).then(
  (obj) => {
    // Required WASM functions to have proper types and some speed ups
    // Will need to figure out a set of required functions while also not doing too many calls to WASM
    const wasm = obj.instance.exports;

    // string -> string
    // To use proper i32 type
    wasm.parse_int_expr();

    // REMAINING GENERATED JS CODE

  },
);
