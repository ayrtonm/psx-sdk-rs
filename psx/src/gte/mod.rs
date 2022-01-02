use crate::hw::Register;
use crate::hw::gte::{VXY0, VZ0};

pub struct V3 {
    vxy: VXY0,
    vz: VZ0,
}

impl V3 {
    pub fn new([x, y, z]: [i16; 3]) -> Self {
        let x = x as u32;
        let y = y as u32;
        let mut vxy = VXY0::skip_load();
        vxy.assign(x | y << 16);
        let mut vz = VZ0::skip_load();
        vz.assign(z);
        Self {
            vxy,
            vz
        }
    }

    pub fn use(&mut self) {
        self.vxy.store();
        self.vz.store();
        // gte cmd goes here
    }
}
