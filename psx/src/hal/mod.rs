//! Hardware abstraction layer for memory-mapped I/O and coprocessor registers.
//!
//! This module provides an API for using memory-mapped I/O and coprocessor
//! registers directly. Volatile accesses are not elided by the compiler, so to
//! avoid unnecessary reads/writes readable register handles store a copy of
//! their value. For read-write registers this value must be explicitly written
//! back to the register.
//!
//! ```rust
//! // `Shared` registers can be `mut` to allow updating their stored value.
//! let mut irq_stat = I_STAT::<Shared>::load();
//! // waits for the next Vblank interrupt request then updates the stored value
//! // when the register is changed by the hardware.
//! irq_stat.wait(IRQ::Vblank);
//! // Acknowledging interrupt requests is not allowed here since writing to a
//! // `Shared` register is not allowed.
//! // irq_stat.ack(IRQ::CDROM).store();
//!
//! // Create a `Mutable` handle to use methods that write to registers.
//! let mut irq_mask = I_MASK::<Mutable>::load();
//! // The `enable_irq` method is available since the handle is `Mutable`.
//! irq_mask.enable_irq(IRQ::Timer0);
//! // The new value must then be stored in the register.
//! irq_mask.store();
//!
//! // If a write will modify the entire register, use `skip_load` to
//! // avoid reading the register.
//! let mut dma_control = DPCR::skip_load();
//! // `dma_control`'s stored value probably differs the register's current
//! // value, but `enable_all` will change all of the relevant bits. Also since
//! // `enable_all` returns `&mut Self`, `store` can be chained on.
//! dma_control.enable_all().store();
//! ```

#![allow(non_camel_case_types)]
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

/// Direct memory access channel and control registers.
pub mod dma;
/// GPU registers.
pub mod gpu;
/// Interrupt request registers.
pub mod irq;
/// Timer registers.
pub mod timer;

#[macro_use]
mod mmio;

/// The address corresponding to a
/// [memory-mapped I/O register](http://problemkaputt.de/psx-spx.htm#iomap).
pub trait Address {
    /// The 32-bit address corresponding to the register. Note there is no
    /// alignment constraint.
    const ADDRESS: u32;
}

/// Volatile memory-mapped I/O or coprocessor register reads.
pub trait Read<T> {
    /// Returns the register's current value. Note that the read will not be
    // /elided by the compiler.
    fn read(&self) -> T;
}

/// Volatile memory-mapped I/O or coprocessor register writes.
///
/// Note that the write(s) will not be elided by the compiler.
pub trait Write<T: Copy> {
    /// Writes `value` to the register.
    fn write(&mut self, value: T);

    /// Writes the slice of `values` to the register.
    fn write_slice(&mut self, values: &[T]) {
        for &v in values {
            self.write(v);
        }
    }
}

// Seal the `Register` and `MutRegister` traits.
mod private {
    use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

    pub trait HasValue<T> {
        fn get(&self) -> T;
        fn get_mut(&mut self) -> &mut T;
    }

    pub trait Primitive:
        Copy
        + PartialEq
        + Not<Output = Self>
        + BitAnd<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + BitAndAssign
        + BitOrAssign
        + BitXorAssign
        + From<u8> {
    }
    impl Primitive for u8 {}
    impl Primitive for u16 {}
    impl Primitive for u32 {}
}

/// Basic operations for all register handles.
///
/// All methods may be elided by the compiler unless stated otherwise.
pub trait Register<T: private::Primitive>: private::HasValue<T> + Read<T> {
    /// Reads the register and creates a handle with a copy of the register's
    /// current value. Note that the read will not be elided by the compiler.
    fn load() -> Self;

    /// Returns the handle's copy of the register.
    fn bits(&self) -> T {
        self.get()
    }

    /// Updates the handle's copy of the register. Note that the read will not
    /// be elided by the compiler. Returns `&mut Self` for convenience.
    fn reload(&mut self) -> &mut Self {
        let new_value = self.read();
        *self.get_mut() = new_value;
        self
    }

