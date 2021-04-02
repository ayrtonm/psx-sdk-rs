use super::{Counter, Name};
use crate::hal::timer::{Current, Mode, Target};

impl<CNT, MODE, TGT, const NAME: Name> Counter<CNT, MODE, TGT, NAME>
where
    CNT: Current,
    MODE: Mode,
    TGT: Target,
{
    pub fn new() -> Self {
        Counter {
            cnt: CNT::load(),
            mode: MODE::load(),
            tgt: TGT::load(),
        }
    }

    pub fn reload(&mut self) {
        self.cnt.reload();
        self.mode.reload();
        self.tgt.reload();
    }

    pub fn split(self) -> (CNT, MODE, TGT) {
        (self.cnt, self.mode, self.tgt)
    }
}
