use crate::iso9660::ISO;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct ISOFile {
    pub name: Vec<u8>,
    pub contents: Vec<u8>,
}

pub struct ISODir {
    pub name: Vec<u8>,
    pub files: Vec<ISOFile>,
    pub directories: Vec<ISODir>,
}

const DEFAULT_SYS_CNF: &'static [u8] = b"BOOT = cdrom:\\PROGRAM.EXE;1\x0d\x0a\
TCB = 4\x0d\x0a\
EVENT = 16\x0d\x0a\
STACK = 801FFFF0";

pub fn create_default_iso(exe: PathBuf) {
    let mut iso_name = exe.clone();
    iso_name.set_extension("iso");

    let mut exe_data = Vec::new();
    File::open(exe)
        .expect("Could not find executable")
        .read_to_end(&mut exe_data)
        .expect("Could not read executable");

    let sys_cnf = ISOFile {
        name: "SYSTEM.CNF".into(),
        contents: DEFAULT_SYS_CNF.into(),
    };
    let program = ISOFile {
        name: "PROGRAM.EXE".into(),
        contents: exe_data,
    };
    let root = ISODir {
        name: "".into(),
        files: vec![sys_cnf, program],
        directories: Vec::new(),
    };
    let mut iso = ISO::new();
    iso.set_root(root);

    let mut output = File::create(iso_name).expect("Could not create ISO");
    output
        .write_all(iso.serialize())
        .expect("Could not write to ISO");
}
