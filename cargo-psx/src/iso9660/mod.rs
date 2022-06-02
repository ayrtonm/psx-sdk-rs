#![allow(dead_code)]
use crate::package::ISODir;
use std::mem::size_of;
use std::slice;
use str_a::{CharSetA, StrA, VarStrA};
use str_d::{StrD, VarStrD};
use uint::{U16, U32, U32LSB, U32MSB};

mod str_a;
mod str_d;
mod uint;

const SECTOR_SIZE: usize = 2352;
const KB: usize = 1024;
const MB: usize = 1024 * KB;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Unused<const N: usize>([u8; N]);

impl<const N: usize> Default for Unused<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
struct DateTime {
    year: StrD<4>,
    month: StrD<2>,
    day: StrD<2>,
    hour: StrD<2>,
    minute: StrD<2>,
    second: StrD<2>,
    centisecond: StrD<2>,
    time_zone: u8,
}

#[repr(packed)]
#[derive(Default, Debug, Clone, Copy)]
pub struct PrimaryVolumeDescriptor {
    type_code: u8,
    id: StrA<5>,
    version: u8,
    _unused0: u8,
    sys_id: StrA<32>,
    vol_id: StrD<32>,
    _unused1: Unused<8>,
    vol_space_size: U32,
    _unused2: Unused<32>,
    vol_set_size: U16,
    vol_seq_num: U16,
    blk_size: U16,
    path_table_size: U32,
    type_l_path_table: U32LSB,
    opt_type_l_path_table: U32LSB,
    type_m_path_table: U32MSB,
    opt_type_m_path_table: U32MSB,
    root_dir: DirEntry<StrA<34>>,
    vol_set_id: StrD<128>,
    pub_id: StrA<128>,
    preparer_id: StrA<128>,
    app_id: StrA<128>,
    copyright_file_id: StrD<128>,
    abstract_file_id: StrD<128>,
    bibliographic_file_id: StrD<128>,
    created: DateTime,
    modified: DateTime,
    expiration: DateTime,
    effective: DateTime,
    file_structure_version: u8,
    _unused3: u8,
    app_specific0: Unused<141>,
    xa_signature: StrA<8>,
    xa_flags: Unused<2>,
    xa_startup_dir: Unused<8>,
    xa_reserved: Unused<8>,
    app_specific1: Unused<345>,
    reserved: Unused<653>,
    raw_reserved: Unused<304>,
}

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct VolumeDescriptorSetTerminator {
    type_code: u8,
    id: StrA<5>,
    version: u8,
    _unused: Unused<2041>,
    _raw_unused: Unused<304>,
}

impl VolumeDescriptorSetTerminator {
    pub fn new() -> Self {
        Self {
            type_code: 0xff,
            id: StrA::new(b"CD001"),
            version: 1,
            _unused: Default::default(),
            _raw_unused: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct PathTableEntry<E> {
    len: u8,
    xa_len: u8,
    extent: E,
    dir_num: u16,
    dir_id: VarStrD,
}

impl PathTableEntry<U32LSB> {
    pub fn new(dir_name: &Vec<u8>, dir_num: u16) -> Self {
        Self {
            len: dir_name.len().try_into().unwrap(),
            // TODO:
            xa_len: 0,
            // TODO:
            extent: 0.into(),
            dir_num,
            dir_id: VarStrD::new(dir_name),
        }
    }
}

impl<const N: usize> Copy for DirEntry<StrA<N>> {}
impl<const N: usize> Default for DirEntry<StrA<N>> {
    fn default() -> Self {
        Self::new(b"")
    }
}

#[derive(Debug, Clone)]
struct DirEntry<T: CharSetA> {
    len: u8,
    xa_len: u8,
    extent: [u8; 8],
    data_len: [u8; 8],
    date_time: [u8; 7],
    flags: u8,
    _unused0: u8,
    _unused1: u8,
    vol_seq_num: u32,
    file_name_len: u8,
    file_id: T,
}

impl<const N: usize> DirEntry<StrA<N>> {
    pub fn new(file_name: &[u8]) -> Self {
        Self {
            len: file_name.len().try_into().unwrap(),
            // TODO:
            xa_len: 0,
            extent: [0; 8],
            data_len: [0; 8],
            date_time: [0; 7],
            flags: 0,
            _unused0: 0,
            _unused1: 0,
            vol_seq_num: 0,
            file_name_len: 0,
            file_id: StrA::new(file_name),
        }
    }
}
impl DirEntry<VarStrA> {
    pub fn new(file_name: &Vec<u8>) -> Self {
        Self {
            len: file_name.len().try_into().unwrap(),
            // TODO:
            xa_len: 0,
            extent: [0; 8],
            data_len: [0; 8],
            date_time: [0; 7],
            flags: 0,
            _unused0: 0,
            _unused1: 0,
            vol_seq_num: 0,
            file_name_len: 0,
            file_id: VarStrA::new(file_name),
        }
    }
}

#[repr(packed)]
#[derive(Debug)]
pub struct Sector([u8; SECTOR_SIZE]);

impl Default for Sector {
    fn default() -> Self {
        Self([0; SECTOR_SIZE])
    }
}

#[repr(packed)]
#[derive(Debug)]
pub struct ISO {
    system_area: [Sector; 16],
    pvd: PrimaryVolumeDescriptor,
    term: VolumeDescriptorSetTerminator,
    //sectors: Vec<Sector>
}

impl ISO {
    pub fn new() -> Self {
        Self {
            system_area: Default::default(),
            pvd: PrimaryVolumeDescriptor::new(),
            term: VolumeDescriptorSetTerminator::new(),
        }
    }

    pub fn serialize(&self) -> &[u8] {
        // SAFETY: `ISO` is currently a flat struct
        unsafe { slice::from_raw_parts(self as *const ISO as *const u8, size_of::<ISO>()) }
    }

    pub fn set_root(&mut self, root: ISODir) {
        #[derive(Debug)]
        struct Context {
            path_table: Vec<PathTableEntry<U32LSB>>,
        }
        let path_table = Vec::new();
        let mut ctxt = Context { path_table };
        fn add_dir(dir: &ISODir, ctxt: &mut Context) {
            for f in &dir.files {}
            for d in &dir.directories {
                let parent = 0;
                ctxt.path_table.push(PathTableEntry::new(&d.name, parent));
                add_dir(d, ctxt);
            }
        }
        add_dir(&root, &mut ctxt);
        println!("{:?}", ctxt);
    }
}

impl PrimaryVolumeDescriptor {
    pub fn new() -> Self {
        Self {
            type_code: 1,
            id: StrA::new(b"CD001"),
            version: 1,
            sys_id: StrA::new(b"PLAYSTATION"),
            vol_id: StrD::new(b"SCUS_12345"),
            vol_space_size: 0x000419c7.into(),
            vol_set_size: 1.into(),
            vol_seq_num: 1.into(),
            blk_size: 0x800.into(),
            path_table_size: 0x32.into(),
            type_l_path_table: 0x12.into(),
            opt_type_l_path_table: 0x13.into(),
            type_m_path_table: 0x14.into(),
            opt_type_m_path_table: 0x15.into(),
            ..Default::default()
        }
    }
}
