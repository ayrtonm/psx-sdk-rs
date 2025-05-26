# Rustc build instructions

The `nightlier` feature enables 8 and 16-bit atomics, but requires patching LLVM.
Building the compiler is computationally expensive, so it may take quite a bit
of time. See the [system requirements](https://rustc-dev-guide.rust-lang.org/getting-started.html#system-requirements)
for building the rust compiler for more specifics.

## Building the compiler

1. Clone the rust source and checkout this specific commit:

    ```sh
    git clone https://github.com/rust-lang/rust.git
    cd rust
    git checkout 163cb4ea3f0ae3bc7921cc259a08a7bf92e73ee6
    ```

    Note that this particular commit has been chosen to match
    `nightly-2025-05-23`, and work with the required LLVM patch.

2. Configure the build script `bootstrap.toml` to use `rust-lld`.

    ```sh
    ./configure --enable-lld
    ```

    Optionally remove unnecessary targets to speed up the LLVM build by editing
    the generated `bootstrap.toml`:

    ```toml
    profile = 'dist'

    [llvm]
    targets = "Mips;X86"

    [build]
    configure-args = ['--enable-lld']

    [rust]
    lld = true
    ```

3. Patch LLVM.

    ```sh
    git submodule update --init --progress src/llvm-project
    cd src/llvm-project
    git apply /path/to/patches/llvm_atomic_fence.patch
    ```

4. Build the rust compiler. See `INSTALL.md` for further details.

    ```sh
    # For the initial build
    ./x.py build -i library/std
    # To rebuild
    ./x.py build -i library/std --keep-stage 1
    ```

5. Create a new toolchain with the patched compiler:

    ```sh
    rustup toolchain link psx build/x86_64-unknown-linux-gnu/stage1
    ```

    where `psx` is the name for the new toolchain.

6. When using `cargo-psx`, make sure to set the toolchain argument to `psx`.

    ```sh
    cargo psx run --toolchain psx
    ```
