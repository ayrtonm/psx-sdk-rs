use std::process::Command;

fn main() {
    Command::new("../mips_toolchain/as")
            .args(&["-O2", "-o", "libbios.o", "src/libbios.s"])
            .status()
            .unwrap();
    Command::new("../mips_toolchain/ar")
            .args(&["rcs", "libbios.a", "libbios.o"])
            .status()
            .unwrap();
    println!("cargo:rustc-link-search=./");
    println!("cargo:rustc-link-lib=static=bios");
}
