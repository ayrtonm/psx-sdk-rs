fn main() {
    println!("cargo:rustc-link-search=../libbios");
    println!("cargo:rustc-link-search=../libpsx/target/target/release");
    println!("cargo:rustc-link-search=../libpsx/target/target/release/deps");
}
