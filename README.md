# psx-sdk-rs

This is a basic SDK to run custom Rust code on a Playstation 1. You'll need to
build the rust compiler from source. Building the compiler is computationally
expensive, so it may take quite a bit of time. See the [system requirements](https://rustc-dev-guide.rust-lang.org/getting-started.html#system-requirements)
for building the rust compiler for more specifics.

## Building the compiler

1. Clone the rust source and checkout this specific commit:

    ```
    git clone https://github.com/rust-lang/rust.git
    cd rust
    git checkout 2c858a7c3f189eb11ad89d9bf9f2e87cac9d2b76
    ```

2. Configure the build script to use `rust-lld` and optionally remove unnecessary targets to speed up the LLVM build:

    ```
    cp config.toml.example config.toml
    # Set lld to true
    sed -i 's/#lld = false/lld = true/' config.toml
    # Only build the MIPS and X86 targets
    sed -i 's/#targets.*$/targets = "Mips;X86"/' config.toml
    # Don't build any experimental targets
    sed -i 's/#experimental-targets.*$/experimental-targets = ""/' config.toml
    ```

3. Optionally patch LLVM to enable the `nightlier` feature. To use this feature set `max_atomic_width: Some(0),` to `Some(16)` in `patches/rustc_psx.patch` before applying it.

    ```
    git submodule update --init --progress src/llvm-project
    cd src/llvm-project
    git apply /path/to/patches/llvm_atomic_fence.patch
    ```


4. Patch the rust compiler. Applying these patches to a different commit may require manual intervention:

    ```
    git apply /path/to/patches/rustc_mips32.patch
    git apply /path/to/patches/rustc_psx.patch
    ```


5. Build the rust compiler:

    ```
    # For the initial build
    ./x.py build -i library/std
    # To rebuild
    ./x.py build -i library/std --keep-stage 1
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

To install:

```
cargo install cargo-psx
```

To uninstall:

```
cargo uninstall cargo-psx
```

For more options:

```
cargo psx --help
```
    
## Usage

The `examples` directory has some demos which have been tested in
[mednafen](https://mednafen.github.io/) with the SCPH7001 BIOS. Getting stdout
may require setting `psx.dbg_level` in `mednafen.cfg` to at least 2. Other BIOS
versions/regions may work but have not been tested. To try out a demo run `cargo
psx run` from its directory. To use a different emulator run `cargo psx build`
then open the .exe in `/target/mipsel-sony-psx/release/`. To use `cargo psx run`
with other emulators change the
[runner](https://doc.rust-lang.org/cargo/reference/config.html#target) for the
`mipsel-sony-psx` target. Note that some examples may require building with
`cargo psx run --alloc` to link in the alloc crate.

### Program template

To create a new program just use `cargo init` and add `psx = "*"` to
`Cargo.toml` under `[dependencies]`. Then replace `src/main.rs` with the
following template

```rust
#![no_std]
#![no_main]

// This can be any import from the `psx` crate.
use psx;

#[no_mangle]
fn main() {
}
```

[`no_std`](https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html#what-does-no_std-mean)
is required to link the `core` crate instead of `std`. `no_main` tells rustc to
make no assumption about the [program's entry
point](https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html#the-code).
The attribute on `main` is required since the entry point defined in the `psx`
crate expects to call an [`unmangled
function`](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html#no_mangle).
The `main` function should not return, but the return type can be either `()`,
`!` or `Result<()>`.

Optionally create a `.cargo` directory and a `config.toml` inside with the
following to allow running with `cargo psx run`

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
