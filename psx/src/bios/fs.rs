//! Filesystem manipulation operations
//!
//! This module contains basic methods to manipulate the contents of the
//! filesystem using BIOS functions. This module follows the general design of
//! `std::fs` in the standard library with PlayStation-specific deviations where
//! necessary. The BIOS internally uses C-style null-terminated strings for path
//! names, but all methods in this module support string slices. Omitting
//! null-terminators incurs the runtime cost of copying the path into a
//! temporary buffer however. Only memory card files are currently supported.

#![deny(missing_docs)]

use super::kernel;
use crate::std::AsCStr;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::mem::forget;

// Memcard directory frame limits filename to 20 chars, device name can
// be up to 7 chars and 1 null-terminator gives 28 chars for `MAX_FILENAME`
const MAX_FILENAME: usize = 28;

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
    /// card 8 kB `blocks`. Note that if a new file is not created, this option
    /// will immediately call into the BIOS to ensure the failure occurred
    /// because the file already exists. The final
    /// [`Result`][core::result::Result] then contains the specified
    /// implementor of [`FileError`].
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
    pub fn open<'a, 'f, E: FileError<'a>>(&self, path: &'f str) -> Result<File<'f>, E> {
        let flags = self.into();

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
            _ => {
                let fd = fd as Fd;
                Ok(File { fd, path })
            },
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    FdErrorKind(fn(&'f File<'f>) -> ErrorKind, &'f File<'f>),
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
    Current(i32),
}

type Fd = u8;

/// A reference to an open [BIOS file](http://problemkaputt.de/psx-spx.htm#biosfilefunctions).
///
/// Files are automatically closed when they go out of scope. Errors detected on
/// closing are ignored by the implementation of `Drop`.
#[derive(Debug)]
pub struct File<'f> {
    fd: Fd,
    path: &'f str,
}

impl<'f> File<'f> {
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

    /// Attempts to create a file with the specified `size` in bytes.
    ///
    /// This function will open the file if it already exists. The specified
    /// size is rounded down to a multiple of 8 kB or set to 8 kB if
    /// unspecified.
    ///
    /// See the [`OpenOptions::open`] function for more details.
    pub fn create(path: &str, size: Option<u16>) -> Result<File, DeferredError> {
        let size = size.unwrap_or(8 * 1024) >> 13;
        OpenOptions::new().create_new(size).open(path)
    }

    /// Seeks to an offset, in bytes, in a file.
    ///
    /// If the seek operation is successful, this method returns the new
    /// position from the start of the file. That position can be used later
    /// with [`SeekFrom::Start`].
    pub fn seek<'a, E>(&'a self, pos: SeekFrom) -> Result<usize, E>
    where E: FileError<'a> {
        let (offset, seek_ty) = match pos {
            SeekFrom::Start(offset) => (offset, 0),
            SeekFrom::Current(offset) => (offset as u32, 1),
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
    pub fn read<'a, E>(&'a self, dst: &mut [[u8; 128]]) -> Result<usize, E>
    where E: FileError<'a> {
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
    pub fn write<'a, E>(&'a mut self, src: &[[u8; 128]]) -> Result<usize, E>
    where E: FileError<'a> {
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
    /// This function internally uses [`File::read`] since the
    /// [BIOS `getc`](http://problemkaputt.de/psx-spx.htm#biosfilefunctions)
    /// can't disambiguate between an error code and a return value of `0xFF`.
    pub fn getc<'a, E>(&'a self) -> Result<u8, E>
    where E: FileError<'a> {
        let ret = [0; 128];
        self.read(&mut [ret]).map(|_| ret[0])
    }

    /// Writes a byte to the file.
    pub fn putc<'a, E>(&'a mut self, ch: u8) -> Result<usize, E>
    where E: FileError<'a> {
        let mut temp = [0; 128];
        temp[0] = ch;
        self.write(&[temp])
    }

    /// Manually closes the file, returning a possible BIOS error code.
    pub fn close<'a, E>(self) -> Result<Fd, E>
    where E: FileError<'a> {
        let res = unsafe { kernel::file_close(self.fd) };
        forget(self);
        match res {
            i8::MIN..=-2 => {
                illegal!("Received unknown error code from BIOS in `kernel::file_close`")
            },
            -1 => Err(E::new_error()),
            _ => Ok(res as Fd),
        }
    }

    /// Renames a file.
    pub fn rename<'a, E>(&'a mut self, new_name: &'f str) -> Result<(), E>
    where E: FileError<'a> {
        let res = self.path.as_cstr::<_, _, MAX_FILENAME>(|old| {
            new_name.as_cstr::<_, _, MAX_FILENAME>(|new| unsafe {
                kernel::file_rename(old.as_ptr(), new.as_ptr())
            })
        });
        match res {
            1 => {
                self.path = new_name;
                Ok(())
            },
            0 => Err(E::new_error()),
            _ => illegal!("Received unknown error code from BIOS in `kernel::file_rename`"),
        }
    }

    /// Deletes the file.
    ///
    /// The deleted file references must still be closed either by going out of
    /// scope or with [`File::close`].
    pub fn delete<'a, E>(&mut self) -> Result<(), E>
    where E: FileError<'a> {
        let res = self
            .path
            .as_cstr::<_, _, MAX_FILENAME>(|path| unsafe { kernel::file_delete(path.as_ptr()) });
        match res {
            1 => Ok(()),
            0 => Err(E::new_error()),
            _ => illegal!("Received unknown error code from BIOS in `kernel::file_delete`"),
        }
    }

    /// Recovers a deleted file.
    pub fn recover<'a, E>(&mut self) -> Result<(), E>
    where E: FileError<'a> {
        let res = self
            .path
            .as_cstr::<_, _, MAX_FILENAME>(|path| unsafe { kernel::file_undelete(path.as_ptr()) });
        match res {
            1 => Ok(()),
            0 => Err(E::new_error()),
            _ => illegal!("Received unknown error code from BIOS in `kernel::file_undelete`"),
        }
    }
}

