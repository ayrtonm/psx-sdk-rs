#![no_std]
#![no_main]
#![feature(naked_functions)]

use psx::dma;
use psx::dma::Channel;

#[no_mangle]
fn main() {
    dma::Control.load_mut()
        .enable(Channel::GPU)
        .enable(Channel::OTC)
        .store();
}
