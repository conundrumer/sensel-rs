extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").expect("TARGET was not set");

    cc::Build::new()
        .include("vendor/sensel-api/sensel-lib/src")
        .file("vendor/sensel-api/sensel-lib/src/sensel.c")
        .file("vendor/sensel-api/sensel-lib/src/sensel_register.c")
        .file(if target.contains("windows") {
            "vendor/sensel-api/sensel-lib/src/sensel_serial_win.c"
        } else {
            "vendor/sensel-api/sensel-lib/src/sensel_serial_linux.c"
        })
        .flag("-w") // the code causes warnings so suppress them
        .compile("sensel");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
