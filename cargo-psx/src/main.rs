use cargo_metadata::MetadataCommand;
use std::env;
use std::process::{self, Command, Stdio};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
enum CargoCommand {
    Build,
    Check,
    Run,
    Test,
}

impl From<CargoCommand> for &'static str {
    fn from(cmd: CargoCommand) -> &'static str {
        match cmd {
            CargoCommand::Build => "build",
            CargoCommand::Check => "check",
            CargoCommand::Run => "run",
            CargoCommand::Test => "test",
        }
    }
}

impl FromStr for CargoCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "build" => Ok(CargoCommand::Build),
            "check" => Ok(CargoCommand::Check),
            "run" => Ok(CargoCommand::Run),
            "test" => Ok(CargoCommand::Test),
            _ => Err(format!("Invalid cargo command {}", s)),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str), hidden = true)]
    _psx: String,

    #[structopt(long, help = "run `cargo clean` before the build subcommand")]
    clean: bool,
    #[structopt(name = "build|check|run|test", parse(try_from_str))]
    cargo_subcmd: Option<CargoCommand>,

    #[structopt(long, help = "Sets the rustup toolchain (defaults to `psx`)")]
    toolchain: Option<String>,
    #[structopt(long, help = "Sets the game region to NA, EU or J")]
    region: Option<String>,
    #[structopt(long, help = "Specifies a custom linker script to use")]
    link: Option<String>,
    #[structopt(long, help = "Builds the `alloc` crate")]
    alloc: bool,
    #[structopt(
        long,
        help = "Enables link-time optimization and sets codegen units to 1"
    )]
    lto: bool,
    #[structopt(long, help = "Sets opt-level=s to optimize for size")]
    small: bool,

    #[structopt(long)]
    cargo_args: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    const TARGET_TRIPLE: &str = "mipsel-sony-psx";
    const LTO_FLAGS: &str = " -Clto=fat -Ccodegen-units=1 -Cembed-bitcode=yes";
    const SMALL_FLAGS: &str = " -Copt-level=s";
    const LINKER_SCRIPT_ARG: &str = "-Clink-arg=-T";
    const CARGO_CMD: &str = "cargo";
    const BUILD_STD: &str = "-Zbuild-std=core";

    let mut cargo_args: Vec<String> = opt
        .cargo_args
        .iter()
        .map(|arg| {
            let mut s = arg.to_string();
            s.insert_str(0, "--");
            s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>()
        })
        .flatten()
        .collect();

    // Always compile in release mode
    cargo_args.push("--release".to_string());

    // Set specified region
    let region = opt.region.map(|s| format!("psx/{}_region", s));
    if let Some(region) = region {
        cargo_args.push("--features".to_string());
        cargo_args.push(region);
    }

    // Set toolchain if not default
    let toolchain = match opt.toolchain {
        Some(name) => format!("+{}", name),
        None => "+psx".to_string(),
    };

    // Set build-std option to pass to cargo
    let mut build_std = BUILD_STD.to_string();
    if opt.alloc {
        build_std.push_str(",alloc");
    };

    // Try getting RUSTFLAGS from env
    let mut rustflags = env::var("RUSTFLAGS").ok().unwrap_or(String::new());

    // Set linker script if any
    if let Some(script) = opt.link {
        rustflags.push_str(&format!(" {}{}", LINKER_SCRIPT_ARG, script));
    };

    // Set other RUSTFLAGS
    if opt.lto {
        rustflags.push_str(LTO_FLAGS);
    }

    if opt.small {
        rustflags.push_str(SMALL_FLAGS);
    }

    let metadata = &MetadataCommand::new()
        .exec()
        .expect("Could not parse metadata");

    if opt.clean {
        for pkg in &metadata.packages {
            let mut clean = Command::new(CARGO_CMD)
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
        }
    }

    if let Some(subcmd) = opt.cargo_subcmd {
        let subcmd: &str = subcmd.into();
        let mut build = Command::new(CARGO_CMD)
            .arg(toolchain)
            .arg(subcmd)
            .arg(build_std)
            .arg("-Zbuild-std-features=compiler-builtins-mem")
            .arg("--target")
            .arg(TARGET_TRIPLE)
            .args(cargo_args)
            .env("RUSTFLAGS", rustflags)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect(&format!("`cargo {:?}` failed to start", subcmd));

        let status = build.wait().expect("`cargo build` wasn't running");
        if !status.success() {
            let code = status.code().unwrap_or(1);
            process::exit(code);
        }
    }
}
