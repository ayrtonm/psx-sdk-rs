use core::ffi::CStr;

pub fn get_system_date() -> u32 {
    get_system_info(0)
}

pub fn get_system_version() -> &'static CStr {
    unsafe { CStr::from_ptr(get_system_info(2) as *const i8) }
}

pub fn get_system_info(idx: u8) -> u32 {
    match idx {
        // Return the BIOS date in BCD
        0 => 0x20221004,
        // Return a pointer to the version string
        2 => "BIOS VERSION 0.1 by Ayrton\0".as_ptr() as u32,
        _ => 0xFFFFFFFF,
    }
}
