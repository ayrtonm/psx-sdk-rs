fn main() {
    println!("cargo:rustc-link-search=../libbios");
    println!("cargo:rustc-link-search=../libpsx/target/mipsel-sony-psx/release");
    println!("cargo:rustc-link-search=../libpsx/target/mipsel-sony-psx/release/deps");
}
