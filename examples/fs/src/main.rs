#![no_std]
#![no_main]

use core::str::from_utf8_unchecked;
use psx::bios::fs::{File, FileTy, SeekFrom, CDROM};
use psx::bios::kernel;
use psx::println;

#[no_mangle]
fn main() {
    unsafe {
        kernel::set_default_exit_from_exception();
        kernel::init_card(true);
        kernel::start_card();
    }
    let file_name = "cdrom:\\TEST";
    let file = File::<CDROM>::open(file_name).unwrap_or_else(|err| {
        println!("{:?}", err);
        panic!("file not found")
    });
    let mut buf = [0; CDROM::SECTOR_SIZE];
    println!("{:?}", file.seek(SeekFrom::Start(0)));
    println!("{:?}", file.read(&mut buf));
    unsafe {
        println!("{}", from_utf8_unchecked(&buf));
        kernel::stop_card();
    }
}
