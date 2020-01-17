use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let package_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let lsl_dir = package_dir.join("liblsl-1.13.0-b14");
    let lsl_build_dir = out_dir.join("lsl_build");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    println!("cargo:rustc-link-search={}", lsl_build_dir.display());
    println!("cargo:rustc-link-lib=static=lsl-static");
    println!("cargo:rustc-link-lib=stdc++");

    if !lsl_build_dir.exists() {
        std::fs::create_dir(&lsl_build_dir)?;
    }
    Command::new("cmake")
        .arg(&lsl_dir)
        .arg("-DLSL_BUILD_STATIC")
        .arg("-DBOOST_ALL_NO_LIB")
        .current_dir(&lsl_build_dir)
        .spawn()?
        .wait();

    Command::new("make")
        .current_dir(&lsl_build_dir)
        .arg(format!("-j{}", num_cpus::get() - 1))
        .spawn()?
        .wait();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let lsl_binding = Path::new("bindings.rs");
    bindings
        .write_to_file(out_dir.join(lsl_binding))
        .expect("Couldn't write bindings!");

    Ok(())
}
