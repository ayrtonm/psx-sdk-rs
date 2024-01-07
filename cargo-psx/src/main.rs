use cargo_metadata::MetadataCommand;
use clap::Parser;
use std::env;
use std::process::{self, Command, Stdio};
use std::str::FromStr;

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

#[derive(Debug, Parser)]
struct Opt {
    #[clap(parse(try_from_str), hide = true)]
    _psx: String,

    #[clap(long, help = "run `cargo clean` before the build subcommand")]
    clean: bool,
    #[clap(name = "build|check|run|test", parse(try_from_str))]
    cargo_subcmd: Option<CargoCommand>,

    #[clap(long, help = "Sets the rustup toolchain (defaults to `nightly`)")]
    toolchain: Option<String>,
    #[clap(long, help = "Specifies a custom linker script to use")]
    link: Option<String>,
    #[clap(long, help = "Outputs an ELF")]
    elf: bool,
    #[clap(long, help = "Ouputs an ELF with debug info")]
    debug: bool,
    #[clap(
        long,
        help = "Enables link-time optimization and sets codegen units to 1"
    )]
    lto: bool,
    #[clap(long, help = "Sets opt-level=s to optimize for size")]
    small: bool,

    #[clap(long, help = "Adds a load offset to the executable")]
    load_offset: Option<u32>,

    #[clap(long, help = "Sets the initial stack pointer")]
    stack_pointer: Option<u32>,

    #[clap(long)]
    cargo_args: Vec<String>,

    #[clap(long, help = "Enables the listed features")]
    features: Option<String>,
}

fn main() {
    let opt = Opt::parse();

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

    // Set toolchain if not default
    let toolchain = match opt.toolchain {
        Some(name) => format!("+{}", name),
        None => "+nightly".to_string(),
    };

    // Set build-std option to pass to cargo
    let build_std = "-Zbuild-std=core,alloc".to_string();

    // Rust doesn't do cross-crate inlining unless functions are marked as
    // #[inline]. Pretty much everything in the psx crate should be inlined since
    // they're such low-level functions, but to avoid doing that manually we
    // codegen-units to 1 by default to get essentially the same effect without the
    // burden of always doing LTO. This default is overriden when setting RUSTFLAGS
    // through an env var, but the performance of builds without this flag is
    // extremely unpredictable.
    let default_rustflags = "-Ccodegen-units=1".to_string();
    // Try getting RUSTFLAGS from env
    let mut rustflags = env::var("RUSTFLAGS").ok().unwrap_or(default_rustflags);

    // Set linker script if any
    let script = opt.link.unwrap_or("psexe.ld".to_string());
    rustflags.push_str(&format!(" -Clink-arg=-T{}", script));
    let format = if opt.debug || opt.elf {
        "elf32-tradlittlemips"
    } else {
        "binary"
    };
    rustflags.push_str(&format!(" -Clink-arg=--oformat={}", format));

    // Set optional RUSTFLAGS
    if opt.debug {
        rustflags.push_str(" -g");
    }

    if opt.lto {
        rustflags.push_str(" -Clto=fat -Cembed-bitcode=yes");
    }

    if opt.small {
        rustflags.push_str(" -Copt-level=s");
    }

    let metadata = &MetadataCommand::new()
        .exec()
        .expect("Could not parse metadata");

    const CARGO_CMD: &str = "cargo";
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
        let mut cmd = Command::new(CARGO_CMD);
        cmd.arg(toolchain)
            .arg(subcmd)
            .arg(build_std)
            .arg("-Zbuild-std-features=compiler-builtins-mem")
            .arg("--target")
            .arg("mipsel-sony-psx")
            .args(cargo_args)
            .env("RUSTFLAGS", rustflags);
        if let Some(features) = opt.features {
            cmd.arg("--features").arg(features);
        }
        if let Some(offset) = opt.load_offset {
            assert!(offset % 4 == 0, "Load offset must be a multiple of 4 bytes");
            cmd.env("PSX_LOAD_OFFSET", offset.to_string());
        }
        if let Some(sp) = opt.stack_pointer {
            assert!(
                sp % 4 == 0,
                "Initial stack pointer must be a multiple of 4 bytes"
            );
            cmd.env("PSX_STACK_POINTER", sp.to_string());
        }
        let mut build = cmd
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
