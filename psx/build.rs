use std::path::PathBuf;
use std::{env, fs};

/// A BIOS function description
///
/// This is derived from a &str of the form `A(04h) file_close(fd: u8) -> i8;`
struct FnDesc<'a> {
    /// The function signature (e.g. file_close(fd: u8) -> i8;)
    sig: &'a str,
    /// The function name (e.g. file_close)
    name: &'a str,
    /// The function "type" (e.g. A, B, C or SYS)
    ty: &'a str,
    /// The MIPS register used for the function number: R4 or R9
    arg: u32,
    /// The function number (e.g. 0x04 for file_close)
    num: &'a str,
    is_syscall: bool,
}

fn parse_fn_desc(fn_desc: &str) -> FnDesc {
    let mut type_end = 1;
    let mut num_start = 2;
    let mut num_end = 4;
    let mut sig_start = 7;
    let is_syscall = fn_desc.starts_with('S');
    let arg = if is_syscall {
        type_end += 2;
        num_start += 2;
        num_end += 2;
        sig_start += 2;
        4
    } else {
        9
    };
    let sig = &fn_desc[sig_start..];
    FnDesc {
        sig,
        name: sig
            .split('(')
            .next()
            .expect("Unable to parse function description"),
        ty: &fn_desc[0..type_end],
        arg,
        num: &fn_desc[num_start..num_end],
        is_syscall,
    }
}

const INDENT: &str = "    ";

fn decl_bios_fn(func: &FnDesc) -> String {
    format!("{}/// Calls BIOS function [{}({}h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)\n\
             {0}pub fn {3}\n", INDENT, func.ty, func.num, func.sig)
}

fn mk_bios_trampoline(func: &FnDesc) -> String {
    let li_stmt = &format!("li ${}, 0x{}", func.arg, func.num);
    let j_stmt = &if func.is_syscall {
        format!(
            "syscall 0x0\n\
                 {}jr $ra\n\
                 {0}nop",
            INDENT
        )
    } else {
        format!("j 0x{}0", func.ty)
    };
    let stmts = if func.is_syscall {
        [li_stmt, j_stmt]
    } else {
        [j_stmt, li_stmt]
    };
    format!(
        "\n\
             .section .text.bios.{}\n\
             .globl {0}\n\
             {0}:\n\
                 {}{}\n\
                 {1}{3}\n",
        func.name, INDENT, stmts[0], stmts[1]
    )
}

fn main() {
    const SYS_MODULE_SRC_DIR: &str = "src/sys";
    let bios_functions: Vec<FnDesc> = include_str!("bios.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("//"))
        .map(parse_fn_desc)
        .collect();

    // Generate the bios function trampolines
    let asm_file = &format!("{}/trampoline.s", SYS_MODULE_SRC_DIR);
    let asm = bios_functions
        .iter()
        .fold(String::new(), |s, f| s + &mk_bios_trampoline(f));
    fs::write(
        asm_file,
        format!(
            "// This file was automatically generated by build.rs\n\
                 .set noreorder\n{}",
            asm
        ),
    )
    .unwrap_or_else(|_| panic!("Unable to write to {}", asm_file));

    // Generate the bios function declarations
    let src_file = &format!("{}/kernel.rs", SYS_MODULE_SRC_DIR);
    let src = bios_functions
        .iter()
        .fold(String::new(), |s, f| s + &decl_bios_fn(f));
    fs::write(
        src_file,
        format!(
            "//! BIOS kernel functions\n\
                 // This file was automatically generated by build.rs\n\n\
                 core::arch::global_asm!(include_str!(\"trampoline.s\"));\n\n\
                 extern \"C\" {{\n\
                 {}\
                 }}\n",
            src
        ),
    )
    .unwrap_or_else(|_| panic!("Unable to write to {}", src_file));
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("psexe.ld"), include_str!("psexe.ld").to_string()).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
