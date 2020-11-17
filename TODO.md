# Todo

## libpsx

This is a very preliminary list of things missing in libpsx (almost
everything...). Very roughly in order of priority.

- [x] Use allocator
    - [x] Add call to heap init in `libpsx::exe`
    - [x] Add a `no heap` option
    - [x] Add a real alloc-free option (`no heap` w/o building alloc)
    - [ ] Fix linker error for `alloc` crate with rust-lld
- [ ] Add DMA channels
    - [x] Basic GPU DMA channel
    - [ ] Other DMA channels
    - [ ] DMA control/interrupt registers
- [ ] Finish GPU support
  - [x] Support DMA
  - [ ] Add texture support
  - [ ] Finish VRAM copy functions
  - [ ] Add draw quad functions with logical vertex ordering
  - [ ] Make framebuffer more flexible
    - [ ] Add version without `RefCell`
    - [ ] Consider a version with two-step swap (first call `draw` then `display`)
    - [ ] Allow setting color depth, video mode and interlacing
  - [ ] Support depth ordering tables
  - [ ] Support timer
- [ ] Add controller support
- [ ] Add interrupt control
    - [ ] Make allocator impl interrupt-free
- [ ] Add CDROM/ISO support
    - [ ] via kernel
        - [ ] fix `load_exe` demo
    - [ ] via IO registers
- [ ] Add thread support via kernel functions
    - [ ] Figure out how this connects to the Atomic API that was disabled in rustc
- [ ] Finish adding relevant kernel functions
    - [ ] Try inlining bios asm trampolines
- [ ] Add relevant coprocessor 0 asm snippets
- [ ] Add GTE support
    - [ ] Add coprocessor 2 asm snippets
- [ ] Add SPU support
- [ ] Add MDEC support
- [ ] Add memory card support
- [x] Replace all `unwrap` with `expect`

## cargo-psx

This is a list of pending features for cargo-psx in a pretty random order.

- [x] Replace all `unwrap` with `expect`
- [x] Pad psexe size to multiple of 0x800
    - [x] Fix the case where the file size is already a multiple of 0x800
- [ ] Figure out the multi-psexe story for large binaries
- [ ] Decide whether to add rudimentary ISO support
- [x] Make build-std configurable for alloc-free binaries
- [x] Throw in `RUSTFLAGS` env variable and see if adding `RUSTC` makes sense
- [ ] Make output .psexe name configurable
    - [x] Add compiler profile (release/debug) to default names
    - [x] Consider adding region to default names
