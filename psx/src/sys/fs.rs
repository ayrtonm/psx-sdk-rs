//! Memory card and CD-ROM filesystem operations
#![allow(missing_docs)]
use crate::std::AsCStr;
use crate::sys::kernel;
use core::ffi::CStr;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::mem::forget;
#[cfg(feature = "nightlier")]
use core::sync::atomic::AtomicBool;
#[cfg(feature = "nightlier")]
use core::sync::atomic::Ordering;

/// Initialize the filesystem
pub fn init_filesystem() {
    unsafe {
        kernel::psx_set_default_exit_from_exception();
        kernel::psx_init_card(true);
        kernel::psx_start_card();
    }
}

/// Close the filesystem
pub fn close_filesystem() {
    unsafe {
        kernel::psx_stop_card();
    }
}

/// Entries returned by the `ReadDir` iterator.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DirEntry {
    file_name: [u8; 0x14],
    _attribute: u32,
    file_size: u32,
    _unused: u32,
    _first_sector: u32,
    _reserved: u32,
}

impl DirEntry {
    pub unsafe fn from_bytes(ptr: *const [u8; 40]) -> DirEntry {
        *ptr.cast()
    }
}

impl Debug for DirEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DirEntry")
            .field("file_name", &CStr::from_bytes_until_nul(&self.file_name))
            .field("file_size", &self.file_size)
            .finish()
    }
}

// TODO: Explain why bugs make this unsafe. It would be possible to make an
// iterator safe by making it eager but that may not be practical
pub unsafe fn first_file<P: AsRef<[u8]>>(path: P) -> Option<DirEntry> {
    path.as_cstr(|path| {
        let dir_entry = kernel::psx_first_file(path.as_ptr());
        if dir_entry.is_null() {
            None
        } else {
            Some(DirEntry::from_bytes(dir_entry))
        }
    })
}

pub unsafe fn next_file() -> Option<DirEntry> {
    let dir_entry = kernel::psx_next_file();
    if dir_entry.is_null() {
        None
    } else {
        Some(DirEntry::from_bytes(dir_entry))
    }
}

/// A marker trait for the BIOS file types.
pub trait FileTy: Default {
    // TODO: Fix the demos that assumed this was in words
    /// The number of bytes in a sector associated with operations on the given
    /// file type.
    const SECTOR_SIZE: usize;
}

/// A marker type for memory card files managed by the BIOS.
#[derive(Default, Debug)]
pub struct MemCard;

impl FileTy for MemCard {
    const SECTOR_SIZE: usize = 128;
}

/// A marker type for CD-ROM files managed by the BIOS.
#[derive(Default, Debug)]
pub struct CDROM;

impl FileTy for CDROM {
    const SECTOR_SIZE: usize = 2048;
}

/// Options and flags which can be used to configure how a file is opened.
///
/// This builder exposes the ability to configure how a [`File`] is opened and
/// what operations are permitted on the open file. The [`File::open`] and
/// [`File::new`] methods are aliases for commonly used options using this
/// builder.
///
/// Generally speaking, when using `OpenOptions`, you'll first call
/// [`OpenOptions::new`], then chain calls to methods to set each option, then
/// call [`OpenOptions::open`], passing the path of the file you're trying to
/// open. This will give you a [`Result`][core::result::Result] with a [`File`]
/// inside that you can further operate on.
#[derive(Default)]
pub struct OpenOptions<T: FileTy> {
    create: bool,
    async_mode: bool,
    blocks: u16,

    _ty: PhantomData<T>,
}

impl<T: FileTy> OpenOptions<T> {
    /// Creates a blank new set of options ready for configuration.
    ///
    /// All options are initially set to `false`.
    pub fn new() -> Self {
        Self::new_impl()
    }

