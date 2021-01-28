use crate::hal::{Mutable, State};
use crate::hal::{D0_BCR, D0_CHCR, D0_MADR};
use crate::hal::{D1_BCR, D1_CHCR, D1_MADR};
use crate::hal::{D2_BCR, D2_CHCR, D2_MADR};
use crate::hal::{D3_BCR, D3_CHCR, D3_MADR};
use crate::hal::{D4_BCR, D4_CHCR, D4_MADR};
use crate::hal::{D5_BCR, D5_CHCR, D5_MADR};
use crate::hal::{D6_BCR, D6_CHCR, D6_MADR};

mod channel;
mod control;

pub use channel::BlockControl;
pub use channel::ChannelControl;
pub use channel::MemoryAddress;
pub use channel::SharedBlockControl;
pub use channel::SharedChannelControl;
pub use channel::SharedMemoryAddress;

macro_rules! channel {
    ([$madr:ident, $bcr:ident, $chcr:ident]) => {
        impl<S: State> SharedMemoryAddress for $madr<S> {}

        impl MemoryAddress for $madr<Mutable> {}

        impl<S: State> SharedBlockControl for $bcr<S> {}

        impl BlockControl for $bcr<Mutable> {}

        impl<S: State> SharedChannelControl for $chcr<S> {}

        impl ChannelControl for $chcr<Mutable> {}
    };

    ([$madr:ident, $bcr:ident, $chcr:ident], $($others:tt)*) => {
        channel!([$madr, $bcr, $chcr]);
        channel!($($others)*);
    };
}

channel! {
    [D0_MADR, D0_BCR, D0_CHCR],
    [D1_MADR, D1_BCR, D1_CHCR],
    [D2_MADR, D2_BCR, D2_CHCR],
    [D3_MADR, D3_BCR, D3_CHCR],
    [D4_MADR, D4_BCR, D4_CHCR],
    [D5_MADR, D5_BCR, D5_CHCR],
    [D6_MADR, D6_BCR, D6_CHCR]
}