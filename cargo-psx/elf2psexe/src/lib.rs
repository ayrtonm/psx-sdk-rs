use std::path::Path;

mod elf;
mod psexe;

pub struct Section {
    base: u32,
    contents: SectionType,
}

enum SectionType {
    /// The section's data is contained in the file
    ProgBits(Vec<u8>),
    /// BSS data that's set to 0 by the loader (not contained in the
    /// file). There can be only one contiguous Memfill resion in an
    /// EXE file.
    Memfill(u32),
}

#[derive(Clone, Copy)]
pub enum Region {
    NorthAmerica,
    Europe,
    Japan,
}

impl Region {
    fn from_str(s: &str) -> Region {
        match s {
            "NA" => Region::NorthAmerica,
            "EU"  => Region::Europe,
            "JP"  => Region::Japan,
            _    => panic!("Invalid region {}", s)
        }
    }
}

pub fn main(args: Vec<&str>, no_pad: bool) {

    if args.len() < 3 {
        println!("usage: elf2psexe <REGION> <elf-bin> <psx-bin>");
        println!("Valid regions: NA, EU or JP");
        panic!("Missing argument");
    }

    let region = Region::from_str(&args[0]);
    let elfpath = &args[1];
    let psexepath = &args[2];

    let elf = elf::ElfReader::new(Path::new(elfpath));

    let entry = elf.entry();
    let sections = elf.into_sections();

    let psexe = psexe::PsxWriter::new(Path::new(psexepath), region);

    psexe.dump(entry, sections, no_pad);
}
