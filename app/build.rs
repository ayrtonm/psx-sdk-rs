fn main() {
    println!("cargo:rustc-link-search=../libbios");
    println!("cargo:rustc-link-lib=static=bios");
}
