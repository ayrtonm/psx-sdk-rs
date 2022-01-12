use crate::dma::{Channel, ChannelControl, Direction, Result, Step};
use crate::gpu::Packet;
use crate::hw::dma::otc::{Address, Block, Control};
use core::slice;

pub struct OTC(Channel<Address, Block, Control>);

impl OTC {
    pub fn new() -> Self {
        OTC(Channel::new())
    }

    pub fn control(&mut self) -> &mut Control {
        &mut self.0.control
    }

    pub fn init<'a>(&mut self, list: &'a mut [u32]) -> Result<&'a mut [Packet<()>]> {
        let (ordering_table, _) = self.init_and(list, || ())?;
        Ok(ordering_table)
    }

    pub fn init_and<'a, F: FnOnce() -> R, R>(&mut self, list: &'a mut [u32], f: F) -> Result<(&'a mut [Packet<()>], R)> {
        self.control()
            .set_direction(Direction::ToMemory)
            .set_step(Step::Backward);
        let res = self.0.send_and(list, f)?;
        let ordering_table = unsafe {
            slice::from_raw_parts_mut(list.as_ptr() as *mut Packet<()>, list.len())
        };
        Ok((ordering_table, res))
    }
}

// This test is flaky
//#[test_case]
//fn one_entry() {
//    use crate::gpu::Packet;
//    fuzz!(|x: u32| {
//        let empty = Packet::empty();
//        let mut otc = OTC::new();
//        let mut buf = [x; 1];
//        otc.init(&mut buf).expect("OTC DMA failed");
//        //crate::println!("{} =? {}", empty.tag(), buf[0]);
//        assert!(empty.tag() == buf[0]);
//    });
//}

//#[test_case]
//fn multiple_entries() {
//    use crate::gpu::Packet;
//    let mut otc = OTC::new();
//    let mut buf = [0; 4];
//    otc.init(&mut buf);
//
//    let mut buf2 = [0; 4];
//    let packets = Packet::init_table(&mut buf2);
//
//    //assert!(buf[0] == TERMINATION);
//    //assert!(buf2[0] == TERMINATION);
//    let diff = buf[1] - buf2[1];
//    crate::println!("{:x?}", buf[1]);
//    crate::println!("{:x?}", buf2[1]);
//    let diff = buf[2] - buf2[2];
//    crate::println!("{:x?}", buf[2]);
//    crate::println!("{:x?}", buf2[2]);
//    let diff = buf[3] - buf2[3];
//    crate::println!("{:x?}", buf[3]);
//    crate::println!("{:x?}", buf2[3]);
//    //for n in 2..4 {
//    //    let d2 = buf[n] - buf2[n];
//    //    crate::println!("{:x?}", d2);
//    //    assert!(diff == d2);
//    //}
//}
