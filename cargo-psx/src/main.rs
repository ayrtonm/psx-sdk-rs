#![feature(bool_to_option)]

use cargo_metadata::{Metadata, MetadataCommand, Package};
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

fn print_help() {
    println!("cargo-psx");
    println!("Runs cargo build to produce a PSEXE\n");
    println!("USAGE:");
    println!("  cargo psx [clean] [build|check|run|test] [OPTIONS] [cargo-build OPTIONS]\n");
    println!("OPTIONS:");
    println!("  --help, -h           Prints help information");
    println!("  --debug              Builds in release mode with debug info");
    println!("  --toolchain <NAME>   Sets the rustup toolchain to use (defaults to `psx`)");
    //println!("  --region <REGION>    Sets the game region to NA (default), EU or JP");
    println!("  --use-alloc          Builds the `alloc` crate");
    println!("  --lto                Enables link-time optimization and sets codegen units to 1");
    println!("  --small              Sets opt-level=s to optimize for size");
    println!("  --panic              Enables panic messages");
    println!("  --no-UB              Disables undefined behavior used to improve performance");
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
    let build = extract_flag("build", cargo_args);
    let run = extract_flag("run", cargo_args);
    let test = extract_flag("test", cargo_args);
    let skip_build = clean && !build && !check && !run && !test;

    let debug = extract_flag("--debug", cargo_args);
    let toolchain_name = extract_key_value("--toolchain", cargo_args);
    // TODO: figure out a way to make this work with the linker script
    let region = extract_key_value("--region", cargo_args);
    // This is enabled later on if we are only running `cargo clean`
    let use_alloc = extract_flag("--use-alloc", cargo_args);
    let lto = extract_flag("--lto", cargo_args);
    let small = extract_flag("--small", cargo_args);
    let pretty_panic = extract_flag("--panic", cargo_args);
    let no_ub = extract_flag("--no-UB", cargo_args);

    let region = region.unwrap_or("NA".to_string());
    let toolchain_name = toolchain_name.unwrap_or("psx".to_string());
    let build_std = if use_alloc { "core,alloc" } else { "core" };
    let mut enable_feature = |flag, name| {
        if flag {
            cargo_args.push("--features".to_string());
            cargo_args.push(format!("psx/{}", name));
        };
    };
    // TODO: review feature flags in use
    enable_feature(pretty_panic, "pretty_panic");
    enable_feature(no_ub, "forbid_UB");

    let lto_flags = " -C lto=fat -C codegen-units=1 -C embed-bitcode=yes";
    let small_flag = " -C opt-level=s";
    let mut rustflags = env::var("RUSTFLAGS").ok().unwrap_or("".to_string());
    // TODO: make this user configurable
    if !debug {
        rustflags.push_str(" -C link-arg=-Tpsexe.ld");
    }
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
        assert!(!build);
        "check"
    } else if run {
        "run"
    } else if test {
        "test"
    } else {
        "build"
    };

    cargo_args.push("--release".to_string());

    // Gets cargo metadata for `clean` and `pack` steps
    let metadata = &MetadataCommand::new()
        .exec()
        .expect("Could not parse cargo metadata");

    if clean {
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
}
