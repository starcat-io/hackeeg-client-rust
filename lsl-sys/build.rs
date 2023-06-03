// Copyright © 2020 Starcat LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use flate2::read::GzDecoder;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;
use std::io::{self, Write};
use tar::Archive;

fn build_lsl_unix(lsl_dir: PathBuf, lsl_build_dir: PathBuf) {
    println!("cargo:rustc-link-lib=stdc++");
    Command::new("cmake")
        .arg(&lsl_dir)
        .arg("-DLSL_BUILD_STATIC=1")
        .arg("-DBOOST_ALL_NO_LIB=1")
        .current_dir(&lsl_build_dir)
        .spawn()
        .expect("Can't spawn subprocess.")
        .wait();

    Command::new("make")
        .current_dir(&lsl_build_dir)
        //.arg(format!("-j{}", num_cpus::get() - 1))
        .spawn()
        .expect("Can't spawn subprocess.")
        .wait();
}

fn build_lsl_windows(lsl_dir: PathBuf, lsl_lib_dir: PathBuf) {
    println!("cargo:rustc-link-lib=static:-bundle=winmm");
    println!("cargo:rustc-link-lib=static:-bundle=iphlpapi");
    println!("cargo:rustc-link-search={}", lsl_lib_dir.display());
    Command::new("cmake")
        .arg(&lsl_dir)
        .arg("-B build")
        .arg("-DLSL_BUILD_STATIC=1")
        .arg("-G Visual Studio 17 2022")
        .arg("-A x64")
        .current_dir(&lsl_dir)
        .spawn()
        .expect("Can't spawn subprocess.")
        .wait()
        .expect(&("Cannot execute command, path: ".to_owned() + &env::var("PATH").unwrap()));

    Command::new("cmake")
        .arg("--build")
        .arg("build")
        .arg("--config Release")
        .current_dir(&lsl_dir)
        .spawn()
        .expect("Can't spawn subprocess.")
        .wait()
        .expect(&("Cannot execute command, path: ".to_owned() + &env::var("PATH").unwrap()));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let package_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();

    let lsl_dir = out_dir.join("liblsl-1.16.2");
    let lsl_build_dir = lsl_dir.join("build");
    let lsl_include_dir = lsl_dir.join("include");
    let lsl_lib_dir = lsl_build_dir.join("Release");

    if !lsl_dir.exists() {
        let tar_gz = File::open(package_dir.join("liblsl-1.16.2.tar.gz"))?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(&out_dir)?;
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    println!("cargo:rustc-link-search={}", lsl_build_dir.display());
    println!("cargo:rustc-link-lib=static=lsl");

    if !lsl_build_dir.exists() {
        std::fs::create_dir(&lsl_build_dir)?;
    }

    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        build_lsl_unix(lsl_dir, lsl_build_dir);
    } else if cfg!(target_os = "windows") {
        build_lsl_windows(lsl_dir, lsl_lib_dir);
    } else {
        println!("cargo:warning=Unsupported operating system.") 
    }
     
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", lsl_include_dir.display()))
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let lsl_binding = Path::new("lsl_bindings.rs");
    bindings
        .write_to_file(out_dir.join(lsl_binding))
        .expect("Couldn't write bindings!");

    Ok(())
}
