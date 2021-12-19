use crate::hw::cop0::EPC;
use crate::hw::Register;
use core::fmt;
use core::fmt::{Debug, Formatter};

impl Debug for EPC {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("cop0::EPC")
            .field("bits", &self.to_bits())
            .finish()
    }
}
