use core::hint::unreachable_unchecked;
use core::ops::{Range, RangeFrom};
use core::ptr::slice_from_raw_parts;

/// Prints a formatted message with up to four u32 arguments to the TTY console.
#[macro_export]
macro_rules! printf {
    ($msg:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        crate::bios::printf($msg.as_ptr(), $arg0, $arg1, $arg2, $arg3);
    };

    ($msg:expr $(,$args:expr)*) => {
        printf!($msg $(,$args)*, unsafe { core::mem::MaybeUninit::uninit().assume_init() });
    };
}

// cfg(test) is only needed because this is private and only used in tests for
// now
#[cfg(test)]
macro_rules! slice_cmp {
    ($a:expr, $b:expr) => {{
        let n = $a.len();
        let mut res = true;
        if n != $b.len() {
            res = false;
        }
        const_for! {
            i in 0, n => {
                if $a[i] != $b[i] {
                    res = false;
                }
            }
        }
        res
    }};
}

macro_rules! const_for {
    {$idx:ident in $start:expr, $end:expr => $body:block} => {
        {
            let mut $idx = $start;
            while $idx < $end {
                $body
                $idx += 1;
            }
        }
    };
}

macro_rules! const_iter {
    {$element:ident in $slice:expr => $body:block} => {
        {
            let mut i = 0;
            while i < $slice.len() {
                let $element = unsafe { *$crate::std::get_unchecked($slice, i) };
                i += 1;
                $body
            }
        }
    };

    {&$element:ident in $slice:expr => $body:block} => {
        {
            let mut i = 0;
            while i < $slice.len() {
                let $element = unsafe { &*$crate::std::get_unchecked($slice, i) };
                i += 1;
                $body
            }
        }
    };
}

pub const fn illegal() -> ! {
    if cfg!(feature = "forbid_UB") {
        panic!("")
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub const unsafe fn get_unchecked<T>(slice: &[T], idx: usize) -> &T {
    &*slice.as_ptr().add(idx)
}

pub const unsafe fn slice<T>(slice: &[T], range: Range<usize>) -> &[T] {
    let ptr = slice.as_ptr().add(range.start);
    let len = range.end - range.start;
    &*slice_from_raw_parts(ptr, len)
}

pub const unsafe fn slice_from<T>(slice: &[T], range: RangeFrom<usize>) -> &[T] {
    let ptr = slice.as_ptr().add(range.start);
    let len = slice.len() - range.start;
    &*slice_from_raw_parts(ptr, len)
}

pub const fn binary_search(slice: &[u32], x: u32) -> Option<usize> {
    let mut size = slice.len();
    if size == 0 {
        return None
    };
    let mut base = 0;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let cmp = unsafe { *get_unchecked(slice, mid) } > x;
        base = if cmp { base } else { mid };
        size -= half;
    }
    if unsafe { *get_unchecked(slice, base) } == x {
        Some(base)
    } else {
        None
    }
}
