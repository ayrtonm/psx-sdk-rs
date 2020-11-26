use crate::registers::{BitTwiddle, Read, Update, Write};

pub enum BlockLen {
    // TODO: this should be u32 since 0x10000 is valid and gets mapped to 0u16
    Words(usize),
    Blocks { words: usize, blocks: usize },
    LinkedList,
}

pub enum Direction {
    ToRam,
    FromRam,
}

pub enum Step {
    Forward,
    Backward,
}

pub enum Mode {
    Immediate,
    Request,
    LinkedList,
}

pub trait Addr: Read + Write {
    fn get(&mut self) -> u32 {
        self.read()
    }

    fn set(&mut self, address: *const u32) {
        let mut address = address as u32;
        if cfg!(debug_assertions) {
            address &= 0x00FF_FFFF;
        }
        self.write(address);
    }
}

pub trait Block: Read + Write {
    // Note that this depends on sync mode, meaning that the channel may not
    // necessarily be in the given block mode
    fn set(&mut self, dma_blocks: BlockLen) {
        //TODO: add debug mode checks
        match dma_blocks {
            BlockLen::Words(words) => {
                let words = match words {
                    0..=0xFFFF => words as u32,
                    0x1_0000 => 0,
                    _ => unreachable!("Number of words can't exceed 0x1_0000"),
                };
                self.write(words);
            },
            BlockLen::Blocks { words, blocks } => {
                self.write(words as u32 | ((blocks as u32) << 16))
            },
            BlockLen::LinkedList => self.write(0),
        }
    }
}

pub trait Control: Update {
    fn set_direction(&mut self, direction: Direction) {
        let bit = match direction {
            Direction::ToRam => 0,
            Direction::FromRam => 1,
        };
        self.update(|val| val.clear(0) | bit);
    }

    fn set_step(&mut self, step: Step) {
        let bit = match step {
            Step::Forward => 0,
            Step::Backward => 1,
        };
        self.update(|val| val.clear(1) | (bit << 1));
    }
    fn set_chopping(&mut self, chop: bool) {
        let bit = if chop { 1 } else { 0 };
        self.update(|val| val.clear(8) | (bit << 8));
    }
    fn sync_mode(&self) -> Option<Mode> {
        let value = self.read();
        match value.bits(9..=10) {
            0 => Some(Mode::Immediate),
            1 => Some(Mode::Request),
            2 => Some(Mode::LinkedList),
            _ => None,
        }
    }
    fn set_sync_mode(&mut self, mode: Mode) {
        let bits = match mode {
            Mode::Immediate => 0,
            Mode::Request => 1,
            Mode::LinkedList => 2,
        };
        self.update_bits(9..=10, bits);
    }
    fn start<T: Copy>(&mut self, res: T) -> Transfer<Self, T> {
        self.update(|val| val.set(24));
        if let Some(Mode::Immediate) = self.sync_mode() {
            self.update(|val| val.set(28));
        }
        Transfer { control: self, res }
    }
    fn busy(&self) -> bool {
        self.read().bit(24) == 1
    }
}

#[must_use]
pub struct Transfer<'a, C: Control + ?Sized, T: Copy> {
    control: &'a C,
    res: T,
}

impl<C: Control, T: Copy> Transfer<'_, C, T> {
    pub fn busy(&self) -> bool {
        self.control.busy()
    }

    pub fn wait(&self) -> T {
        while self.busy() {}
        self.res
    }

    pub fn if_done(&self) -> Option<T> {
        if !self.busy() {
            Some(self.res)
        } else {
            None
        }
    }
}

pub struct Channel<A: Addr, B: Block, C: Control> {
    pub addr: A,
    pub block: B,
    pub control: C,
}

pub type Gpu = Channel<gpu::Addr, gpu::Block, gpu::Control>;
pub type Otc = Channel<otc::Addr, otc::Block, otc::Control>;

macro_rules! mk_mod {
    ($name:ident, $offset:expr) => {
        pub mod $name {
            use crate::rw_register;
            rw_register!(Addr, 0x1F80_1080 + ($offset * 0x10));
            rw_register!(Block, 0x1F80_1084 + ($offset * 0x10));
            rw_register!(Control, 0x1F80_1088 + ($offset * 0x10));

            impl super::Addr for Addr {}
            impl super::Block for Block {}
            impl super::Control for Control {}
        }
    };
}

mk_mod!(mdec_in, 0);
mk_mod!(mdec_out, 1);
mk_mod!(gpu, 2);
mk_mod!(cdrom, 3);
mk_mod!(spu, 4);
mk_mod!(pio, 5);
mk_mod!(otc, 6);
