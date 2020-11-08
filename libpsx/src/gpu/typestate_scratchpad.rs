//use core::marker::PhantomData;
//pub struct Enabled;
//pub struct Disabled;
//pub trait Screen {
//    type Opposite: Screen;
//    const CMD: u32;
//    fn toggle() -> PhantomData::<Self::Opposite> {
//        bios::gpu_gp1_command_word(Self::CMD);
//        PhantomData::<Self::Opposite>
//    }
//}
//
//impl Screen for Disabled {
//    type Opposite = Enabled;
//    const CMD: u32 = 0x0300_0000;
//}
//
//impl Screen for Enabled {
//    type Opposite = Disabled;
//    const CMD: u32 = 0x0300_0001;
//}
