psx-sdk-rs
==========

Basic SDK to run custom Rust code on a Playstation.

In order to use this you'll need a nighly rustc (since we use unstable
features), the rust source (since we need to crosscompile libcore) and
a linker targetting `mipsel-unknown-elf`.

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
