pub fn get_system_info(idx: u8) -> u32 {
    match idx {
        // Return the date in BCD
        0 => 0x20221004,
        // Return a pointer to the version string
        2 => "BIOS VERSION 0.1\0".as_ptr() as u32,
        _ => 0xFFFFFFFF,
    }
}
