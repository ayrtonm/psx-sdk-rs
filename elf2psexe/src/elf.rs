use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use Section;
use SectionType;

pub struct ElfReader {
    elf: File,
    entry: u32,
    sections: Vec<Section>,
}

impl ElfReader {
    pub fn new(path: &Path) -> ElfReader {
        let elf =
            match OpenOptions::new().read(true).open(path) {
                Ok(elf) => elf,
                Err(e) => panic!("Can't open {}: {}", path.display(), e),
            };

        let mut reader = ElfReader {
            elf: elf,
            entry: 0,
            sections: Vec::new(),
        };

        reader.parse();

        reader
    }

    /// Parse ELF header and make sure it's a valid 32bit MIPS
    /// executable. Then parse all the sections.
    fn parse(&mut self) {
        // Read the ELF header. We're always expecting a 32bit executable
        // so the header should be 52bytes long
        let mut header = [0; 52];
        self.read(&mut header);

        if &header[..4] != b"\x7fELF" {
            panic!("Invalid ELF file: bad magic");
        }

        if header[4] != 1 {
            panic!("Invalid ELF file: not a 32bit object");
        }

        if header[5] != 1 {
            panic!("Invalid ELF file: not a little endian object");
        }

        if header[6] != 1 {
            panic!("Invalid ELF file: bad IDENT version");
        }

        if halfword(&header[16..]) != 2 {
               panic!("Invalid ELF file: not an executable");
        }

        if halfword(&header[18..]) != 8 {
            panic!("Invalid ELF file: not a MIPS executable");
        }

        if word(&header[20..]) != 1 {
            panic!("Invalid ELF file: bad object version");
        }

        self.entry = word(&header[24..]);

        let section_header_off = word(&header[32..]) as u64;
        let section_header_sz = halfword(&header[46..]) as u64;
        let section_count = halfword(&header[48..]) as u64;

        if section_header_sz < 40 {
            panic!("Invalid ELF file: bad section header size");
        }

        for s in 0..section_count {
            let offset = section_header_off + section_header_sz * s;

            if let Some(s) = self.parse_section(offset) {
                self.sections.push(s);
            }
        }

        // Make sure we have at least one ProgBits section
        if self.sections.iter().find(|s| {
            match s.contents {
                SectionType::ProgBits(_) => true,
                _ => false,
            }
        }).is_none() {
            panic!("No progbits section found");
        }
    }

    fn parse_section(&mut self, header_offset: u64) -> Option<Section> {
        self.seek(header_offset);

        // Read the section header
        let mut header = [0; 40];
        self.read(&mut header);

        let section_type = word(&header[4..]);
        let section_flags = word(&header[8..]);
        let section_addr = word(&header[12..]);
        let section_offset = word(&header[16..]) as u64;
        let section_size = word(&header[20..]);
        let section_align = word(&header[32..]);

        if section_align != 0 && section_addr % section_align != 0 {
            // I think it's not possible (unless the ELF is completely
            // broken) but I'd rather make sure
            panic!("bad section alignment");
        }

        // We only keep sections with the ALLOC attribute flag.
        if section_flags & 2 != 0 {
            match section_type {
                // Progbits
                1 => {
                    // This section contains data stored in the elf
                    // file.
                    let mut data = vec![0; section_size as usize];
                    self.seek(section_offset);
                    self.read(&mut data);

                    Some(Section {
                        base: section_addr,
                        contents: SectionType::ProgBits(data),
                    })
                }
                // Nobits
                8 => {
                    // This is a "BSS" type section: not present in
                    // the file but must be initialized to 0 by the
                    // loader.
                    Some(Section {
                        base: section_addr,
                        contents: SectionType::Memfill(section_size),
                    })
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn read(&mut self, buf: &mut [u8]) {
        match self.elf.read(buf) {
            Ok(n) => {
                if n != buf.len() {
                    panic!("Unexpected end of file");
                }
            }
            Err(e) => panic!("Read failed: {}", e),
        }
    }

    fn seek(&mut self, pos: u64) {
        match self.elf.seek(SeekFrom::Start(pos)) {
            Ok(n) => {
                if n != pos {
                    panic!("Unexpected end of file");
                }
            }
            Err(e) => panic!("Read failed: {}", e),
        }
    }

    pub fn entry(&self) -> u32 {
        self.entry
    }

    pub fn into_sections(self) -> Vec<Section> {
        self.sections
    }
}

/// Retreive a big endian 16bit integer
fn halfword(buf: &[u8]) -> u16 {
    (buf[0] as u16) | ((buf[1] as u16) << 8)
}

/// Retreive a big endian 32bit integer
fn word(buf: &[u8]) -> u32 {
    (halfword(buf) as u32) | ((halfword(&buf[2..]) as u32) << 16)
}
