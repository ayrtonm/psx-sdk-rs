use std::fs;

fn mk_bios_fn(fn_desc: &str) -> String {
    let fn_sig = &fn_desc[7..fn_desc.len() - 1];
    let li_stmt = &format!("li $9, 0x{}", &fn_desc[2..4]);
    let j_stmt = &format!("j 0x{}0", &fn_desc[0..1]);
    let returns = fn_sig.contains("->");
    let mut ret = String::new();
    ret.push_str("#[allow(unused_variables)]\n");
    ret.push_str("#[naked]\n");
    ret.push_str("#[inline(never)]\n");
    ret.push_str("pub extern \"C\" fn asm_"); ret.push_str(fn_sig); ret.push_str(" {\n");
    if returns {
        let ret_ty = fn_sig.split("->").skip(1).next().unwrap();
        ret.push_str("    let ret:");
        ret.push_str(ret_ty);
        ret.push_str(";\n");
    }
    ret.push_str("    unsafe {\n");
    ret.push_str("        asm!(\""); ret.push_str(li_stmt); ret.push_str("\n");
    ret.push_str("              ");  ret.push_str(j_stmt);  ret.push_str("\",\n");
    ret.push_str("               lateout(\"$2\") ");
    if returns {
        ret.push_str("ret);\n");
    } else {
        ret.push_str("_);\n");
    }
    ret.push_str("    }\n");
    if returns {
        ret.push_str("    ret\n");
    }
    ret.push_str("}\n");
    ret
}
fn main() {
    let bios_functions = [
        "A(33h) malloc(size: usize) -> *mut u8;",
        "A(34h) free(buf: *mut u8);",
        "A(37h) calloc(sizex: usize, sizey: usize) -> *const u8;",
        "A(38h) realloc(old_buf: *const u8, new_size: usize);",
        "A(39h) init_heap(addr: usize, size: usize);",
        "A(3Fh) printf(s: *const u8, v: u32);",
        "A(47h) gpu_send_dma(xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src: u32);",
        "A(48h) gpu_gp1_command_word(cmd: u32);",
        "A(49h) gpu_command_word(cmd: u32);",
        "A(4Ah) gpu_command_word_params(src: *const u32, num: usize);",
        "A(4Dh) gpu_get_status() -> u32;",
    ];
    let src_file = "src/bios_asm.rs";
    let src = bios_functions.iter().fold(String::new(), |s, f| s + &mk_bios_fn(f));
    fs::write(src_file, src).unwrap();
}
