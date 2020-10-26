use std::process::Command;

fn main() {
    let asm_files = ["bios", "load_delay_slot"];
    for f in &asm_files {
        let src_file = &format!("src/{}.s", f);
        let obj_file = &format!("{}.o", f);
        let ar_file = &format!("lib{}.a", f);
        Command::new("../mips_toolchain/as")
                .args(&["-O2", "-o", obj_file, src_file])
                .status()
                .unwrap();
        Command::new("../mips_toolchain/ar")
                .args(&["rcs", ar_file, obj_file])
                .status()
                .unwrap();
        println!("cargo:rustc-link-search=./");
        println!("cargo:rustc-link-lib=static={}", f);
    }
}
