//! Filesystem manipulation operations
//!
//! This module contains basic methods to manipulate the contents of the
//! filesystem using BIOS functions. This module follows the general design of
//! `std::fs` in the standard library with PlayStation-specific deviations where
//! necessary. Only memory card files are currently supported.

#![deny(missing_docs)]

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
    /// card `blocks`. Note that if a new file is not created, this option will
    /// immediately call into the BIOS to ensure the failure occurred because
    /// the file already exists. The final [`Result`][core::result::Result] then
    /// contains the specified implementor of [`FileError`].
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
    /// * [`ErrorKind`]: The error code returned by calling into the BIOS
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
                    if let ErrorKind::FileAlreadyExists = ErrorKind::new_error() {
                        let mut opt = self.clone();
                        opt.create = false;
                        opt.open_existing = false;
                        opt.open(path)
                    } else {
                        // If we failed to create a new file for another reason, just give up
                        Err(E::new_error())
                    }
                } else {
                    Err(E::new_error())
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

/// A list specifying I/O error codes returned by the BIOS.
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    /// Ok (though many successful functions leave old error codes unchanged)
    NoError,
    /// File not found
    FileNotFound,
    /// Bad device port number (tty2 and up)
    BadDevPort,
    /// Invalid or unused file handle
    InvalidFileHandle,
    /// General error (physical I/O error, unformatted device)
    PhysicalError,
    /// File already exists
    FileAlreadyExists,
    /// Tried to rename a file from one device to another device
    AttemptedToMoveDevices,
    /// Unknown device name
    UnknownDevice,
    /// Sector alignment error or file pointer outside file bounds
    SectorAlignmentError,
    /// Not enough free file handles
    NoFreeHandles,
    /// Not enough free memory card blocks
    NoFreeBlocks,
    /// Unknown error code
    UnknownError,
}

impl From<u32> for ErrorKind {
    fn from(err: u32) -> Self {
        match err {
            0x00 => ErrorKind::NoError,
            0x02 => ErrorKind::FileNotFound,
            0x06 => ErrorKind::BadDevPort,
            0x09 | 0xFFFF_FFFF => ErrorKind::InvalidFileHandle,
            0x10 => ErrorKind::PhysicalError,
            0x11 => ErrorKind::FileAlreadyExists,
            0x12 => ErrorKind::AttemptedToMoveDevices,
            0x13 => ErrorKind::UnknownDevice,
            0x16 => ErrorKind::SectorAlignmentError,
            0x18 => ErrorKind::NoFreeHandles,
            0x1C => ErrorKind::NoFreeBlocks,
            _ => ErrorKind::UnknownError,
        }
    }
}

/// Implements [`FileError`] to allow calling into the BIOS at any point to
/// check the last error.
///
/// Uses the [`FileError::error`] method to get the error code from the BIOS.
/// The associated lifetime `'f` corresponds to the corresponding file
/// descriptor if any.
pub enum DeferredError<'f> {
    #[doc(hidden)]
    FdErrorKind(fn(&'f File) -> ErrorKind, &'f File),
    #[doc(hidden)]
    ErrorKind(fn() -> ErrorKind),
}

impl<'f> Debug for DeferredError<'f> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.error().fmt(f)
    }
}

/// Allows different ways of handling filesystem errors from the BIOS.
pub trait FileError<'f> {
    #[doc(hidden)]
    fn new_file_error(file: &'f File) -> Self;
    #[doc(hidden)]
    fn new_error() -> Self;

    /// Returns a concrete [`ErrorKind`] code. Not guaranteed to call into the
    /// BIOS.
    fn error(&self) -> ErrorKind;
}

impl FileError<'_> for () {
    fn new_file_error(_file: &File) -> Self {}
    fn new_error() -> Self {}
    fn error(&self) -> ErrorKind {
        ErrorKind::UnknownError
    }
}

impl FileError<'_> for ErrorKind {
    fn new_file_error(file: &File) -> Self {
        let err = unsafe { kernel::get_last_file_error(file.fd) };
        err.into()
    }

    fn new_error() -> Self {
        let err = unsafe { kernel::get_last_error() };
        err.into()
    }

    fn error(&self) -> ErrorKind {
        *self
    }
}

impl<'f> FileError<'f> for DeferredError<'f> {
    fn new_file_error(file: &'f File) -> Self {
        DeferredError::FdErrorKind(
            |file| {
                let err = unsafe { kernel::get_last_file_error(file.fd) };
                err.into()
            },
            file,
        )
    }

    fn new_error() -> Self {
        DeferredError::ErrorKind(|| {
            let err = unsafe { kernel::get_last_error() };
            err.into()
        })
    }

    fn error(&self) -> ErrorKind {
        match self {
            DeferredError::FdErrorKind(func, arg) => func(arg),
            DeferredError::ErrorKind(func) => func(),
        }
    }
}

