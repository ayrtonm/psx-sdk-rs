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

    Note that enabling incremental compilation here only affects the build of
    the compiler itself, not any code generated for the PlayStation. This can
    speed up compilation of the compiler after an initial build, but comes at
    the cost of increased memory usage and storage requirements.

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

`cargo-psx` acts as a wrapper for `cargo-build` and `cargo-check` in addition to
simias's `elf2psexe` utility. Basically this lets you run `cargo psx` to build
instead of `cargo build +psx -Z build-std=core,alloc --target=mipsel-sony-psx`.

To install, just do:

```
cd cargo-psx
cargo install --path .
```
    
## Usage

The `examples` directory has some demos which may or may not be broken at the
moment due to changes in the `psx` crate. To try one out just run `cargo psx`
from the demo's directory. This defaults to building an ELF using a toolchain
named `psx` and repackaging it into a PSEXE with region `NA`. See `cargo psx -h`
for more.

### Program template

To create a new program just use `cargo-init`, replace `src/main.rs` with
this template and add `psx = { path = "path/to/psx/crate" }` to `Cargo.toml`
under `[dependencies]`. Note the unmangled main interface.

```rust
#![no_std]
#![no_main]

extern crate psx;

#[no_mangle]
fn main() {
}
```

## Documentation

To generate documentation for the `psx` crate:

```
cd psx
cargo doc --target mipsel-unknown-linux-gnu
```

Then open `target/mipsel-unknown-linux-gnu/doc/psx/index.html` in a browser.
Once things become a bit more stable I'll probably document things more
thoroughly and link a tutorial here.

## Optionally running executables on hardware

You'll also need a way to run custom "PS-EXE" executables on the
console, I (simias) use an Xplorer-FX flashed with caetla 0.34 and the
catflap4linux to control it.
