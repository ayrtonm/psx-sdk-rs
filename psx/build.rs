use std::path::PathBuf;
use std::{env, fs};

fn main() {
    const LINKER_SCRIPT: &'static str = "psexe.ld";
    const LOAD_OFFSET: &'static str = "PSX_LOAD_OFFSET";
    const STACK_POINTER: &'static str = "PSX_STACK_POINTER";
    println!("cargo:rerun-if-changed={}", LINKER_SCRIPT);
    println!("cargo:rerun-if-env-changed={}", LOAD_OFFSET);
    println!("cargo:rerun-if-env-changed={}", STACK_POINTER);

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    // Read the template ld script and add a load offset is requested
    let mut ld_script = include_str!("psexe.ld").to_string();
    if let Ok(offset) = env::var(LOAD_OFFSET) {
        ld_script = ld_script.replace("LOAD_OFFSET = 0", &format!("LOAD_OFFSET = {}", offset));
    }
    if let Ok(sp) = env::var(STACK_POINTER) {
        ld_script = ld_script.replace(
            "STACK_INIT = RAM_BASE + 0x001FFF00",
            &format!("STACK_INIT = {}", sp),
        );
    }

    // Put the ld script in the output directory
    fs::write(out.join(LINKER_SCRIPT), ld_script).unwrap();
}
