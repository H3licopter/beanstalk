use colour::red_ln;
use std::fs;
use std::path::{Path, PathBuf};
use wat::parse_file;

pub fn compile_wat_file(path: &Path) {
    let wasm = parse_file(path);
    match wasm {
        Ok(wasm) => {
            let file_stem = match path.file_stem() {
                Some(s) => PathBuf::from(s).with_extension("wasm"),
                None => PathBuf::from("wasm.wasm"),
            };
            println!("Compiling: {:?} to WASM", file_stem);

            let parent_folder = match path.parent() {
                Some(p) => p,
                None => Path::new(""),
            };

            let output_path = PathBuf::from(parent_folder).join(file_stem);
            match fs::write(output_path, wasm) {
                Ok(_) => {
                    println!("WASM compiled successfully");
                }
                Err(e) => red_ln!("Error writing WASM: {:?}", e),
            }
        }
        Err(e) => red_ln!("Error compiling WAT to WebAssembly: {:?}", e),
    }
}
