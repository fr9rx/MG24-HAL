use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Put `link.x` somewhere the linker will look. Without this an application
    // depending on mg24-hal would have to vendor the script itself.
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is always set"));
    fs::write(out.join("link.x"), include_bytes!("link.x")).expect("failed to write link.x");

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rerun-if-changed=build.rs");
}
