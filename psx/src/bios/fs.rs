use super::kernel;

// The BIOS file_seek from end is buggy
pub enum SeekFrom {
    Start(u32),
    Current(u32),
}

type Fd = i8;

/// A handle to a [BIOS file](http://problemkaputt.de/psx-spx.htm#biosfilefunctions).
#[derive(Debug)]
pub struct File {
    fd: Fd,
}

impl File {
    /// Memory card pathnames should be something like `"bu00:\\$NAME\0"` where
    /// `$NAME` is the actual filename.
    pub fn open(pathname: &str, flags: Option<u32>) -> Option<Self> {
        let flags = flags.unwrap_or(1);
        let fd = unsafe { kernel::file_open(pathname.as_ptr(), flags) };
        match fd {
            -1 => None,
            _ => Some(File { fd }),
        }
    }

    // TODO: does the BIOS return a value here?
    pub fn seek(&mut self, pos: SeekFrom) {
        let (offset, seek_ty) = match pos {
            SeekFrom::Start(offset) => (offset, 0),
            SeekFrom::Current(offset) => (offset, 1),
        };
        unsafe { kernel::file_seek(self.fd, offset, seek_ty) }
    }

    pub fn read(&self, dst: &mut [u8]) -> Option<usize> {
        let res = unsafe { kernel::file_read(self.fd, dst.as_mut_ptr(), dst.len()) };
        match res {
            -1 => None,
            _ => Some(res as u32 as usize),
        }
    }

    pub fn write(&mut self, src: &[u8]) -> Option<usize> {
        let res = unsafe { kernel::file_write(self.fd, src.as_ptr(), src.len()) };
        match res {
            -1 => None,
            _ => Some(res as u32 as usize),
        }
    }

    // The BIOS getc can't disambiguate between 0xFF and an error
    pub fn getc(&self) -> Option<u8> {
        let mut ret = [0; 1];
        self.read(&mut ret).map(|_| ret[0])
    }

    // Could use the BIOS putc here
    pub fn putc(&mut self, ch: u8) -> Option<usize> {
        self.write(&[ch])
    }

    pub fn close(self) {
        let _res = unsafe { kernel::file_close(self.fd) };
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _res = unsafe { kernel::file_close(self.fd) };
    }
}
