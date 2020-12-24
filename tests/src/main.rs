#![no_std]
#![no_main]
#![feature(naked_functions)]

use psx::dma::control::Control;
use psx::dma::Channel;
use psx::value::LoadMut;

#[no_mangle]
fn main() {
    Control.load_mut().enable(Channel::GPU).store();
}
