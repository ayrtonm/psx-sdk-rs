#![feature(bool_to_option)]

use cargo_metadata::{Metadata, MetadataCommand, Package, Target};
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

fn apply_packages<F: Fn(&Package)>(metadata: &Metadata, f: F) {
    for pkg in &metadata.packages {
        f(&pkg);
    }
}

fn apply_targets<F: Fn(&Target)>(metadata: &Metadata, f: F) {
    for pkg in &metadata.packages {
        for target in &pkg.targets {
            if target.kind.iter().any(|k| k == "bin") {
                f(&target);
            }
        }
    }
}

fn psexe_name(name: &str, profile: &str, region: &str) -> String {
    format!("{}_{}_{}.psexe", name, profile, region)
}

fn print_help() {
    println!("cargo-psx");
    println!("Runs a cargo build then repackages the resulting ELF as a PSEXE\n");
    println!("USAGE:");
    println!("  cargo psx [clean|check] [OPTIONS] [cargo-build OPTIONS]\n");
    println!("OPTIONS:");
    println!("  --help, -h           Prints help information");
    println!("  --debug              Builds in release mode with debug info");
    println!("  --toolchain <NAME>   Sets the rustup toolchain to use (defaults to `psx`)");
    println!("  --region <REGION>    Sets the game region to NA (default), EU or JP");
    println!("  --skip-build         Skips build and only packages an existing ELF into a PSEXE");
    println!("  --skip-pack          Skips packaging and only builds an ELF");
    println!("  --no-pad             Skips padding the PSEXE file size to a multiple of 0x800");
    println!("  --no-alloc           Avoids building the `alloc` crate");
    println!("  --lto                Enables link-time optimization and sets codegen units to 1");
    println!("  --small              Sets opt-level=s to optimize for size");
    println!("  --no-hints           Opts out of aggressive code inlining using hints");
    println!("  --panic              Enables panic messages");
    println!("  --no-UB              Disables undefined behavior used to improve performance");
    //println!("  --test               Used internally for testing");
    println!("");
    println!("Run `cargo build -h` for build options");
}

fn main() {
    // Skips `cargo psx`
    let mut args = env::args().skip(2).collect::<Vec<String>>();
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        return
    };
    let cargo_args = &mut args;

    let check = extract_flag("check", cargo_args);
    let clean = extract_flag("clean", cargo_args);

    let debug = extract_flag("--debug", cargo_args);
    let toolchain_name = extract_key_value("--toolchain", cargo_args);
    let region = extract_key_value("--region", cargo_args);
    // This is enabled later on if we are only running `cargo clean`
    let mut skip_build = extract_flag("--skip-build", cargo_args);
    // This is enabled later on if we are only running `cargo check`
    let mut skip_pack = extract_flag("--skip-pack", cargo_args);
    let no_pad = extract_flag("--no-pad", cargo_args);
    let no_alloc = extract_flag("--no-alloc", cargo_args);
    let lto = extract_flag("--lto", cargo_args);
    let small = extract_flag("--small", cargo_args);
    let pretty_panic = extract_flag("--panic", cargo_args);
    let no_hints = extract_flag("--no-hints", cargo_args);
    let no_ub = extract_flag("--no-UB", cargo_args);
    let testing = extract_flag("--test", cargo_args);

    let region = region.unwrap_or("NA".to_string());
    let toolchain_name = toolchain_name.unwrap_or("psx".to_string());
    let build_std = if no_alloc { "core" } else { "core,alloc" };
    let mut enable_feature = |flag, name| {
        if flag {
            cargo_args.push("--features".to_string());
            cargo_args.push(format!("psx/{}", name));
        };
    };
    enable_feature(pretty_panic, "pretty_panic");
    enable_feature(no_hints, "no_inline_hints");
    enable_feature(no_ub, "forbid_UB");
    enable_feature(testing, "no_std_test");

    let lto_flags = " -C lto=fat -C codegen-units=1 -C embed-bitcode=yes";
    let small_flag = " -C opt-level=s";
    let mut rustflags = env::var("RUSTFLAGS").ok().unwrap_or("".to_string());
    if lto {
        rustflags.push_str(lto_flags);
    }
    if small {
        rustflags.push_str(small_flag);
    };
    if debug {
        rustflags.push_str(" -g");
    }

    let target_triple = "mipsel-sony-psx";

    let cargo_subcmd = if check {
        skip_pack = true;
        "check"
    } else {
        "build"
    };

    cargo_args.push("--release".to_string());

    // Gets cargo metadata for `clean` and `pack` steps
    let metadata = &MetadataCommand::new()
        .exec()
        .expect("Could not parse cargo metadata");
    let profile = "release";

    if clean {
        skip_build = true;
        skip_pack = true;
        apply_packages(metadata, |pkg| {
            let mut clean = Command::new("cargo")
                .arg("clean")
                .arg("-p")
                .arg(&pkg.name)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("`cargo clean` failed to start");
            let status = clean.wait().expect("`cargo clean` wasn't running");
            if !status.success() {
                let code = status.code().unwrap_or(1);
                process::exit(code);
            }
        });
        apply_targets(metadata, |target| {
            let psexe_name = &psexe_name(&target.name, profile, &region);
            let mut rm = Command::new("rm")
                .arg(psexe_name)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect(&format!("`rm {}` failed to start", psexe_name));
            let status = rm
                .wait()
                .expect(&format!("`rm {}` wasn't running", psexe_name));
            if !status.success() {
                let code = status.code().unwrap_or(1);
                process::exit(code);
            }
        });
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
            .expect(&format!("`cargo {}` failed to start", cargo_subcmd));

        let status = build.wait().expect("`cargo build` wasn't running");
        if !status.success() {
            let code = status.code().unwrap_or(1);
            process::exit(code);
        }
    }

    if !skip_pack {
        let target_dir = metadata.target_directory.join(target_triple).join(profile);
        apply_targets(metadata, |target| {
            let elf = &target_dir
                .join(&target.name)
                .to_str()
                .expect("Could not convert ELF path to UTF-8")
                .to_string();
            let psexe = &psexe_name(&target.name, profile, &region);
            let convert_args = vec![region.as_str(), elf, psexe];
            elf2psexe::main(convert_args, no_pad);
        });
    }
}
