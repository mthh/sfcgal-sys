extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::path::Path;


fn main() {
    println!("cargo:rustc-link-lib=SFCGAL");

    cc::Build::new()
        .file("src/wrapper.c")
        .include("/usr/include")
        .out_dir(&Path::new("target/"))
        .compile("sfcgalwrapper.a");

    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::Stable_1_33)
        .header("src/wrapper.h")
        .whitelist_type("sfcgal_.*$")
        .whitelist_var("sfcgal_.*$")
        .whitelist_function("sfcgal_.*$")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
