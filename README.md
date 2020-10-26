# psx-sdk-rs

This is a basic SDK to run custom Rust code on a Playstation 1. You'll need to
build the rust compiler from source with a patched version of LLVM. Building the
compiler and LLVM is computationally expensive, so it may take quite a bit
of time. See the [system requirements](https://rustc-dev-guide.rust-lang.org/getting-started.html#system-requirements)
for building the rust compiler and LLVM for more specifics. You'll also need a
MIPS assembler and a linker targetting `mipsel-unknown-elf`.


this you'll need to compile rustc from source using a patched version of LLVM
and a linker targetting `mipsel-unknown-elf`.

You'll also need a way to run custom "PS-EXE" executables on the
console, I use an Xplorer-FX flashed with caetla 0.34 and the
catflap4linux to control it.

`psx.ld` contains the linker script to put the executable at the
correct location in RAM (without overwriting the BIOS).

`elf2psexe` is a tool that converts the `ELF` executables produced by
`ld` into the standard `PS-EXE` format used on the console (and
understood by many Playstation utilities).

The applications are in `apps/`. The build system is a bit crappy
since I can't use cargo unfortunately. Instead I use ad-hoc Makefiles
and a global `config.mk` for the various settings.

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

6. Optionally install and rename the rust compiler:
    ```
    sudo cp build/x86_64-unknown-linux-gnu/stage1/bin/rustc /usr/local/bin/psx_rustc
    sudo cp -r build/x86_64-unknown-linux-gnu/stage1/lib/* /usr/local/lib/
    ```

Building the MIPS toolchain is as simple as `cd mips_toolchain` then running `make`. By default the Makefile builds all the usual binutils binaries, but only `ld`, `as`, `ar` and `objdump` are copied to the main toolchain directory.
