fn main() {
    println!("cargo:rerun-if-changed=bios.ld");
    // This is really only required if linking with mipsel-unknown-elf-ld
    println!("cargo:rustc-link-arg=--oformat=binary");
}