    /// Checks if any of the given `bits` are set in the handle's copy of the
    /// register.
    fn any_set(&self, bits: T) -> bool {
        self.get() & bits != T::from(0)
    }

    /// Checks if all of the given `bits` are set in the handle's copy of the
    /// register.
    fn all_set(&self, bits: T) -> bool {
        self.get() & bits == bits
    }

    /// Checks if all of the given `bits` are cleared in the handle's copy
    /// of the register.
    fn all_cleared(&self, bits: T) -> bool {
        self.get() & bits == T::from(0)
    }
}

/// Basic operations for [`Mutable`] register handles.
///
/// All methods may be elided by the compiler unless stated otherwise. Most
/// methods also return `&mut Self` for convenience.
pub trait MutRegister<T: private::Primitive>: Sized + Register<T> + Write<T> {
    /// Creates a handle without reading the register's current value.
    fn skip_load() -> Self;

    /// Writes the handle's copy of the register's value to the register. Note
    /// that the write will not be elided by the compiler.
    fn store(&mut self) -> &mut Self {
        self.write(self.get());
        self
    }

    /// Sets the handle's copy of the register to `bits`.
    fn assign(&mut self, bits: T) -> &mut Self {
        *self.get_mut() = bits;
        self
    }

    /// Sets the given `bits` in the handle's copy of the register.
    fn set_bits(&mut self, bits: T) -> &mut Self {
        *self.get_mut() |= bits;
        self
    }

    /// Clears the given `bits` in the handle's copy of the register.
    fn clear_bits(&mut self, bits: T) -> &mut Self {
        *self.get_mut() &= !bits;
        self
    }

    /// Toggles the given `bits` in the handle's copy of the register.
    fn toggle_bits(&mut self, bits: T) -> &mut Self {
        *self.get_mut() ^= bits;
        self
    }

    /// Sets all bits in the handle's copy of the register.
    fn set_all(&mut self) -> &mut Self {
        *self.get_mut() |= !T::from(0);
        self
    }

    /// Clears all bits in the handle's copy of the register.
    fn clear_all(&mut self) -> &mut Self {
        *self.get_mut() &= T::from(0);
        self
    }

    /// Toggles all bits in the handle's copy of the register.
    fn toggle_all(&mut self) -> &mut Self {
        *self.get_mut() ^= !T::from(0);
        self
    }
}

/// A marker trait for a register handle which may be shared between threads.
pub struct Shared {}
/// A marker trait for a mutable register handle.
pub struct Mutable {}
#[doc(hidden)]
pub trait State {}
impl State for Shared {}
impl State for Mutable {}

// Comments and address taken more or less verbatim from nocash specs
read_only! {
  /// Read responses to GP0(C0h) and GP1(10h) commands
  GPUREAD<u32>: 0x1F801810,
  /// Read GPU Status Register
  GPUSTAT<u32>: 0x1F801814
}

write_only! {
  /// Send GP0 Commands/Packets (Rendering and VRAM Access)
  GP0<u32>: 0x1F801810,
  /// Send GP1 Commands (Display Control)
  GP1<u32>: 0x1F801814
}

