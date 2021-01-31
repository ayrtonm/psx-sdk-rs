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

`cargo-psx` is an optional wrapper for cargo that sets some commonly required
flags and arguments. Basically this lets you just run `cargo psx run` instead of
`cargo run +psx -Z build-std=core,alloc --target mipsel-sony-psx`.

To install, just do:

```
cd cargo-psx
cargo install --path .
```

To uninstall, just do:

```
cargo uninstall cargo-psx
```
    
## Usage

The `examples` directory has some demos which may or may not be broken at the
moment due to changes in the `psx` crate. To try them out just run `cargo psx
run` from the demo's directory. The demos are configured to run in
[mednafen](https://mednafen.github.io/) by default. To use another emulator run
`cargo psx build` then open the PS-EXE in `/target/mipsel-sony-psx/release/`.
Some emulators may require appending the ".psexe" file extension to the
executable. To use `cargo psx run` with other emulators change the
[runner](https://doc.rust-lang.org/cargo/reference/config.html#target) for the
`mipsel-sony-psx` target.

### Program template

To create a new program just use `cargo init` and add `psx = { path =
"path/to/psx/crate" }` to `Cargo.toml` under `[dependencies]`. Then replace
`src/main.rs` with the following template

```rust
#![no_std]
#![no_main]

extern crate psx;

#[no_mangle]
fn main() {
}
```

Optionally create a `.cargo` directory and a `config.toml` inside with the
following

```
[target.mipsel-sony-psx]
runner = "mednafen"
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
