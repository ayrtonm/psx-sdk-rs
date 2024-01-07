#![allow(missing_docs, dead_code)]

//! Gamepad and memory card registers
use crate::hw::{MemRegister, Register};

pub struct TxData(MemRegister<u8, 0x1F80_1040>);
pub struct RxData(MemRegister<u8, 0x1F80_1040>);
pub type Status = MemRegister<u32, 0x1F80_1044>;
pub type Mode = MemRegister<u16, 0x1F80_1048>;
pub type Control = MemRegister<u16, 0x1F80_104A>;
pub type Baud = MemRegister<u16, 0x1F80_104E>;

impl TxData {
    /// Sends data to the I/O port, whether it's ready for writes or not.
    pub fn send(x: u8) {
        let mut tx = Self(MemRegister::skip_load());
        tx.0.assign(x).store();
    }
}

impl RxData {
    /// Gets the data at the I/O port, whether it's ready to be read or not.
    pub fn recv() -> u8 {
        let rx = Self(MemRegister::new());
        rx.0.to_bits()
    }
}

impl Status {
    const TX_READY: u32 = 0;
    const RX_NOT_EMPTY: u32 = 1;
    const RX_PARITY_ERROR: u32 = 3;
    const ACK_INPUT: u32 = 7;
    const IRQ: u32 = 9;
    const BAUD_TIMER: u32 = 11;

    pub fn tx_ready(&self) -> bool {
        self.all_set(1 << Self::TX_READY)
    }

    pub fn rx_not_empty(&self) -> bool {
        self.all_set(1 << Self::RX_NOT_EMPTY)
    }
}

#[repr(u16)]
pub enum BaudFactor {
    Mul1 = 1,
    Mul16 = 2,
    Mul64 = 3,
}

#[repr(u16)]
pub enum CharLength {
    Bits5 = 0,
    Bits6 = 1,
    Bits7 = 2,
    Bits8 = 3,
}

#[repr(u16)]
pub enum RxIntMode {
    Bits1 = 0,
    Bits2 = 1,
    Bits4 = 2,
    Bits8 = 3,
}

impl Mode {
    const BAUD_RELOAD_FACTOR: u32 = 0;
    const CHAR_LENGTH: u32 = 2;
    const PARITY_ENABLE: u32 = 4;
    const PARITY_TYPE: u32 = 5;
    const CLK_OUTPUT_POLARITY: u32 = 8;

    pub fn set_baud_factor(&mut self, baud_factor: BaudFactor) -> &mut Self {
        self.clear_bits(0b11).set_bits(baud_factor as u16)
    }

    pub fn set_char_length(&mut self, char_length: CharLength) -> &mut Self {
        self.clear_bits(0b11 << Self::CHAR_LENGTH)
            .set_bits((char_length as u16) << Self::CHAR_LENGTH)
    }
}

impl Control {
    const TX_ENABLE: u32 = 0;
    const JOYN_OUTPUT: u32 = 1;
    const RX_ENABLE: u32 = 2;
    const ACK: u32 = 4;
    const RESET: u32 = 6;
    const RX_INT_MODE: u32 = 8;
    const TX_INT_ENABLE: u32 = 10;
    const RX_INT_ENABLE: u32 = 11;
    const ACK_INT_ENABLE: u32 = 12;
    const SLOT_NUM: u32 = 13;

    pub fn enable_tx(&mut self) -> &mut Self {
        self.set_bits(1 << Self::TX_ENABLE)
    }

    pub fn disable_tx(&mut self) -> &mut Self {
        self.clear_bits(1 << Self::TX_ENABLE)
    }

    pub fn enable_output(&mut self) -> &mut Self {
        self.set_bits(1 << Self::JOYN_OUTPUT)
    }

    pub fn disable_output(&mut self) -> &mut Self {
        self.clear_bits(1 << Self::JOYN_OUTPUT)
    }

    pub fn select_p1(&mut self) -> &mut Self {
        self.clear_bits(1 << Self::SLOT_NUM)
    }

    pub fn select_p2(&mut self) -> &mut Self {
        self.set_bits(1 << Self::SLOT_NUM)
    }

    pub fn ack(&mut self) -> &mut Self {
        self.set_bits(1 << Self::ACK)
    }

    pub fn enable_rx_interrupt(&mut self) -> &mut Self {
        self.set_bits(1 << Self::RX_INT_ENABLE)
    }

    pub fn disable_rx_interrupt(&mut self) -> &mut Self {
        self.clear_bits(1 << Self::RX_INT_ENABLE)
    }

    pub fn enable_ack_interrupt(&mut self) -> &mut Self {
        self.set_bits(1 << Self::ACK_INT_ENABLE)
    }

    pub fn disable_ack_interrupt(&mut self) -> &mut Self {
        self.clear_bits(1 << Self::ACK_INT_ENABLE)
    }

    pub fn set_rx_interrupt_mode(&mut self, mode: RxIntMode) -> &mut Self {
        self.clear_bits(0b11 << Self::RX_INT_MODE)
            .set_bits((mode as u16) << Self::RX_INT_MODE)
    }
}
