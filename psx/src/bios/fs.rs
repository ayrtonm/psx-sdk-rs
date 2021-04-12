use super::kernel;
use core::fmt;
use core::fmt::{Debug, Formatter};

#[derive(Default)]
pub struct OpenOptions {
    create: bool,
    async_mode: bool,
    blocks: u16,
}

impl OpenOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn async_mode(&mut self, mode: bool) -> &mut Self {
        self.async_mode = mode;
        self
    }

    pub fn blocks(&mut self, blocks: u16) -> &mut Self {
        self.blocks = blocks;
        self
    }

    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    // The lifetime on `E` doesn't actually matter since it's never a deferred file
    // error, but rust doesn't allow anonymous lifetimes there
    pub fn open<'a, E: FileError<'a>>(&self, pathname: &str) -> Result<File, E> {
        let flags = self.into();
        let fd = unsafe { kernel::file_open(pathname.as_ptr(), flags) };
        match fd {
            // This uses the generic error function since an error would not have a valid file
            // descriptor
            -1 => Err(E::get_error()),
            _ => Ok(File { fd }),
        }
    }
}

impl From<&OpenOptions> for u32 {
    fn from(opt: &OpenOptions) -> u32 {
        // Bits 0-1 (read/write) aren't used by the BIOS, but at least 1 should be set
        1 | ((opt.create as u32) << 9) |
            ((opt.async_mode as u32) << 15) |
            ((opt.blocks as u32) << 16)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Error {
    NoError,
    FileNotFound,
    BadDevPort,
    InvalidFileHandle,
    PhysicalError,
    FileAlreadyExists,
    AttemptedToMoveDevices,
    UnknownDevice,
    SectorAlignmentError,
    NoFreeHandles,
    NoFreeBlocks,
    UnknownError,
}

impl From<u32> for Error {
    fn from(err: u32) -> Self {
        match err {
            0x00 => Error::NoError,
            0x02 => Error::FileNotFound,
            0x06 => Error::BadDevPort,
            0x09 | 0xFFFF_FFFF => Error::InvalidFileHandle,
            0x10 => Error::PhysicalError,
            0x11 => Error::FileAlreadyExists,
            0x12 => Error::AttemptedToMoveDevices,
            0x13 => Error::UnknownDevice,
            0x16 => Error::SectorAlignmentError,
            0x18 => Error::NoFreeHandles,
            0x1C => Error::NoFreeBlocks,
            _ => Error::UnknownError,
        }
    }
}

pub enum DeferredError<'f> {
    #[doc(hidden)]
    FdError(fn(&'f File) -> Error, &'f File),
    #[doc(hidden)]
    Error(fn() -> Error),
}

impl<'f> Debug for DeferredError<'f> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.error().fmt(f)
    }
}

pub trait FileError<'f> {
    #[doc(hidden)]
    fn get_file_error(file: &'f File) -> Self;
    #[doc(hidden)]
    fn get_error() -> Self;
    fn error(&self) -> Error;
}

impl FileError<'_> for () {
    fn get_file_error(_file: &File) -> Self {}
    fn get_error() -> Self {}
    fn error(&self) -> Error {
        Error::UnknownError
    }
}

impl FileError<'_> for Error {
    fn get_file_error(file: &File) -> Self {
        let err = unsafe { kernel::get_last_file_error(file.fd) };
        err.into()
    }

    fn get_error() -> Self {
        let err = unsafe { kernel::get_last_error() };
        err.into()
    }

    fn error(&self) -> Error {
        *self
    }
}

impl<'f> FileError<'f> for DeferredError<'f> {
    fn get_file_error(file: &'f File) -> Self {
        DeferredError::FdError(
            |file| {
                let err = unsafe { kernel::get_last_file_error(file.fd) };
                err.into()
            },
            file,
        )
    }

    fn get_error() -> Self {
        DeferredError::Error(|| {
            let err = unsafe { kernel::get_last_error() };
            err.into()
        })
    }

    fn error(&self) -> Error {
        match self {
            DeferredError::FdError(func, arg) => func(arg),
            DeferredError::Error(func) => func(),
        }
    }
}

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

impl<'f> File {
    /// Memory card pathnames should be something like `"bu00:\\$NAME\0"` where
    /// `$NAME` is the actual filename.
    pub fn open(pathname: &str) -> Result<File, DeferredError> {
        OpenOptions::new().create(false).open(pathname)
    }

    pub fn create(pathname: &str) -> Result<File, DeferredError> {
        OpenOptions::new().create(true).open(pathname)
    }

    pub fn seek<E: FileError<'f>>(&'f mut self, pos: SeekFrom) -> Result<i32, E> {
        let (offset, seek_ty) = match pos {
            SeekFrom::Start(offset) => (offset, 0),
            SeekFrom::Current(offset) => (offset, 1),
        };
        // TODO: file_seek currently returns a i32 since that's a reasonable default,
        // but I should check what the result actually represents
        let res = unsafe { kernel::file_seek(self.fd, offset, seek_ty) };
        match res {
            -1 => Err(E::get_file_error(self)),
            _ => Ok(res),
        }
    }

    pub fn read<E: FileError<'f>>(&'f self, dst: &mut [u8]) -> Result<usize, E> {
        let res = unsafe { kernel::file_read(self.fd, dst.as_mut_ptr(), dst.len()) };
        match res {
            -1 => Err(E::get_file_error(self)),
            _ => Ok(res as u32 as usize),
        }
    }

    pub fn write<E: FileError<'f>>(&'f mut self, src: &[u8]) -> Result<usize, E> {
        let res = unsafe { kernel::file_write(self.fd, src.as_ptr(), src.len()) };
        match res {
            -1 => Err(E::get_file_error(self)),
            _ => Ok(res as u32 as usize),
        }
    }

    // The BIOS getc can't disambiguate between 0xFF and an error
    pub fn getc<E: FileError<'f>>(&'f self) -> Result<u8, E> {
        let mut ret = [0; 1];
        self.read(&mut ret).map(|_| ret[0])
    }

    // Could use the BIOS putc here
    pub fn putc<E: FileError<'f>>(&'f mut self, ch: u8) -> Result<usize, E> {
        self.write(&[ch])
    }

    pub fn close<'a, E: FileError<'a>>(self) -> Result<Fd, E> {
        let res = unsafe { kernel::file_close(self.fd) };
        match res {
            // Does get_file_error make sense here?
            -1 => Err(E::get_error()),
            _ => Ok(res),
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _res = unsafe { kernel::file_close(self.fd) };
    }
}