/// Enumeration of possible methods to seek within a file.
///
/// Does not include from end as the BIOS seek function is buggy.
pub enum SeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(u32),
    /// Sets the offset to the current position plus the specified number of
    /// bytes.
    Current(u32),
}

type Fd = i8;

/// A reference to an open [BIOS file](http://problemkaputt.de/psx-spx.htm#biosfilefunctions).
///
/// Files are automatically closed when they go out of scope. Errors detected on
/// closing are ignored by the implementation of `Drop`.
#[derive(Debug)]
pub struct File {
    fd: Fd,
}

impl<'f> File {
    /// Attempts to open a file.
    ///
    /// Memory card paths should be formatted as `"bu00:\\FILE_NAME"` where
    /// `bu00` is the [device name](http://problemkaputt.de/psx-spx.htm#controllerandmemorycardmisc) and `FILE_NAME`
    /// is the [file name](http://problemkaputt.de/psx-spx.htm#memorycarddataformat).
    ///
    /// See the [`OpenOptions::open`] function for more details.
    pub fn open(path: &str) -> Result<File, DeferredError> {
        OpenOptions::new().open(path)
    }

    /// Attempts to create a file.
    ///
    /// This function will open the file if it already exists.
    ///
    /// See the [`OpenOptions::open`] function for more details.
    pub fn create(path: &str) -> Result<File, DeferredError> {
        OpenOptions::new().create_new(1).open(path)
    }

    /// Seeks to an offset, in bytes, in a file.
    ///
    /// If the seek operation is successful, this method returns the new
    /// position from the start of the file. That position can be used later
    /// with [`SeekFrom::Start`].
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
            -1 => Err(E::new_file_error(self)),
            _ => Ok(res as usize),
        }
    }

    /// Reads some bytes from the file into the specified `dst`, returning how
    /// many bytes were read.
    ///
    /// Memory card files can only be read in increments of 128 bytes.
    pub fn read<E: FileError<'f>>(&'f self, dst: &mut [[u8; 128]]) -> Result<usize, E> {
        let res = unsafe { kernel::file_read(self.fd, dst.as_mut_ptr().cast(), dst.len() * 128) };
        match res {
            i32::MIN..=-2 => {
                illegal!("Received unknown error code from BIOS in `kernel::file_read`")
            },
            -1 => Err(E::new_file_error(self)),
            _ => Ok(res as usize),
        }
    }

    /// Writes some bytes to the file from the specified `src`, returning how
    /// many bytes were written.
    ///
    /// Memory card files can only be written to in increments of 128 bytes.
    pub fn write<E: FileError<'f>>(&'f mut self, src: &[[u8; 128]]) -> Result<usize, E> {
        let src = src.as_ref();
        let res = unsafe { kernel::file_write(self.fd, src.as_ptr().cast(), src.len() * 128) };
        match res {
            i32::MIN..=-2 => {
                illegal!("Received unknown error code from BIOS in `kernel::file_write`")
            },
            -1 => Err(E::new_file_error(self)),
            _ => Ok(res as usize),
        }
    }

    /// Reads a byte from the file.
    ///
    /// This function internally uses [`File::read`] since the BIOS `getc`
    /// [can't disambiguate between an error code](http://problemkaputt.de/psx-spx.htm#biosfilefunctions)
    /// and a return value of `0xFF`.
    pub fn getc<E: FileError<'f>>(&'f self) -> Result<u8, E> {
        let ret = [0; 128];
        self.read(&mut [ret]).map(|_| ret[0])
    }

    /// Writes a byte to the file.
    pub fn putc<E: FileError<'f>>(&'f mut self, ch: u8) -> Result<usize, E> {
        let mut temp = [0; 128];
        temp[0] = ch;
        self.write(&[temp])
    }

    /// Manually closes the file, returning a possible BIOS error code.
    pub fn close<'a, E: FileError<'a>>(self) -> Result<Fd, E> {
        let res = unsafe { kernel::file_close(self.fd) };
        match res {
            i8::MIN..=-2 => {
                illegal!("Received unknown error code from BIOS in `kernel::file_close`")
            },
            // Does new_file_error make sense here?
            -1 => Err(E::new_error()),
            _ => Ok(res),
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _res = unsafe { kernel::file_close(self.fd) };
    }
}
