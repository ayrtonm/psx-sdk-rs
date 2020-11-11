// This refers to the same `Component` alias in libpsx::gpu::vertex, but the
// types may not match for performance reasons
pub type Component = u32;

//pub struct Res {
//    h: Hres,
//    v: Vres,
//}

pub enum Hres { H256, H320, H368, H512, H640 }
pub enum Vres { V240, V480 }
pub enum Vmode { NTSC, PAL }
pub enum Depth { Lo, Hi }
pub enum DmaSource { Off, FIFO, CPU, GPU }

pub type Res = (Hres, Vres);
//impl Res {
//    pub fn new(h: Hres, v: Vres) -> Self {
//        Res { h, v }
//    }
//    pub fn h(&self) -> u32 {
//        (&self.h).into()
//    }
//    pub fn v(&self) -> u32 {
//        (&self.v).into()
//    }
//}

impl From<&Hres> for u32 {
    fn from(h: &Hres) -> u32 {
        match h {
            Hres::H256 => 256,
            Hres::H320 => 320,
            Hres::H368 => 368,
            Hres::H512 => 512,
            Hres::H640 => 640,
        }
    }
}

impl From<&Vres> for u32 {
    fn from(v: &Vres) -> u32 {
        match v {
            Vres::V240 => 240,
            Vres::V480 => 480,
        }
    }
}
