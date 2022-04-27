use std::path::PathBuf;
use std::{env, fs};

fn main() {
    const LINKER_SCRIPT: &'static str = "psexe.ld";
    println!("cargo:rerun-if-changed={}", LINKER_SCRIPT);
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());
    fs::write(
        out.join(LINKER_SCRIPT),
        include_str!("psexe.ld").to_string(),
    )
    .unwrap();
}
