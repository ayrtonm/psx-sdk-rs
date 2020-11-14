# Todo

## Libpsx

This is a very preliminary list of things missing in libpsx (almost
everything...). Very roughly in order of priority.

- [ ] Replace all `unwrap` with `expect`
- [ ] Add controller support
- [ ] Finish GPU support
  - [ ] Add texture support
  - [ ] Finish VRAM copy functions
  - [ ] Make framebuffer more flexible
    - [ ] Add version without `RefCell`
    - [ ] Allow setting color depth, video mode and interlacing
  - [ ] Support depth ordering tables
  - [ ] Support DMA
  - [ ] Support timer
- [ ] Add interrupt control
- [ ] Add DMA channels
- [ ] Use allocator
    - [ ] Add call to heap init in `libpsx::exe`
    - [ ] Add a real alloc-free option
- [ ] Finish adding relevant kernel functions
    - [ ] Try inlining bios asm trampolines
- [ ] Add relevant coprocessor 0 asm snippets
- [ ] Add GTE support
     [ ] Add coprocessor 2 asm snippets
- [ ] Add SPU support
- [ ] Add MDEC support
- [ ] Add memory card support
- [ ] Add CDROM/ISO support

## Cargo-psx

This is a list of pending features for cargo-psx in a pretty random order.

- [ ] Replace all `unwrap` with `expect`
- [x] Pad psexe size to multiple of 0x800
    - [ ] Fix the case where the file size is already a multiple of 0x800
- [ ] Figure out the multi-psexe story for large binaries
- [ ] Decide whether to add rudimentary ISO support
- [ ] Make build-std configurable for alloc-free binaries
- [ ] Throw in `RUSTFLAGS` env variable and see if adding `RUSTC` makes sense
