//! Filesystem manipulation operations
//!
//! This module contains basic methods to manipulate the contents of the
//! filesystem using BIOS functions. This module follows the general design of
//! `std::fs` in the standard library with PlayStation-specific deviations where
//! necessary. Only memory card files are currently supported.

#![warn(missing_docs)]

use super::kernel;
use crate::std::AsCStr;
use core::fmt;
use core::fmt::{Debug, Formatter};

/// Options and flags which can be used to configure how a file is opened.
///
/// This builder exposes the ability to configure how a [`File`] is opened and
/// what operations are permitted on the open file. The [`File::open`] and
/// [`File::create`] methods are aliases for commonly used options using this
/// builder.
///
/// Generally speaking, when using `OpenOptions`, you'll first call
/// [`OpenOptions::new`], then chain calls to methods to set each option, then
/// call [`OpenOptions::open`], passing the path of the file you're trying to
/// open. This will give you a [`Result`][core::result::Result] with a [`File`]
/// inside that you can further operate on.
#[derive(Default, Clone)]
pub struct OpenOptions {
    // These fields correspond to the accessmode bits
    create: bool,
    async_mode: bool,
    blocks: u16,

    // This field only affect the behavior of the high-level methods
    // If creating a new file fails because it already exists, open the existing file
    open_existing: bool,
}

impl OpenOptions {
    /// Creates a blank new set of options ready for configuration.
    ///
    /// All options are initially set to `false`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the option for asynchronous mode.
    ///
    /// This option, when true, will indicate that the BIOS should not wait for
    /// completion to return.
    pub fn async_mode(&mut self, mode: bool) -> &mut Self {
        self.async_mode = mode;
        self
    }

    /// Sets the option to create a new file, or open it if it already exists.
    ///
    /// If a new file is created, it will contain the specified number of memory
    /// card `blocks`.
    pub fn create(&mut self, blocks: u16) -> &mut Self {
        self.open_existing = true;
        self.create_new(blocks)
    }

    /// Sets the option to create a new file, failing if it already exists.
    ///
    /// The new file, if created, will contain the specified number of memory
    /// card `blocks`.
    pub fn create_new(&mut self, blocks: u16) -> &mut Self {
        self.create = true;
        self.blocks = blocks;
        self
    }

    /// Opens a file at `path` with the options specified by `self`.
    ///
    /// # Errors
    ///
    /// If the function fails the resulting `Err` will contain a type
    /// implementing [`FileError`]. Implementors of this trait include:
    /// * [`Error`]: The error code returned by calling into the BIOS
    ///   immediately after failing.
    /// * [`DeferredError`]: Allows calling into the BIOS to check the error at
    ///   a later point.
    /// * `()`: Skips calling into the BIOS, ignoring any possible error(s).
    // The lifetime on `E` doesn't actually matter since it's never a deferred file
    // error, but rust doesn't allow anonymous lifetimes there
    pub fn open<'a, E: FileError<'a>>(&self, path: &str) -> Result<File, E> {
        let flags = self.into();
        // Memcard directory frame limits filename to 20 chars, device name can
        // be up to 7 chars and 1 null-terminator gives 28 chars for `MAX_FILENAME`
        const MAX_FILENAME: usize = 28;

        let fd = path.as_cstr::<_, _, MAX_FILENAME>(|path| unsafe {
            kernel::file_open(path.as_ptr(), flags)
        });
        match fd {
            i8::MIN..=-2 => {
                illegal!("Received unknown error code from BIOS in `kernel::file_open`")
            },
            -1 => {
                // If we couldn't create a new file, open the existing file
                if self.open_existing {
                    // Make sure that we failed to create a new file because it already exists
                    let err = Error::get_error();
                    if let err = Error::FileAlreadyExists {
                        let mut opt = self.clone();
                        opt.create = false;
                        opt.open_existing = false;
                        opt.open(path)
                    } else {
                        // If we failed to create a new file for another reason, just give up
                        Err(E::get_error())
                    }
                } else {
                    Err(E::get_error())
                }
            },
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
    /// Memory card pathnames should be something like `"bu00:\\$NAME"` where
    /// `$NAME` is the actual filename.
    pub fn open(pathname: &str) -> Result<File, DeferredError> {
        OpenOptions::new().open(pathname)
    }

    pub fn create(pathname: &str) -> Result<File, DeferredError> {
        OpenOptions::new().create_new(1).open(pathname)
    }

    pub fn seek<E: FileError<'f>>(&'f mut self, pos: SeekFrom) -> Result<usize, E> {
        let (offset, seek_ty) = match pos {
            SeekFrom::Start(offset) => (offset, 0),
            SeekFrom::Current(offset) => (offset, 1),
        };
        let res = unsafe { kernel::file_seek(self.fd, offset, seek_ty) };
        match res {
            i32::MIN..=-2 => {
                illegal!("Received unknown error code from BIOS in `kernel::file_seek`")
            },
            -1 => Err(E::get_file_error(self)),
            _ => Ok(res as usize),
        }
    }

    pub fn read<E: FileError<'f>>(&'f self, dst: &mut [[u8; 128]]) -> Result<usize, E> {
        let res = unsafe { kernel::file_read(self.fd, dst.as_mut_ptr().cast(), dst.len() * 128) };
        match res {
            i32::MIN..=-2 => {
                illegal!("Received unknown error code from BIOS in `kernel::file_read`")
            },
            -1 => Err(E::get_file_error(self)),
            _ => Ok(res as usize),
        }
    }

    pub fn write<E: FileError<'f>>(&'f mut self, src: &[[u8; 128]]) -> Result<usize, E> {
        let src = src.as_ref();
        let res = unsafe { kernel::file_write(self.fd, src.as_ptr().cast(), src.len() * 128) };
        match res {
            i32::MIN..=-2 => {
                illegal!("Received unknown error code from BIOS in `kernel::file_write`")
            },
            -1 => Err(E::get_file_error(self)),
            _ => Ok(res as usize),
        }
    }

    // The BIOS getc can't disambiguate between 0xFF and an error
    pub fn getc<E: FileError<'f>>(&'f self) -> Result<u8, E> {
        let ret = [0; 128];
        self.read(&mut [ret]).map(|_| ret[0])
    }

    // Could use the BIOS putc here
    pub fn putc<E: FileError<'f>>(&'f mut self, ch: u8) -> Result<usize, E> {
        let mut temp = [0; 128];
        temp[0] = ch;
        self.write(&[temp])
    }

    pub fn close<'a, E: FileError<'a>>(self) -> Result<Fd, E> {
        let res = unsafe { kernel::file_close(self.fd) };
        match res {
            i8::MIN..=-2 => {
                illegal!("Received unknown error code from BIOS in `kernel::file_close`")
            },
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
