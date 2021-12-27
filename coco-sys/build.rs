use std::{collections::HashSet, env, path::PathBuf};

fn main() {
    generate_bindings();
    compile_and_link_coco();
}

fn generate_bindings() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // These would be generated multiple times
    let ignored_macros = IgnoreMacros(
        [
            "FP_INFINITE",
            "FP_NAN",
            "FP_NORMAL",
            "FP_SUBNORMAL",
            "FP_ZERO",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(ignored_macros))
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

fn compile_and_link_coco() {
    cc::Build::new()
        .file("vendor/coco-prebuilt/coco.c")
        .file("wrapper.c")
        .warnings(false)
        .compile("coco");
}

// ADDITIONAL HELPERS

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}
