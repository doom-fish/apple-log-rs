fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/c-shim/apple_log_shim.c");
    cc::Build::new()
        .file("src/c-shim/apple_log_shim.c")
        .flag("-fmodules")
        .compile("apple_log_shim");
}
