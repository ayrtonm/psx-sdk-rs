use crate::cop0;

// TODO: Should I put this in MMIO? Technically it's not memory-mapped IO, but
// it kinda makes sense that it should be there (at least more sense that cop0).
pub struct GTE(());

impl GTE {
    pub fn enable(&mut self) {
        let mut stat = cop0::Status::read();
        stat.insert(cop0::Status::CU2);
        stat.write();
    }
}
