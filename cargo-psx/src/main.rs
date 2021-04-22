#![feature(bool_to_option)]

use cargo_metadata::{Metadata, MetadataCommand, Package};
use std::env;
use std::process::{self, Command, Stdio};

trait Args {
    fn extract_flag(&mut self, flag: &str) -> bool;
    fn extract_key_value(&mut self, key: &str) -> Option<String>;
}

impl Args for Vec<String> {
    fn extract_flag(&mut self, flag: &str) -> bool {
        self.iter()
            .position(|arg| arg == flag)
            .map(|n| {
                self.remove(n);
            })
            .is_some()
    }

    fn extract_key_value(&mut self, key: &str) -> Option<String> {
        self.iter().position(|arg| arg == key).map(|n| {
            let value = self[n + 1].clone();
            self.remove(n + 1);
            self.remove(n);
            value
        })
    }
}

fn apply_packages<F: Fn(&Package)>(metadata: &Metadata, f: F) {
    for pkg in &metadata.packages {
        f(&pkg);
    }
}

fn print_help() {
    println!("cargo-psx");
    println!("Runs `cargo build` to produce a PlayStation executable\n");
    println!("USAGE:");
    println!("  cargo psx [clean] [build|check|run|test] [OPTIONS] [cargo-build OPTIONS]\n");
    println!("OPTIONS:");
    println!("  --help, -h           Prints help information");
    println!("  --debug              Builds an ELF in release mode with debug info");
    println!("  --toolchain <NAME>   Sets the rustup toolchain to use (defaults to `psx`)");
    println!("  --region <REGION>    Sets the game region to NA (default), EU, J or none");
    println!("  --alloc              Builds the `alloc` crate");
    println!("  --lto                Enables link-time optimization and sets codegen units to 1");
    println!("  --small              Sets opt-level=s to optimize for size");
    println!("  --panic              Enables on-screen panic messages");
    println!("  --link <SCRIPT>      Specifies a custom linker script to use");
    println!("  --forbid-UB          Add runtime-checks for unreachable code in the psx crate");
    println!("");
    println!("Run `cargo build -h` for build options");
}

fn main() {
    // Skips `cargo psx`
    let mut args = env::args().skip(2).collect::<Vec<String>>();
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        // Printing help info short circuits everything
        return
    };

    // Extract cargo subcommand(s)
    let check = args.extract_flag("check");
    let clean = args.extract_flag("clean");
    let build = args.extract_flag("build");
    let run = args.extract_flag("run");
    let test = args.extract_flag("test");
    let skip_build = clean && !build && !check && !run && !test;

    let lto_flags = " -C lto=fat -C codegen-units=1 -C embed-bitcode=yes";
    let small_flag = " -C opt-level=s";
    let debug_flag = " -g";

    // Extract flags and key/value pairs
    let debug = args.extract_flag("--debug").then_some(debug_flag);
    let toolchain_name = args.extract_key_value("--toolchain");
    let region = args.extract_key_value("--region");
    let use_alloc = args.extract_flag("--alloc");
    let lto = args.extract_flag("--lto").then_some(lto_flags);
    let small = args.extract_flag("--small").then_some(small_flag);
    let pretty_panic = args
        .extract_flag("--panic")
        .then_some("pretty_panic".to_string());
    let no_ub = args
        .extract_flag("--no-UB")
        .then_some("forbid_UB".to_string());
    let linker_script = args.extract_key_value("--link");

    // The remaining args will be directly passed to cargo
    let mut cargo_args = args;

    // Set defaults for unspecified flags
    let region = match region {
        None => Some("NA_region".to_string()),
        Some(s) if s.to_ascii_lowercase() == "none" => None,
        Some(mut s) => {
            s.push_str("_region");
            Some(s)
        },
    };
    let toolchain_name = toolchain_name.unwrap_or("psx".to_string());
    let build_std = if use_alloc { "core,alloc" } else { "core" };
    // Pick user-specified linker script. Otherwise check if debug mode is enabled
    // to pick between PSEXE and ELF
    let linker_script = linker_script.unwrap_or(
        if debug.is_some() {
            "ELF.ld"
        } else {
            "psexe.ld"
        }
        .to_string(),
    );

    // Set psx-specific features
    let mut enable_feature = |opt_flag: Option<String>| {
        opt_flag.map(|flag| {
            cargo_args.push("--features".to_string());
            cargo_args.push(format!("psx/{}", flag));
        });
    };
    enable_feature(pretty_panic);
    enable_feature(no_ub);
    enable_feature(region);

    // Try getting RUSTFLAGS from env
    let mut rustflags = env::var("RUSTFLAGS").ok().unwrap_or("".to_string());

    // Set linker script in rustflags
    rustflags.push_str(&format!(" -C link-arg=-T{}", linker_script));
    // Set remaining optional rustflag args
    let mut enable_rustflag = |opt_flag: Option<&str>| {
        opt_flag.map(|flag| {
            rustflags.push_str(flag);
        });
    };
    enable_rustflag(lto);
    enable_rustflag(small);
    enable_rustflag(debug);

    let target_triple = "mipsel-sony-psx";

    let cargo_subcmd = if check {
        // `build` and `check` are mutually exclusive
        assert!(!build);
        "check"
    } else if run {
        "run"
    } else if test {
        "test"
    } else {
        "build"
    };

    // Always build in release mode
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
