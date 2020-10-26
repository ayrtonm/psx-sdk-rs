use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mips_dir = "../mips_toolchain";
    println!("cargo:rustc-link-search={}", out_dir);

    let f = "bios";
    let src_file = &format!("src/{}.s", f);
    let obj_file = &format!("{}/{}.o", out_dir, f);
    let ar_file = &format!("{}/lib{}.a", out_dir, f);
    let as_bin = &format!("{}/as", mips_dir);
    let ar_bin = &format!("{}/ar", mips_dir);
    println!("cargo:rerun-if-changed={}", src_file);
    Command::new(as_bin)
            .args(&["-O2", "-o", obj_file, src_file])
            .status()
            .unwrap();
    Command::new(ar_bin)
            .args(&["rcs", ar_file, obj_file])
            .status()
            .unwrap();
    println!("cargo:rustc-link-lib=static={}", f);
}