read_write! {
  /// Interrupt status register
  I_STAT<u16>: 0x1F801070,
  /// Interrupt mask register
  I_MASK<u16>: 0x1F801074,
  /// DMA0 channel 0 - MDECin base address register
  D0_MADR<u32>: 0x1F801080,
  /// DMA0 channel 0 - MDECin block control register
  D0_BCR<u32>: 0x1F801084,
  /// DMA0 channel 0 - MDECin channel control register
  D0_CHCR<u32>: 0x1F801088,
  /// DMA1 channel 1 - MDECout base address register
  D1_MADR<u32>: 0x1F801090,
  /// DMA1 channel 1 - MDECout block control register
  D1_BCR<u32>: 0x1F801094,
  /// DMA1 channel 1 - MDECout channel control register
  D1_CHCR<u32>: 0x1F801098,
  /// DMA2 channel 2 - GPU (lists + image data) base address register
  D2_MADR<u32>: 0x1F8010A0,
  /// DMA2 channel 2 - GPU (lists + image data) block control register
  D2_BCR<u32>: 0x1F8010A4,
  /// DMA2 channel 2 - GPU (lists + image data) channel control register
  D2_CHCR<u32>: 0x1F8010A8,
  /// DMA3 channel 3 - CDROM base address register
  D3_MADR<u32>: 0x1F8010B0,
  /// DMA3 channel 3 - CDROM block control register
  D3_BCR<u32>: 0x1F8010B4,
  /// DMA3 channel 3 - CDROM channel control register
  D3_CHCR<u32>: 0x1F8010B8,
  /// DMA4 channel 4 - SPU base address register
  D4_MADR<u32>: 0x1F8010C0,
  /// DMA4 channel 4 - SPU block control register
  D4_BCR<u32>: 0x1F8010C4,
  /// DMA4 channel 4 - SPU channel control register
  D4_CHCR<u32>: 0x1F8010C8,
  /// DMA5 channel 5 - PIO (Expansion Port) base address register
  D5_MADR<u32>: 0x1F8010D0,
  /// DMA5 channel 5 - PIO (Expansion Port) block control register
  D5_BCR<u32>: 0x1F8010D4,
  /// DMA5 channel 5 - PIO (Expansion Port) channel control register
  D5_CHCR<u32>: 0x1F8010D8,
  /// DMA6 channel 6 - OTC (reverse clear OT) (GPU related) base address register
  D6_MADR<u32>: 0x1F8010E0,
  /// DMA6 channel 6 - OTC (reverse clear OT) (GPU related) block control register
  D6_BCR<u32>: 0x1F8010E4,
  /// DMA6 channel 6 - OTC (reverse clear OT) (GPU related) channel control register
  D6_CHCR<u32>: 0x1F8010E8,
  /// DMA Control register
  DPCR<u32>: 0x1F8010F0,
  /// DMA Interrupt register
  DICR<u32>: 0x1F8010F4,
  /// Timer 0 Dotclock current counter value
  T0_CNT<u16>: 0x1F801100,
  /// Timer 0 Dotclock counter mode
  T0_MODE<u16>: 0x1F801104,
  /// Timer 0 Dotclock counter target value
  T0_TGT<u16>: 0x1F801108,
  /// Timer 1 Horizontal Retrace current counter value
  T1_CNT<u16>: 0x1F801110,
  /// Timer 1 Horizontal Retrace counter mode
  T1_MODE<u16>: 0x1F801114,
  /// Timer 1 Horizontal Retrace counter target value
  T1_TGT<u16>: 0x1F801118,
  /// Timer 2 1/8 system clock current counter value
  T2_CNT<u16>: 0x1F801120,
  /// Timer 2 1/8 system clock counter mode
  T2_MODE<u16>: 0x1F801124,
  /// Timer 2 1/8 system clock counter target value
  T2_TGT<u16>: 0x1F801128
}

