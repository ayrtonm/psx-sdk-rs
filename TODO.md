# Todo

## libpsx

This is a very preliminary list of things missing in libpsx (almost
everything...). Very roughly in order of priority.

- [ ] Replace all `unwrap` with `expect`
- [ ] Add controller support
- [ ] Finish GPU support
  - [ ] Add texture support
  - [ ] Finish VRAM copy functions
  - [ ] Add draw quad functions with logical vertex ordering
  - [ ] Make framebuffer more flexible
    - [ ] Add version without `RefCell`
    - [ ] Consider a version with two-step swap (first call `draw` then `display`)
    - [ ] Allow setting color depth, video mode and interlacing
  - [ ] Support depth ordering tables
  - [ ] Support DMA
  - [ ] Support timer
- [ ] Add interrupt control
    - [ ] Make allocator impl interrupt-free
- [ ] Add thread support via kernel functions
    - [ ] Figure out how this connects to the Atomic API that was disabled in rustc
- [ ] Add DMA channels
- [ ] Use allocator
    - [ ] Add call to heap init in `libpsx::exe`
    - [ ] Add a real alloc-free option
- [ ] Finish adding relevant kernel functions
    - [ ] Try inlining bios asm trampolines
- [ ] Add relevant coprocessor 0 asm snippets
- [ ] Add GTE support
    - [ ] Add coprocessor 2 asm snippets
- [ ] Add SPU support
- [ ] Add MDEC support
- [ ] Add memory card support
- [ ] Add CDROM/ISO support
    - [ ] via kernel
        - [ ] fix `load_exe` demo
    - [ ] via IO registers

## cargo-psx

This is a list of pending features for cargo-psx in a pretty random order.

- [ ] Replace all `unwrap` with `expect`
- [x] Pad psexe size to multiple of 0x800
    - [ ] Fix the case where the file size is already a multiple of 0x800
- [ ] Figure out the multi-psexe story for large binaries
- [ ] Decide whether to add rudimentary ISO support
- [ ] Make build-std configurable for alloc-free binaries
- [ ] Throw in `RUSTFLAGS` env variable and see if adding `RUSTC` makes sense
- [ ] Make output .psexe name configurable
    - [ ] Add compiler profile (release/debug) to default names
    - [ ] Consider adding region to default names