impl Drop for File<'_> {
    fn drop(&mut self) {
        let _res = unsafe { kernel::file_close(self.fd) };
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrorKind, File, FileError, OpenOptions, SeekFrom};

    // TODO: Remove this eventually. This is a temporary solution while I work on
    // wrappers for these kernel functions.
    fn with_card<F: FnOnce() -> R, R>(f: F) -> R {
        use crate::bios::kernel;
        unsafe {
            kernel::init_card(true);
            kernel::start_card();
        }
        let res = f();
        unsafe {
            kernel::stop_card();
        }
        res
    }

    #[test_case]
    fn open_create_close() {
        with_card(|| {
            let path = "bu00:\\DoesNotExist";
            // Opening a non-existent file should not work
            let file = File::open(path);
            let res = file.map_err(|deferred| deferred.error());
            assert!(res.contains_err(&ErrorKind::FileNotFound));

            // Creating a new file should work
            let res = File::create(path, None);
            assert!(res.is_ok());

            let file = res.unwrap();

            // Trying to recreate an existing file should not work
            let retry = OpenOptions::new().create_new(1).open::<ErrorKind>(path);
            assert!(retry.contains_err(&ErrorKind::FileAlreadyExists));

            // Closing a file manually should work
            assert!(file.close::<()>().is_ok());

            // Opening an existing file should work
            let res = File::open(path);
            assert!(res.is_ok());
            let mut file = res.unwrap();

            // Restore the memory card to its initial state
            assert!(file.delete::<()>().is_ok());
        })
    }

    #[test_case]
    fn seek_read_write() {
        with_card(|| {
            let path = "bu00:\\NewFile";

            let mut file = File::create(path, None).unwrap();

            // Seeking 1 kB from the start should work
            let res = file.seek::<()>(SeekFrom::Start(1024));
            assert!(res == Ok(1024));

            // Come up with 2 sectors worth of test data to write
            let mut write_buf = [[0; 128]; 2];
            for i in 0..256 {
                write_buf[i / 128][i % 128] = i as u8;
            }
            // Shadow the buffer identifier since it doesn't need to be mutable
            let write_buf = write_buf;

            // Writing 2 sectors should work
            let res = file.write::<()>(&write_buf);
            assert!(res == Ok(256));

            // Seek from the current position to get to the initial writing location
            let res = file.seek::<()>(SeekFrom::Current(-256));
            // Seek returns offset from the start of the file
            assert!(res == Ok(1024));

            // Read two sectors of data
            let mut read_buf = [[0u8; 128]; 2];
            // Read sectors in reverse order to change things up
            let res = file.read::<()>(&mut read_buf[1..]);
            assert!(res == Ok(128));
            // Read the second sector into the first buffer index
            let res = file.read::<()>(&mut read_buf[..1]);
            assert!(res == Ok(128));

            // Read and written buffers should match with indices flipped
            assert!(read_buf[0] == write_buf[1]);
            assert!(read_buf[1] == write_buf[0]);

            // Restore the memory card to its initial state
            assert!(file.delete::<()>().is_ok());
        })
    }

    #[test_case]
    fn rename() {
        with_card(|| {
            // Use memory card 2 to switch things up
            let mut file = File::create("bu10:\\NewFile", None).unwrap();
            // Renaming a file works
            let renamed_file = file.rename::<()>("bu10:\\Renamed");
            assert!(renamed_file.is_ok());

            // Recovering an existing file doesn't work
            let res = file.recover::<ErrorKind>();
            assert!(res.contains_err(&ErrorKind::FileAlreadyExists));

            // Deleting a file works
            assert!(file.delete::<()>().is_ok());
            // Recovering a deleted file works
            assert!(file.recover::<()>().is_ok());

            // Restore the memory card to its initial state
            assert!(file.delete::<()>().is_ok());
        })
    }
}