/*
Expansion Region 1

  1F000000h 80000h Expansion Region (default 512 Kbytes, max 8 MBytes)
  1F000000h 100h   Expansion ROM Header (IDs and Entrypoints)

Scratchpad

  1F800000h 400h Scratchpad (1K Fast RAM) (Data Cache mapped to fixed address)

Memory Control 1

  1F801000h 4    Expansion 1 Base Address (usually 1F000000h)
  1F801004h 4    Expansion 2 Base Address (usually 1F802000h)
  1F801008h 4    Expansion 1 Delay/Size (usually 0013243Fh; 512Kbytes 8bit-bus)
  1F80100Ch 4    Expansion 3 Delay/Size (usually 00003022h; 1 byte)
  1F801010h 4    BIOS ROM    Delay/Size (usually 0013243Fh; 512Kbytes 8bit-bus)
  1F801014h 4    SPU_DELAY   Delay/Size (usually 200931E1h)
  1F801018h 4    CDROM_DELAY Delay/Size (usually 00020843h or 00020943h)
  1F80101Ch 4    Expansion 2 Delay/Size (usually 00070777h; 128-bytes 8bit-bus)
  1F801020h 4    COM_DELAY / COMMON_DELAY (00031125h or 0000132Ch or 00001325h)

Peripheral I/O Ports

  1F801040h 1/4  JOY_DATA Joypad/Memory Card Data (R/W)
  1F801044h 4    JOY_STAT Joypad/Memory Card Status (R)
  1F801048h 2    JOY_MODE Joypad/Memory Card Mode (R/W)
  1F80104Ah 2    JOY_CTRL Joypad/Memory Card Control (R/W)
  1F80104Eh 2    JOY_BAUD Joypad/Memory Card Baudrate (R/W)
  1F801050h 1/4  SIO_DATA Serial Port Data (R/W)
  1F801054h 4    SIO_STAT Serial Port Status (R)
  1F801058h 2    SIO_MODE Serial Port Mode (R/W)
  1F80105Ah 2    SIO_CTRL Serial Port Control (R/W)
  1F80105Ch 2    SIO_MISC Serial Port Internal Register (R/W)
  1F80105Eh 2    SIO_BAUD Serial Port Baudrate (R/W)

Memory Control 2

  1F801060h 4/2  RAM_SIZE (usually 00000B88h; 2MB RAM mirrored in first 8MB)

CDROM Registers (Address.Read/Write.Index)

  1F801800h.x.x   1   CD Index/Status Register (Bit0-1 R/W, Bit2-7 Read Only)
  1F801801h.R.x   1   CD Response Fifo (R) (usually with Index1)
  1F801802h.R.x   1/2 CD Data Fifo - 8bit/16bit (R) (usually with Index0..1)
  1F801803h.R.0   1   CD Interrupt Enable Register (R)
  1F801803h.R.1   1   CD Interrupt Flag Register (R/W)
  1F801803h.R.2   1   CD Interrupt Enable Register (R) (Mirror)
  1F801803h.R.3   1   CD Interrupt Flag Register (R/W) (Mirror)
  1F801801h.W.0   1   CD Command Register (W)
  1F801802h.W.0   1   CD Parameter Fifo (W)
  1F801803h.W.0   1   CD Request Register (W)
  1F801801h.W.1   1   Unknown/unused
  1F801802h.W.1   1   CD Interrupt Enable Register (W)
  1F801803h.W.1   1   CD Interrupt Flag Register (R/W)
  1F801801h.W.2   1   Unknown/unused
  1F801802h.W.2   1   CD Audio Volume for Left-CD-Out to Left-SPU-Input (W)
  1F801803h.W.2   1   CD Audio Volume for Left-CD-Out to Right-SPU-Input (W)
  1F801801h.W.3   1   CD Audio Volume for Right-CD-Out to Right-SPU-Input (W)
  1F801802h.W.3   1   CD Audio Volume for Right-CD-Out to Left-SPU-Input (W)
  1F801803h.W.3   1   CD Audio Volume Apply Changes (by writing bit5=1)

MDEC Registers

  1F801820h.Write 4   MDEC Command/Parameter Register (W)
  1F801820h.Read  4   MDEC Data/Response Register (R)
  1F801824h.Write 4   MDEC Control/Reset Register (W)
  1F801824h.Read  4   MDEC Status Register (R)

SPU Voice 0..23 Registers

  1F801C00h+N*10h 4   Voice 0..23 Volume Left/Right
  1F801C04h+N*10h 2   Voice 0..23 ADPCM Sample Rate
  1F801C06h+N*10h 2   Voice 0..23 ADPCM Start Address
  1F801C08h+N*10h 4   Voice 0..23 ADSR Attack/Decay/Sustain/Release
  1F801C0Ch+N*10h 2   Voice 0..23 ADSR Current Volume
  1F801C0Eh+N*10h 2   Voice 0..23 ADPCM Repeat Address

SPU Control Registers

  1F801D80h 4  Main Volume Left/Right
  1F801D84h 4  Reverb Output Volume Left/Right
  1F801D88h 4  Voice 0..23 Key ON (Start Attack/Decay/Sustain) (W)
  1F801D8Ch 4  Voice 0..23 Key OFF (Start Release) (W)
  1F801D90h 4  Voice 0..23 Channel FM (pitch lfo) mode (R/W)
  1F801D94h 4  Voice 0..23 Channel Noise mode (R/W)
  1F801D98h 4  Voice 0..23 Channel Reverb mode (R/W)
  1F801D9Ch 4  Voice 0..23 Channel ON/OFF (status) (R)
  1F801DA0h 2  Unknown? (R) or (W)
  1F801DA2h 2  Sound RAM Reverb Work Area Start Address
  1F801DA4h 2  Sound RAM IRQ Address
  1F801DA6h 2  Sound RAM Data Transfer Address
  1F801DA8h 2  Sound RAM Data Transfer Fifo
  1F801DAAh 2  SPU Control Register (SPUCNT)
  1F801DACh 2  Sound RAM Data Transfer Control
  1F801DAEh 2  SPU Status Register (SPUSTAT) (R)
  1F801DB0h 4  CD Volume Left/Right
  1F801DB4h 4  Extern Volume Left/Right
  1F801DB8h 4  Current Main Volume Left/Right
  1F801DBCh 4  Unknown? (R/W)

SPU Reverb Configuration Area

  1F801DC0h 2  dAPF1  Reverb APF Offset 1
  1F801DC2h 2  dAPF2  Reverb APF Offset 2
  1F801DC4h 2  vIIR   Reverb Reflection Volume 1
  1F801DC6h 2  vCOMB1 Reverb Comb Volume 1
  1F801DC8h 2  vCOMB2 Reverb Comb Volume 2
  1F801DCAh 2  vCOMB3 Reverb Comb Volume 3
  1F801DCCh 2  vCOMB4 Reverb Comb Volume 4
  1F801DCEh 2  vWALL  Reverb Reflection Volume 2
  1F801DD0h 2  vAPF1  Reverb APF Volume 1
  1F801DD2h 2  vAPF2  Reverb APF Volume 2
  1F801DD4h 4  mSAME  Reverb Same Side Reflection Address 1 Left/Right
  1F801DD8h 4  mCOMB1 Reverb Comb Address 1 Left/Right
  1F801DDCh 4  mCOMB2 Reverb Comb Address 2 Left/Right
  1F801DE0h 4  dSAME  Reverb Same Side Reflection Address 2 Left/Right
  1F801DE4h 4  mDIFF  Reverb Different Side Reflection Address 1 Left/Right
  1F801DE8h 4  mCOMB3 Reverb Comb Address 3 Left/Right
  1F801DECh 4  mCOMB4 Reverb Comb Address 4 Left/Right
  1F801DF0h 4  dDIFF  Reverb Different Side Reflection Address 2 Left/Right
  1F801DF4h 4  mAPF1  Reverb APF Address 1 Left/Right
  1F801DF8h 4  mAPF2  Reverb APF Address 2 Left/Right
  1F801DFCh 4  vIN    Reverb Input Volume Left/Right

SPU Internal Registers

  1F801E00h+N*04h  4 Voice 0..23 Current Volume Left/Right
  1F801E60h      20h Unknown? (R/W)
  1F801E80h     180h Unknown? (Read: FFh-filled) (Unused or Write only?)

Expansion Region 2 (default 128 bytes, max 8 KBytes)

  1F802000h      80h Expansion Region (8bit data bus, crashes on 16bit access?)

Expansion Region 2 - Dual Serial Port (for TTY Debug Terminal)

  1F802020h/1st    DUART Mode Register 1.A (R/W)
  1F802020h/2nd    DUART Mode Register 2.A (R/W)
  1F802021h/Read   DUART Status Register A (R)
  1F802021h/Write  DUART Clock Select Register A (W)
  1F802022h/Read   DUART Toggle Baud Rate Generator Test Mode (Read=Strobe)
  1F802022h/Write  DUART Command Register A (W)
  1F802023h/Read   DUART Rx Holding Register A (FIFO) (R)
  1F802023h/Write  DUART Tx Holding Register A (W)
  1F802024h/Read   DUART Input Port Change Register (R)
  1F802024h/Write  DUART Aux. Control Register (W)
  1F802025h/Read   DUART Interrupt Status Register (R)
  1F802025h/Write  DUART Interrupt Mask Register (W)
  1F802026h/Read   DUART Counter/Timer Current Value, Upper/Bit15-8 (R)
  1F802026h/Write  DUART Counter/Timer Reload Value,  Upper/Bit15-8 (W)
  1F802027h/Read   DUART Counter/Timer Current Value, Lower/Bit7-0 (R)
  1F802027h/Write  DUART Counter/Timer Reload Value,  Lower/Bit7-0 (W)
  1F802028h/1st    DUART Mode Register 1.B (R/W)
  1F802028h/2nd    DUART Mode Register 2.B (R/W)
  1F802029h/Read   DUART Status Register B (R)
  1F802029h/Write  DUART Clock Select Register B (W)
  1F80202Ah/Read   DUART Toggle 1X/16X Test Mode (Read=Strobe)
  1F80202Ah/Write  DUART Command Register B (W)
  1F80202Bh/Read   DUART Rx Holding Register B (FIFO) (R)
  1F80202Bh/Write  DUART Tx Holding Register B (W)
  1F80202Ch/None   DUART Reserved Register (neither R nor W)
  1F80202Dh/Read   DUART Input Port (R)
  1F80202Dh/Write  DUART Output Port Configuration Register (W)
  1F80202Eh/Read   DUART Start Counter Command (Read=Strobe)
  1F80202Eh/Write  DUART Set Output Port Bits Command (Set means Out=LOW)
  1F80202Fh/Read   DUART Stop Counter Command (Read=Strobe)
  1F80202Fh/Write  DUART Reset Output Port Bits Command (Reset means Out=HIGH)

Expansion Region 2 - Int/Dip/Post

  1F802000h 1 DTL-H2000: ATCONS STAT (R)
  1F802002h 1 DTL-H2000: ATCONS DATA (R and W)
  1F802004h 2 DTL-H2000: Whatever 16bit data ?
  1F802030h 1/4 DTL-H2000: Secondary IRQ10 Flags
  1F802032h 1 DTL-H2000: Whatever IRQ Control ?
  1F802040h 1 DTL-H2000: Bootmode "Dip switches" (R)
  1F802041h 1 PSX: POST (external 7 segment display, indicate BIOS boot status)
  1F802042h 1 DTL-H2000: POST/LED (similar to POST) (other addr, 2-digit wide)   1F802070h 1 PS2: POST2 (similar to POST, but PS2 BIOS uses this address)

Expansion Region 2 - Nocash Emulation Expansion

  1F802060h Emu-Expansion ID1 "E" (R)
  1F802061h Emu-Expansion ID2 "X" (R)
  1F802062h Emu-Expansion ID3 "P" (R)
  1F802063h Emu-Expansion Version (01h) (R)
  1F802064h Emu-Expansion Enable1 "O" (R/W)
  1F802065h Emu-Expansion Enable2 "N" (R/W)
  1F802066h Emu-Expansion Halt (R)
  1F802067h Emu-Expansion Turbo Mode Flags (R/W)

Expansion Region 3 (default 1 byte, max 2 MBytes)

  1FA00000h - Not used by BIOS or any PSX games
  1FA00000h - POST3 (similar to POST, but PS2 BIOS uses this address)

BIOS Region (default 512 Kbytes, max 4 MBytes)

  1FC00000h 80000h   BIOS ROM (512Kbytes) (Reset Entrypoint at BFC00000h)

Memory Control 3 (Cache Control)

  FFFE0130h 4        Cache Control
*/
