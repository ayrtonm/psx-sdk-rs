#![feature(bool_to_option)]

use cargo_metadata::MetadataCommand;
use std::env;
use std::process::{self, Command, Stdio};

fn extract_flag(flag: &str, args: &mut Vec<String>) -> bool {
    args.iter()
        .position(|arg| arg == flag)
        .map(|n| {
            args.remove(n);
        })
        .is_some()
}

fn extract_key_value(key: &str, args: &mut Vec<String>) -> Option<String> {
    args.iter().position(|arg| arg == key).map(|n| {
        let value = args[n + 1].clone();
        args.remove(n + 1);
        args.remove(n);
        value
    })
}

fn main() {
    // Skips `cargo psx`
    let mut args = env::args().skip(2).collect::<Vec<String>>();
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!("cargo-psx");
        println!("Builds with cargo in release mode then repackages the ELF as a PSEXE\n");
        println!("USAGE:");
        println!("  cargo psx [check] [OPTIONS]\n");
        println!("OPTIONS:");
        println!("  --help, -h           Prints help information");
        println!("  --debug              Builds in debug mode");
        println!(
            "  --toolchain <NAME>   Sets the name of the rustup toolchain to use (defaults to `psx`)"
        );
        println!("  --region <REGION>    Sets the game region to NA, EU or JP (default)");
        println!(
            "  --skip-build         Skips building and only packages an existing ELF into a PSEXE"
        );
        println!("  --skip-pack          Skips packaging and only builds an ELF");
        println!("  --no-pad             Skips padding the PSEXE file size to a multiple of 0x800");
        println!("  --no-alloc           Avoids building the `alloc` crate");
        println!(
            "  --lto                Enables link-time optimization and set codegen units to 1"
        );
        println!("  --size               Sets opt-level=s to optimize for size");
        println!("  --panic              Enables panic messages (may add ~10 KB)");
        println!("");
        println!("Run `cargo build -h` for build options");
        return
    };
    let cargo_args = &mut args;

    let debug = extract_flag("--debug", cargo_args);
    let toolchain_name = extract_key_value("--toolchain", cargo_args);
    let region = extract_key_value("--region", cargo_args);
    let skip_build = extract_flag("--skip-build", cargo_args);
    // This is enabled later on if we are only running `cargo check`
    let mut skip_pack = extract_flag("--skip-pack", cargo_args);
    let no_pad = extract_flag("--no-pad", cargo_args);
    let no_alloc = extract_flag("--no-alloc", cargo_args);
    let lto = extract_flag("--lto", cargo_args);
    let size = extract_flag("--size", cargo_args);
    let check = extract_flag("check", cargo_args);
    let pretty_panic = extract_flag("--panic", cargo_args);

    let region = region.unwrap_or("JP".to_string());
    let toolchain_name = toolchain_name.unwrap_or("psx".to_string());
    let build_std = if no_alloc { "core" } else { "core,alloc" };
    if pretty_panic {
        cargo_args.push("--features".to_string());
        cargo_args.push("psx/pretty_panic".to_string());
    };
    let lto_flags = "-C lto=fat -C codegen-units=1 -C embed-bitcode=yes".to_string();
    let size_flag = "-C opt-level=s".to_string();
    let rustflags = &match (env::var("RUSTFLAGS").ok(), lto, size) {
        (None, false, false) => "".to_string(),
        (None, true, false) => lto_flags,
        (None, false, true) => size_flag,
        (None, true, true) => format!("{} {}", lto_flags, size_flag),

        (Some(flags), false, false) => flags,
        (Some(flags), true, false) => format!("{} {}", flags, lto_flags),
        (Some(flags), false, true) => format!("{} {}", flags, size_flag),
        (Some(flags), true, true) => format!("{} {} {}", flags, lto_flags, size_flag),
    };

    let target_triple = "mipsel-sony-psx";

    let cargo_subcmd = if check {
        skip_pack = true;
        "check"
    } else {
        "build"
    };

    if !debug {
        cargo_args.push("--release".to_string());
    }

    if !skip_build {
        let mut build = Command::new("cargo")
            .arg("+".to_string() + &toolchain_name)
            .arg(cargo_subcmd)
            .arg("-Z")
            .arg("build-std=".to_string() + &build_std)
            .arg("--target")
            .arg(target_triple)
            .args(cargo_args)
            .env("RUSTFLAGS", rustflags)
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
            .any(|arg| arg == "--debug")
            .then_some("debug")
            .unwrap_or("release");

        let target_dir = metadata.target_directory.join(target_triple).join(profile);
        for pkg in metadata.packages {
            for target in pkg.targets {
                if target.kind.iter().any(|k| k == "bin") {
                    let elf = &target_dir
                        .join(&target.name)
                        .to_str()
                        .expect("Could not convert ELF path to UTF-8")
                        .to_string();
                    let psexe = &format!("{}_{}_{}{}", &target.name, profile, region, ".psexe");
                    let convert_args = vec![region.as_str(), elf, psexe];
                    elf2psexe::main(convert_args, no_pad);
                }
            }
        }
    }
}
