use crate::gpu::vertex::Component;

pub enum Hres {
    H256,
    H320,
    H368,
    H512,
    H640,
}
pub enum Vres {
    V240,
    V480,
}
pub enum Vmode {
    NTSC,
    PAL,
}
pub enum Depth {
    Lo,
    Hi,
}
pub enum DmaSource {
    Off,
    FIFO,
    CPU,
    GPU,
}

pub type Res = (Hres, Vres);

impl From<&Hres> for Component {
    fn from(h: &Hres) -> Component {
        match h {
            Hres::H256 => 256,
            Hres::H320 => 320,
            Hres::H368 => 368,
            Hres::H512 => 512,
            Hres::H640 => 640,
        }
    }
}

impl From<&Vres> for Component {
    fn from(v: &Vres) -> Component {
        match v {
            Vres::V240 => 240,
            Vres::V480 => 480,
        }
    }
}
