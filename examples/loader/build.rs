use std::env::current_dir;
use std::process::{exit, Command};

fn main() {
    // Get ferris demo directory
    let mut dir = current_dir().expect("Couldn't get current directory");
    dir.pop();
    dir.push("ferris");

    println!("cargo:rerun-if-changed={}", dir.to_string_lossy());

    // Build the ferris demo as a loadable executable for a particular offset in RAM
    let mut build = Command::new("cargo")
        .args([
            "psx",
            "build",
            "--lto",
            "--features",
            "psx/loadable_exe,psx/heap",
            "--load-offset=524288",
            "--stack-pointer=0",
        ])
        .current_dir(dir)
        .spawn()
        .expect("`cargo psx build` failed to start");
    let status = build.wait().expect("Build didn't run");
    if !status.success() {
        let code = status.code().unwrap_or(1);
        exit(code);
    }
}
