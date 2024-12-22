# cargo-psx 

A `cargo build` wrapper for creating PlayStation 1 executables.

```

USAGE:
    cargo-psx [build|check|run|test] [OPTIONS]

OPTIONS:
        --cargo-args <CARGO_ARGS>

        --clean
            run `cargo clean` before the build subcommand
        --debug
            Ouputs an ELF with debug info
        --elf
            Outputs an ELF
        --features <FEATURES>
            Enables the listed features
    -h, --help
            Print help information
        --link <LINK>
            Specifies a custom linker script to use
        --load-offset <LOAD_OFFSET>
            Adds a load offset to the executable
        --lto
            Enables link-time optimization and sets codegen units to 1
        --small
            Sets opt-level=s to optimize for size
        --stack-pointer <STACK_POINTER>
            Sets the initial stack pointer
        --toolchain <TOOLCHAIN>
            Sets the rustup toolchain (defaults to `nightly`)
```
