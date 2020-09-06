
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {

    // We want to re-run this build script if one of these files change.
    println!("cargo:rerun-if-changed=src/crt0.s");
    println!("cargo:rerun-if-changed=src/linker.ld");

    // Tell cargo where to find the products of this script.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    // We need to provide our linker script to the host application.
    fs::copy("src/linker.ld", out.join("link.x")).unwrap();

    // We need to also link in our assembly files.
    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar")
        .no_default_flags(true)
        .warnings_into_errors(true)
        .flag("-mcpu=arm7tdmi")
        .out_dir(out)
        .file("src/crt0.s")
        .compile("crt0");
}