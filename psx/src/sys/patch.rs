//! Unsafe patches to the BIOS
//!
//! This module contains functions that directly patch the BIOS code (in the
//! first 64 kilobytes of RAM), to work around issues in Sony's implementation.

use crate::sys::kernel;

/// This function sets the pad output buffers to the specified pointers. Note
/// that the size (usize parameters) are never used in the BIOS code, so you
/// don't have to care about them.
///
/// The first byte of the buffer must be a non-zero value, or else the BIOS will
/// ignore it
///
/// The BIOS will output the data within the buffer (from the second byte of the
/// buffer), after it has issued a Read command to the controller.
///
/// # Example
///
/// | Buffer               | What the BIOS sends to the controller |
/// |----------------------|---------------------------------------|
/// | `[0xFF, 0xDE, 0xAD]` | `0x01 0x42 0x00 0xDE 0xAD`            |
///
/// # Parameters
/// 1. Pointer to a buffer containing bytes to output to the player
/// 1 controller after Read command.
///
/// 2. Size of the buffer (player 1/ignored).
///
/// 3. Pointer to a buffer containing bytes to output to the player
/// 2 controller after Read command.
///
/// 4. Size of the buffer (player 2/ignored).
pub(crate) type PadOutputFunction = extern "C" fn(*const u8, usize, *const u8, usize);

/// Fetch the internal BIOS function responsible for setting the pad output
/// buffers (also known as `setPadOutputData`), and patches the `readPad`
/// function to disable the 0-1 data clipping
pub(crate) unsafe fn send_pad_output() -> PadOutputFunction {
    // This code was inspired by the games that employed this BIOS patch, which
    // includes Resident Evil 2 and Lemmings.
    // https://psx-spx.consoledev.net/kernelbios/#patch_optional_pad_output
    static NEW_CODE: [u32; 4] = [
        0x00551024, // and  r2,r21
        0x00000000, // nop
        0x00000000, // nop
        0x00000000, // nop
    ];

    static PAD_OUTPUT_OFFSET: usize = 0x7A0;
    static READPAD_1_OFFSET: usize = 0x3D8;
    static READPAD_2_OFFSET: usize = 0x4DC;

    // Fetch the B syscall table from the kernel.
    let mut b0_table: *const u32 = kernel::psx_get_b0_table() as _;
    b0_table = b0_table.add(0x5B);

    // Code for the ChangeClearPAD function. We're using this as a starting point
    // for the code offsets.
    let mut chgclearpad_func: *mut u32 = unsafe { b0_table.read() } as _;

    // Get the pointer to the pad output function.
    let pad_output_func: PadOutputFunction =
        core::mem::transmute(unsafe { chgclearpad_func.byte_add(PAD_OUTPUT_OFFSET) });

    // Patch a couple parts of the code related to the `readPad` function.
    for code in NEW_CODE {
        chgclearpad_func.byte_add(READPAD_1_OFFSET).write(code);
        chgclearpad_func.byte_add(READPAD_2_OFFSET).write(code);
        chgclearpad_func = chgclearpad_func.byte_add(4);
    }

    pad_output_func
}
