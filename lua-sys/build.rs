extern crate metadeps;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    metadeps::probe().unwrap();

    // Configure and generate the bindings.
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .ctypes_prefix("::libc") // Use the ctypes from libc instead.
        .no_unstable_rust()
        .generate()
        .expect("Unable to generate bindings!");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
