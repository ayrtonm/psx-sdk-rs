#![feature(bool_to_option)]

use std::env;
use std::process::{self, Command, Stdio};
use cargo_metadata::MetadataCommand;

fn main() {
    // Skips `cargo psx`
    let args = env::args().skip(2).collect::<Vec<String>>();

    // Splits arguments at `--region` argument
    let mut temp_iter = args.split(|arg| arg == "--region");
    // Gets all arguments before `--region`
    let cargo_args = temp_iter.next().unwrap().iter().cloned().collect::<Vec<String>>();
    // Gets arguments after `--region` if any and pops the first argument after `--region`, i.e. the
    // desired region. Then combines the remaining arguments with the ones before `--region`.
    let (region, cargo_args) = match temp_iter.next() {
        Some(v) => {
            let mut it = v.iter();
            (it.next().unwrap().as_str(),
            cargo_args.iter().chain(it).cloned().collect::<Vec<String>>())
        },
        None => ("NA", cargo_args),
    };

    let target_triple = "mipsel-sony-psx";
    let mut build = Command::new("cargo")
        .arg("build")
        .arg("-Z")
        .arg("build-std=core,alloc")
        .arg("--target")
        .arg(target_triple)
        // Change rustc and the linker to sensible defaults and make configuration easier
        .env("RUSTC", "psx_rustc")
        .env("RUSTFLAGS", "-C linker=../../mips_toolchain/ld")
        .args(cargo_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = build.wait().unwrap();
    if !status.success() {
        let code = status.code().unwrap_or(1);
        process::exit(code);
    }

    let metadata = MetadataCommand::new()
        .exec()
        .unwrap();
    let profile = env::args().any(|arg| arg == "--release").then_some("release").unwrap_or("debug");

    let target_dir = metadata.target_directory.join(target_triple).join(profile);
    for pkg in metadata.packages {
        for target in pkg.targets {
            if target.kind.iter().any(|k| k == "bin") {
                let elf = &target_dir.join(&target.name).to_str().unwrap().to_string();
                let psexe = &format!("{}{}", &target.name, ".psexe");
                let convert_args = vec![region, elf, psexe];
                println!("{:#?}", convert_args);
                elf2psexe::main(convert_args);
            }
        }
    }
}
