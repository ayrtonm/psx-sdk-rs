#![feature(bool_to_option)]

use cargo_metadata::MetadataCommand;
use std::env;
use std::process::{self, Command, Stdio};

fn extract_flag(flag: &str, args: Vec<String>) -> (bool, Vec<String>) {
    let flag_present = args.iter().cloned().filter(|arg| arg == flag).count() == 1;
    let args = if flag_present {
        args.split(|arg| arg == flag)
            .flatten()
            .cloned()
            .collect::<Vec<String>>()
    } else {
        args
    };
    (flag_present, args)
}
fn extract_key_value(key: &str, args: Vec<String>) -> (Option<String>, Vec<String>) {
    // Splits arguments at key argument
    let mut temp_iter = args.split(|arg| arg == key);
    // Gets all arguments before key
    let cargo_args = temp_iter
        .next()
        .expect("Args empty")
        .iter()
        .cloned()
        .collect::<Vec<String>>();
    // Gets arguments after key if any and pops the first argument after key, i.e.
    // the desired key's value. Then combines the remaining arguments with the
    // ones before key.
    match temp_iter.next() {
        Some(v) => {
            let mut it = v.iter();
            (
                Some(
                    it.next()
                        .expect("`split` returned an empty iterator")
                        .to_string(),
                ),
                cargo_args
                    .iter()
                    .chain(it)
                    .cloned()
                    .collect::<Vec<String>>(),
            )
        },
        None => (None, cargo_args),
    }
}

fn main() {
    // Skips `cargo psx`
    let args = env::args().skip(2).collect::<Vec<String>>();
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!("cargo-psx");
        println!("Builds with cargo then repackages the ELF as a PSEXE\n");
        println!("USAGE:");
        println!("  cargo psx [OPTIONS]\n");
        println!("OPTIONS:");
        println!("  --help, -h           Prints help information");
        println!("  --toolchain <NAME>   Sets the name of the rustup toolchain to use (defaults to `psx`)");
        println!("  --region <REGION>    Sets the game region to NA, EU or JP (default)");
        println!(
            "  --skip-build         Skips building and only packages an existing ELF into a PSEXE"
        );
        println!("  --skip-pack          Skips packaging and only builds an ELF");
        println!("  --no-pad             Skips padding the PSEXE file size to a multiple of 0x800");
        println!("  --no-alloc           Avoids building the `alloc` crate");
        println!("");
        println!("Run `cargo build -h` for build options");
        return
    };
    let (region, cargo_args) = extract_key_value("--region", args);
    let (toolchain_name, cargo_args) = extract_key_value("--toolchain", cargo_args);
    let (skip_build, cargo_args) = extract_flag("--skip-build", cargo_args);
    let (skip_pack, cargo_args) = extract_flag("--skip-pack", cargo_args);
    let (no_pad, cargo_args) = extract_flag("--no-pad", cargo_args);
    let (no_alloc, cargo_args) = extract_flag("--no-alloc", cargo_args);

    let region = region.unwrap_or("JP".to_string());
    let toolchain_name = toolchain_name.unwrap_or("psx".to_string());
    let build_std = if no_alloc { "core" } else { "core,alloc" };

    let target_triple = "mipsel-sony-psx";
    if !skip_build {
        // TODO: remove `RUSTFLAGS` env var after fixing rust-lld's alloc error
        let mut build = Command::new("cargo")
            .arg("+".to_string() + &toolchain_name)
            .arg("build")
            .arg("-Z")
            .arg("build-std=".to_string() + &build_std)
            .arg("--target")
            .arg(target_triple)
            .args(cargo_args)
            .env("RUSTFLAGS", "-C linker=../../mips_toolchain/ld")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("`cargo build` failed to start");

        let status = build.wait().expect("`cargo build` wasn't running");
        if !status.success() {
            let code = status.code().unwrap_or(1);
            process::exit(code);
        }
    }

    if !skip_pack {
        let metadata = MetadataCommand::new()
            .exec()
            .expect("Could not parse cargo metadata");
        let profile = env::args()
            .any(|arg| arg == "--release")
            .then_some("release")
            .unwrap_or("debug");

        let target_dir = metadata.target_directory.join(target_triple).join(profile);
        for pkg in metadata.packages {
            for target in pkg.targets {
                if target.kind.iter().any(|k| k == "bin") {
                    let elf = &target_dir
                        .join(&target.name)
                        .to_str()
                        .expect("Could not convert ELF path to UTF-8")
                        .to_string();
                    let psexe = &format!("{}{}", &target.name, ".psexe");
                    let convert_args = vec![region.as_str(), elf, psexe];
                    elf2psexe::main(convert_args, no_pad);
                }
            }
        }
    }
}
