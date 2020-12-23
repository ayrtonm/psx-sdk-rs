#![no_std]
#![no_main]
#![feature(naked_functions)]

use psx::dma;
use psx::dma::Channel;
use psx::value::LoadMut;

#[no_mangle]
fn main() {
    let mut ctrl = dma::Control;
    let mut_val = ctrl.load_mut();
    mut_val.value.enabled(Channel::SPU);
    let written_val = mut_val.enable(Channel::GPU)
        .enable(Channel::OTC)
        .store();
    written_val.enabled(Channel::OTC);
}
