// build.rs

use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=my_c_lib");

    // create common header composed of all required headers
    let headers: Vec<_> = fs::read_dir("my_c_lib")
        .unwrap()
        .into_iter()
        .filter(|path| {
            path.as_ref()
                .unwrap()
                .path()
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("?")
                == "h"
        })
        .map(|path| path.unwrap().path().clone())
        .collect();
    let mut new_hdr = String::new();
    for header in headers {
        new_hdr.push_str(&format!("#include \"{}\"\n", header.to_string_lossy()));
    }
    let _ = fs::write("c_lib.h", new_hdr);

    // create bindings
    let bindings = bindgen::Builder::default()
        .header("c_lib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // compile my_c_lib
    cc::Build::new()
        .file("my_c_lib/lib1.c")
        .file("my_c_lib/lib2.c")
        .compile("c_lib");
}
