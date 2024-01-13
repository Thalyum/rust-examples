// build.rs

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=my_c_lib");

    cc::Build::new()
        .file("my_c_lib/lib1.c")
        .file("my_c_lib/lib2.c")
        .compile("c_lib");
}
