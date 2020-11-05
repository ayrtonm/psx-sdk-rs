// TODO: Modify this to read form a crate's Cargo.toml instead of the working directory's name.
fn crate_name() -> Option<String> {
    std::env::current_dir().map(|dir| {
        dir.file_name().map(|name| {
            name.to_string_lossy().into_owned()
        })
    }).ok().flatten()
}

fn main() {
    let mut args = std::env::args();
    // Skip `cargo`
    args.next();
    // Skip `psx`
    args.next();

    let profile = &if let Some(profile) = args.next() {
        if profile == "-h" {
            println!("Usage: cargo psx [profile] [region]");
            println!("Profiles: release (default), debug");
            println!("Regions: NA (default), E or J");
            return;

        } else {
            profile
        }
    } else {
        "release".to_string()
    };

    let region = &if let Some(region) = args.next() {
        region
    } else {
        "NA".to_string()
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
