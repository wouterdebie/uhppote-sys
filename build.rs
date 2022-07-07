use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    println!("cargo:rustc-link-search=native={}", &out_path.display());
    println!("cargo:rustc-link-lib=static=uhppoted");

    // Copy a cached version of the static lib to the output dir, rather
    // than building it for source if we're building for docs.rs, since
    // there is no network access in docs.rs builds.
    if std::env::var("DOCS_RS").is_ok() {
        fs::copy(
            "docs.rs-build/libuhppoted.a",
            &out_path.join("libuhppoted.a"),
        )
        .unwrap();
    } else {
        let status = Command::new("go")
            .args([
                "build",
                "-trimpath",
                "-buildmode=c-archive",
                "-o",
                &out_path.join("libuhppoted.a").display().to_string(),
                "go/devices.go",
                "go/cards.go",
                "go/events.go",
                "go/time_profiles.go",
                "go/tasks.go",
                "go/main.go",
            ])
            .current_dir("vendor/uhppoted-dll")
            .status()
            .expect("Make upstream uhppoted-dll failed");

        if !status.success() {
            panic!("Make upstream uhppoted-dll failed");
        }
    }
    let bindings = bindgen::Builder::default()
        .header("vendor/uhppoted-dll/lib/libuhppoted.h")
        .allowlist_function(
            "^Get.*|^Set.*|^Open.*|^Put.*|^Delete.*|^Record.*|^Clear.*|^Refresh.*|^Add.*",
        )
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!(
        "cargo:rerun-if-changed={}/vendor/uhppoted-dll/lib/libuhppoted.h",
        manifest_path.display()
    );
}
