# Todo

## libpsx

This is a very preliminary list of things missing in libpsx (almost
everything...). Very roughly in order of priority.

- [ ] Use OnceCell/Lazy for Ctxt
- [ ] Add interrupt control
    - [x] Interrupt enable/disable
    - [ ] Interrupt acknowledge/status
    - [ ] Make allocator impl interrupt-free
- [ ] TIM texture loader
- [ ] Add DMA channels
    - [x] Basic GPU DMA channel
    - [ ] Other DMA channels
    - [ ] DMA control/interrupt registers
- [ ] Finish GPU support
  - [x] Support DMA
  - [ ] Add texture support
    - [x] Display texture in VRAM
    - [ ] Decide how to structure Texcoord+CLUT arguments
    - [ ] Display textured rectangles
  - [ ] Finish VRAM copy functions
  - [ ] Add draw quad functions with logical vertex ordering
  - [ ] Make framebuffer more flexible
    - [ ] Add version without `RefCell`
    - [ ] Consider a version with two-step swap (first call `draw` then `display`)
    - [ ] Allow setting color depth, video mode and interlacing
  - [ ] Support depth ordering tables
  - [ ] Support timer
- [x] Use allocator
    - [x] Add call to heap init in `libpsx::exe`
    - [x] Add a `no heap` option
    - [x] Add a real alloc-free option (`no heap` w/o building alloc)
    - [ ] Test collections (use GNU ld for now)
    - [ ] Fix linker error for `alloc` crate with rust-lld
- [ ] Add controller support
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
