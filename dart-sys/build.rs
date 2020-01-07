use std::env;
use std::path::PathBuf;

use bindgen::EnumVariation;

//We create a build.rs file in our crate's root. Cargo will pick up on the existence of this file, then compile and execute it before the rest of the crate is built. This can be used to generate code at compile time. And of course in our case, we will be generating Rust FFI bindings to bzip2 at compile time. The resulting bindings will be written to $OUT_DIR/bindings.rs where $OUT_DIR is chosen by cargo and is something like

fn main() {
    let mut bindings_builder = bindgen::Builder::default();

    println!("cargo:rerun-if-env-changed=BINDGEN_DART_SDK_PATH");

    println!("Ensure that BINDGEN_DART_SDK_PATH is set to get dart headers!");

    let dartsdk_path = if let Ok(path) = env::var("BINDGEN_DART_SDK_PATH") {
        PathBuf::from(path)
    } else {
        panic!("BINDGEN_DART_SDK_PATH not found in env");
    };

    bindings_builder = bindings_builder
        .header(format!("{}/include/dart_api.h", dartsdk_path.display()))
        .header(format!(
            "{}/include/dart_native_api.h",
            dartsdk_path.display()
        ))
        .header(format!(
            "{}/include/dart_tools_api.h",
            dartsdk_path.display()
        ));

    let bindings = bindings_builder
        .generate_inline_functions(true)
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .use_core()
        .clang_arg("-std=c++14")
        // required for macOS LLVM 8 to pick up C++ headers:
        .clang_args(&["-x", "c++"])
        .generate()
        .expect("Unable to generate bindings");

    // Where the bindings are generated!
    bindings
        .write_to_file("./src/bindings.rs")
        .expect("Couldn't write bindings!");
}
