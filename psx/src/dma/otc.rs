use crate::dma::Channel;
use crate::dma::Result;
use crate::hw::dma::otc::{Address, Block, Control};

pub struct OTC(Channel<Address, Block, Control>);

impl OTC {
    pub fn new() -> Self {
        OTC(Channel::new())
    }

    pub fn control(&mut self) -> &mut Control {
        &mut self.0.control
    }

    pub fn init(&mut self, block: &mut [u32]) -> Result<()> {
        self.0.send_and(block, || ())
    }

    pub fn init_and<F: FnOnce() -> R, R>(&mut self, block: &mut [u32], f: F) -> Result<R> {
        self.0.send_and(block, f)
    }
}

#[test_case]
fn one_entry() {
    use crate::gpu::Packet;
    let empty = Packet::empty();

    let mut otc = OTC::new();
    let mut buf = [0; 1];
    otc.init(&mut buf);
    assert!(empty.tag() == buf[0]);
}

//#[test_case]
fn multiple_entries() {
    use crate::gpu::Packet;
    let mut otc = OTC::new();
    let mut buf = [0; 4];
    otc.init(&mut buf);

    let mut buf2 = [0; 4];
    let packets = Packet::init_table(&mut buf2);

    //assert!(buf[0] == TERMINATION);
    //assert!(buf2[0] == TERMINATION);
    let diff = buf[1] - buf2[1];
    crate::println!("{:x?}", buf[1]);
    crate::println!("{:x?}", buf2[1]);
    let diff = buf[2] - buf2[2];
    crate::println!("{:x?}", buf[2]);
    crate::println!("{:x?}", buf2[2]);
    let diff = buf[3] - buf2[3];
    crate::println!("{:x?}", buf[3]);
    crate::println!("{:x?}", buf2[3]);
    //for n in 2..4 {
    //    let d2 = buf[n] - buf2[n];
    //    crate::println!("{:x?}", d2);
    //    assert!(diff == d2);
    //}
}
