use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.starts_with("aarch64-unknown-none-softfloat") {
        println!("cargo:rustc-link-arg=-Tloader/src/link.ld");
        println!("cargo:rerun-if-changed=loader/src/link.ld");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
