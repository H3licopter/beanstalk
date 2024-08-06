use std::fs;
use std::path::{Path, PathBuf};
use colour::red_ln;
use wat::parse_file;

pub fn compile_wat_file(path: &Path) {
    let wasm = parse_file(path);
    match wasm {
        Ok(wasm) => {
            println!("Compiling: {:?} to WASM", path.file_stem().unwrap());
            let output_path = PathBuf::from(path.parent().unwrap_or(Path::new(""))).join("/dev/pkg/wasm_test_output.wasm");
            let fs_result = fs::write(output_path, wasm);
            match fs_result {
                Ok(_) => {
                    println!("WASM compiled successfully");
                }
                Err(e) => red_ln!("Error writing WASM: {:?}", e),
            }
        }
        Err(e) => red_ln!("Error compiling WAT to WebAssembly: {:?}", e),
    }
}