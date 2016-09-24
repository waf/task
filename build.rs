extern crate serde_codegen;

use std::env;
use std::path::Path;

// build script for serde, see https://serde.rs/codegen-stable.html
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let src = Path::new("src/task_serde_types.in.rs");
    let dst = Path::new(&out_dir).join("task_serde_types.rs");
    serde_codegen::expand(&src, &dst).unwrap();
}
