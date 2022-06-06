# Rustc build instructions

The `nightlier` feature enables 8 and 16-bit atomics, but requires patching LLVM
and the rust compiler. Building the compiler is computationally expensive, so it
may take quite a bit of time. See the [system requirements](https://rustc-dev-guide.rust-lang.org/getting-started.html#system-requirements)
for building the rust compiler for more specifics.

## Building the compiler

1. Clone the rust source and checkout this specific commit:

    ```
    git clone https://github.com/rust-lang/rust.git
    cd rust
    git checkout 3a8e71385940c2f02ec4b23876c0a36fd09bdefe
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

3. Patch LLVM.

    ```
    git submodule update --init --progress src/llvm-project
    cd src/llvm-project
    git apply /path/to/patches/llvm_atomic_fence.patch
    ```


4. Patch the rust compiler. Applying this patch to a different commit may require manual intervention:

    ```
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


7. When using `cargo-psx`, make sure to set the toolchain argument to `psx`.

    ```
    cargo psx run --toolchain psx
    ```