    // TODO: Remove this when the SYNC issue (#6) is fixed in upstream LLVM
    #[cfg(not(feature = "nightlier"))]
    fn new_impl() -> Self {
        static mut FILESYSTEM_INITIALIZED: bool = false;
        unsafe {
            if !FILESYSTEM_INITIALIZED {
                FILESYSTEM_INITIALIZED = true;
                init_filesystem()
            };
        }
        Default::default()
    }

    #[cfg(feature = "nightlier")]
    fn new_impl() -> Self {
        static FILESYSTEM_INITIALIZED: AtomicBool = AtomicBool::new(false);
        if !FILESYSTEM_INITIALIZED.load(Ordering::Relaxed) {
            FILESYSTEM_INITIALIZED.store(true, Ordering::Relaxed);
            init_filesystem()
        };
        Default::default()
    }

    /// Attempts to open a file.
    ///
    /// Paths should be formatted as `"dev:\\FILE_NAME"` where the
    /// [device name](http://problemkaputt.de/psx-spx.htm#controllerandmemorycardmisc)
    /// `dev` is one of `bu00`, `bu10` or `cdrom` and `FILE_NAME` is the
    /// [file name](http://problemkaputt.de/psx-spx.htm#memorycarddataformat).
    pub fn open<'f, P: AsRef<[u8]>>(&self, path: P) -> Result<File<T>, Error<'f, T>> {
        path.as_cstr(|path| {
            let fd = unsafe { kernel::psx_file_open(path.as_ptr(), self.flags()) };
            match fd {
                i8::MIN..=-2 => Err(Error::Resolved(ErrorKind::UnknownError)),
                -1 => Err(Error::Unresolved),
                0..=i8::MAX => Ok(File {
                    fd,
                    _ty: PhantomData,
                }),
            }
        })
    }

    fn flags(&self) -> u32 {
        1 | ((self.create as u32) << 9) |
            ((self.async_mode as u32) << 15) |
            ((self.blocks as u32) << 16)
    }
}

impl OpenOptions<MemCard> {
    /// Sets the option for asynchronous mode.
    ///
    /// This option, when true, will indicate that the BIOS should not wait for
    /// completion to return.
    pub fn async_mode(&mut self, mode: bool) -> &mut Self {
        self.async_mode = mode;
        self
    }

    /// Sets the option to create a new file, failing if it already exists.
    ///
    /// If created successfully, the new file will contain the specified number
    /// of memory card 8 kB `blocks`.
    pub fn create(&mut self, blocks: u16) -> &mut Self {
        self.blocks = blocks;
        self.create = true;
        self
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
    /// Input/Output buffer is not aligned to 4-bytes
    UnalignedBuffer,
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

/// A BIOS file operation error.
#[derive(Debug)]
pub enum Error<'f, T: FileTy> {
    /// An error known without calling into the BIOS.
    Resolved(ErrorKind),
    /// An error that must be resolved by calling into the BIOS.
    Unresolved,
    /// An error that must be resolved by calling into the BIOS with an explicit
    /// file descriptor.
    UnresolvedFile {
        /// The file descriptor passed to the BIOS to resolve the error.
        file: &'f File<T>,
        #[doc(hidden)]
        _ty: PhantomData<T>,
    },
}

impl<'f, T: FileTy> Error<'f, T> {
    /// Gets the error kind, possibly calling into the BIOS for unresolved
    /// errors.
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::Resolved(kind) => *kind,
            Error::Unresolved => {
                let err = unsafe { kernel::psx_get_last_error() };
                ErrorKind::from(err)
            },
            Error::UnresolvedFile { file, _ty } => {
                let err = unsafe { kernel::psx_get_last_file_error(file.fd) };
                ErrorKind::from(err)
            },
        }
    }
}

/// Possible ways to seek within a file.
///
/// Does not include seeking from end as the BIOS seek is buggy.
pub enum SeekFrom {
    /// An offset for seeking from the start of the file.
    Start(u32),
    /// An offset for seeking from the current position in the file.
    Current(i32),
}

