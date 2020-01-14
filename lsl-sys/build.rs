use std::path::Path;

static LIBLSL_NAME: &str = "lsl";

fn main() {
    let out_dir: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().into();

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        //.clang_arg(format!("-DFUSE_USE_VERSION={}", 26))
        .generate()
        .expect("Unable to generate bindings");

    let lsl_binding = Path::new("bindings.rs");
    bindings
        .write_to_file(out_dir.join(lsl_binding))
        .expect("Couldn't write bindings!");
}
