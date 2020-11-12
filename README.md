# psx-sdk-rs

This is a basic SDK to run custom Rust code on a Playstation 1. You'll need to
build the rust compiler from source with a patched version of LLVM. Building the
compiler and LLVM is computationally expensive, so it may take quite a bit of
time. See the [system requirements](https://rustc-dev-guide.rust-lang.org/getting-started.html#system-requirements)
for building the rust compiler and LLVM for more specifics. You'll also need to
build a MIPS assembler and a linker targetting `mipsel-unknown-elf`.

## Building the compiler

1. Clone the rust source:

    ```
    git clone https://github.com/rust-lang/rust.git
    cd rust
    ```

2. Configure the build settings:

    ```
    cp config.toml.example config.toml
    ```

3. Patch the rust compiler:

    ```
    git apply ../rustc_mips1.patch
    ```

4. Patch LLVM:

    ```
    git submodule update --init --progress src/llvm-project
    cd src/llvm-project
    git apply ../../../llvm_mips1.patch
    ```

5. Build the rust compiler:

    ```
    cd ../..
    ./x.py build --stage 1 compiler/rustc
    ```

6. Install a new toolchain with the compiler:

    ```
    rustup toolchain link psx build/x86_64-unknown-linux-gnu/stage1
    ```

Building the MIPS toolchain is as simple as running `cd mips_toolchain` then
`make`. By default the Makefile builds all the usual binutils binaries, although
only `ld`, `as`, `ar` and `objdump` are copied to the main toolchain directory.

## Building the demo
The `app/` directory contains a demo with some nice animation to verify the
compiler is working as xpected. Passing `-Z build-std=core` to cargo is required
to compile PlayStation executables, so `app/.cargo/config.toml` sets this by
default under the alias `cargo b`. The `rustc` option may also need to be
changed to point to the patched rust compiler if it's not in your `$PATH`.

Note `build-std` requires that the rust source code be present in the same
directory where it was compiled. If you'd like to avoid `build-std`, another
option might be to download the [core
source](https://docs.rs/rust-libcore/0.0.3/core/) and add it as a dependency
though the kinks are still being worked out with this method.

After compiling with cargo, the executable needs to be converted from ELF to the
PS-EXE format used on the console using the `elf2psexe` utility or by running
`app/finish.sh`.

## Program template
```rust
#![no_std]
#![no_main]

libpsx::exe!();

fn main() {
}
```

## Running executables on hardware

You'll also need a way to run custom "PS-EXE" executables on the
console, I (simias) use an Xplorer-FX flashed with caetla 0.34 and the
catflap4linux to control it.

## Todo

 - Update TODO list given the recent overhaul in libpsx
 - Figure out how to compile and use rust-lld
