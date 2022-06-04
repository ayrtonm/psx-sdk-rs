#![no_std]
#![no_main]

use psx::printf;
use psx::println;
use psx::sys::fs::{File, FileTy, CDROM};

#[no_mangle]
fn main() {
    // BIOS `File`s are parameterized by the file type (CDROM or MemCard). Only
    // MemCard files allow writing so their file type is usually inferred, but
    // CD-ROM files always require specifying the file type.
    let file = File::<CDROM>::open("cdrom:\\FILE.TXT").expect("Could not find FILE.TXT");
    let mut txt = [0; CDROM::SECTOR_SIZE];
    file.read(&mut txt).unwrap();
    printf!("%s\n", &mut txt);
}