/// A file descriptor for an open memory card or CD-ROM [BIOS file](http://problemkaputt.de/psx-spx.htm#biosfilefunctions).
///
/// Files are automatically closed when they go out of scope.
#[derive(Debug)]
pub struct File<T: FileTy> {
    fd: i8,

    _ty: PhantomData<T>,
}

impl<T: FileTy> File<T> {
    /// Attempts to open a file.
    ///
    /// Paths should be null-terminated and formatted as `"dev:\\FILE_NAME"`
    /// where the [device name](http://problemkaputt.de/psx-spx.htm#controllerandmemorycardmisc)
    /// `dev` is one of `bu00`, `bu10` or `cdrom` and `FILE_NAME` is the
    /// [file name](http://problemkaputt.de/psx-spx.htm#memorycarddataformat).
    pub fn open(path: &str) -> Result<File<T>, Error<T>> {
        OpenOptions::new().open(path)
    }

    /// Seeks to an offset, in bytes, in a file.
    ///
    /// If the seek operation is successful, this method returns the new
    /// position from the start of the file. That position can be used later
    /// with [`SeekFrom::Start`].
    pub fn seek(&self, pos: SeekFrom) -> Result<usize, Error<T>> {
        let (offset, seek_ty) = match pos {
            SeekFrom::Start(offset) => (offset, 0),
            SeekFrom::Current(offset) => (offset as u32, 1),
        };
        let res = unsafe { kernel::psx_file_seek(self.fd, offset, seek_ty) };
        self.try_return_usize(res)
    }

    /// Reads some bytes from the file into `dst`, returning how many bytes were
    /// read.
    ///
    /// Memory card and CD-ROM files can only be read in increments of their
    /// respective sector sizes.
    pub fn read(&self, dst: &mut [u8]) -> Result<usize, Error<T>> {
        if !dst.as_ptr().cast::<u32>().is_aligned() {
            return Err(Error::Resolved(ErrorKind::UnalignedBuffer))
        }
        let res = unsafe { kernel::psx_file_read(self.fd, dst.as_mut_ptr().cast(), dst.len()) };
        self.try_return_usize(res)
    }

    /// Manually closes the file, possibly returning a BIOS error code.
    pub fn close<'f>(self) -> Result<i8, Error<'f, T>> {
        let res = unsafe { kernel::psx_file_close(self.fd) };
        forget(self);
        match res {
            i8::MIN..=-2 => Err(Error::Resolved(ErrorKind::UnknownError)),
            -1 => Err(Error::Unresolved),
            0..=i8::MAX => Ok(res),
        }
    }

    fn try_return_usize(&self, res: i32) -> Result<usize, Error<T>> {
        match res {
            i32::MIN..=-2 => Err(Error::Resolved(ErrorKind::UnknownError)),
            -1 => Err(Error::UnresolvedFile {
                file: self,
                _ty: PhantomData,
            }),
            0..=i32::MAX => Ok(res as usize),
        }
    }
}

impl File<MemCard> {
    /// Attempts to create a new memory card file with the specified `size` in
    /// bytes.
    pub fn new(path: &str, size: usize) -> Result<File<MemCard>, Error<MemCard>> {
        let blocks = size >> 13;
        OpenOptions::new().create(blocks as u16).open(path)
    }

    /// Writes some bytes to the file from the given `src`, returning how many
    /// bytes were written.
    ///
    /// Memory card files can only be written in increments of their sector
    /// size.
    pub fn write(&mut self, src: &[u8]) -> Result<usize, Error<MemCard>> {
        if !src.as_ptr().cast::<u32>().is_aligned() {
            return Err(Error::Resolved(ErrorKind::UnalignedBuffer))
        }
        let res = unsafe { kernel::psx_file_write(self.fd, src.as_ptr().cast(), src.len()) };
        self.try_return_usize(res)
    }
}

impl<T: FileTy> Drop for File<T> {
    fn drop(&mut self) {
        let _res = unsafe { kernel::psx_file_close(self.fd) };
    }
}
