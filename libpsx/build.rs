use std::env;
use std::fs;
use std::process::Command;

fn wrap_bios_function(fn_desc: &str) -> String {
    let name = &fn_desc[7..];
    let header = format!(".section .text.asm_{0}\n\
                          .global asm_{0}\n\
                          .type asm_{0}, function\n\
                          \n\
                          asm_{0}:\n", name);
    let fn_call = format!("j 0x{}0\n", &fn_desc[0..1]);
    let specify_fn = format!("li $t1, 0x{}\n", &fn_desc[2..4]);
    format!("{}{}{}\n", header, fn_call, specify_fn)
}

fn create_bios_src() -> String {
    let src_header = ".set mips1\n\
                      .set noreorder\n\
                      .set noat\n\
                      .set nomacro\n\
                      .text\n";
    let bios_functions = [
        "A(33h) malloc",
        "A(34h) free",
        "A(37h) calloc",
        "A(38h) realloc",
        "A(39h) init_heap",
        "A(3Fh) printf",
        "A(47h) gpu_send_dma",
        "A(48h) gpu_gp1_command_word",
        "A(49h) gpu_command_word",
        "A(4Ah) gpu_command_word_params",
        "A(4Dh) gpu_get_status",
    ];
    let src_body: String = bios_functions.iter()
                                         .map(|f| wrap_bios_function(f))
                                         .collect();
    format!("{}\n{}", src_header, src_body)
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mips_dir = "../mips_toolchain";
    let lib_name = "bios";
    let src_file = &format!("src/{}.s", lib_name);
    let obj_file = &format!("{}/{}.o", out_dir, lib_name);
    let ar_file = &format!("{}/lib{}.a", out_dir, lib_name);
    let as_bin = &format!("{}/as", mips_dir);
    let ar_bin = &format!("{}/ar", mips_dir);
    fs::write(src_file, create_bios_src()).expect("Unable to write to file");
    Command::new(as_bin)
        .args(&["-O2", "-msoft-float", "-mabi=32", "-o", obj_file, src_file])
        .status()
        .unwrap();
    Command::new(ar_bin)
        .args(&["rcs", ar_file, obj_file])
        .status()
        .unwrap();
    println!("cargo:rustc-link-lib=static={}", lib_name);
    println!("cargo:rustc-link-search={}", out_dir);
}
