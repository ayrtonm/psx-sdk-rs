use std::process::{Command, Stdio};
// TODO: Modify this to read form a crate's Cargo.toml instead of the working directory's name.
fn crate_name() -> Option<String> {
    std::env::current_dir().map(|dir| {
        dir.file_name().map(|name| {
            name.to_string_lossy().into_owned()
        })
    }).ok().flatten()
}

fn main() {
    // Skips `cargo psx`
    let mut args = std::env::args().skip(2);

    //let mut process = Command::new("cargo");
    //    process.arg("build")
    //    .arg("-Z")
    //    .arg("build-std=core,alloc")
    //    .arg("--target")
    //    .arg("mipsel-sony-psx")
    //    .arg("--verbose")
    //    .env("RUSTC", "psx_rustc")
    //    .env("RUSTFLAGS", "-C linker=../../mips_toolchain/ld")
    //    .args(args);
    //    process.stdin(Stdio::inherit())
    //    .stdout(Stdio::inherit())
    //    .stderr(Stdio::inherit())
    //    .spawn()
    //    .unwrap();

    //run cargo build -Z build-std=core,alloc
    let region = &if let Some(region) = args.next() {
        if region == "-h" {
            println!("Usage: cargo psx [region] [profile]");
            println!("Profiles: release (default), debug");
            println!("Regions: NA (default), E or J");
            return;

        } else {
            region
        }
    } else {
        "NA".to_string()
    };

    let profile = &if let Some(profile) = args.next() {
        profile
    } else {
        "release".to_string()
    };

    match crate_name() {
        Some(crate_name) => {
            let input = &format!("target/mipsel-sony-psx/{}/{}", profile, crate_name);
            let output = &format!("{}.psexe", crate_name);
            let args = vec!["", region, input, output];
            elf2psexe::main(args);
        },
        None => panic!("Could not read crate name"),
    };
}
