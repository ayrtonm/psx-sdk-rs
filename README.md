# psx-sdk-rs

This is a basic SDK to run custom Rust code on a Playstation 1. You'll need to
build the rust compiler from source with a patched version of LLVM. Building the
compiler and LLVM is computationally expensive, so it may take quite a bit of
time. See the [system requirements](https://rustc-dev-guide.rust-lang.org/getting-started.html#system-requirements)
for building the rust compiler and LLVM for more specifics.

## Building the compiler

1. Clone the rust source:

    ```
    git clone https://github.com/rust-lang/rust.git
    cd rust
    ```

2. Configure the build script to use `rust-lld` and (optionally) incremental compilation:

    ```
    cp config.toml.example config.toml
    sed -i 's/#lld = false/lld = true/' config.toml
    sed -i 's/#incremental = false/incremental = true/' config.toml
    ```

3. Patch the rust compiler:

    ```
    git apply ../rustc_psx.patch
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

6. Create a new toolchain with the patched compiler:

    ```
    rustup toolchain link psx build/x86_64-unknown-linux-gnu/stage1
    ```

    where `psx` is the name for the new toolchain.

## Installing cargo-psx

```
cd cargo-psx
cargo install --path .
```
    
## Building the demo

```
cd examples/rotating_square
cargo psx --release
```

This defaults to building an ELF using a toolchain named `psx` and repackaging
it into a PSEXE with region `NA`. Note that building without `--release` (in
debug mode) is possible, but will probably be unusably slow. See `cargo psx -h`
for more options.

## Program template

```rust
#![no_std]
#![no_main]

libpsx::exe!();

fn main(mut ctxt: Ctxt) {
}
```

Note the modified `main` interface. See `examples` for more details.

## Optionally running executables on hardware

You'll also need a way to run custom "PS-EXE" executables on the
console, I (simias) use an Xplorer-FX flashed with caetla 0.34 and the
catflap4linux to control it.
