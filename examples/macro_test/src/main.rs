#![no_std]
#![no_main]
#![feature(once_cell)]

psx::exe!();
use psx::unzip;

fn main(mut _io: IO) {
    let ar = unzip!("../ferris.tim.zip");
    *ar;
}
