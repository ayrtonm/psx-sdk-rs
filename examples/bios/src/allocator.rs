use crate::global::Global;
use core::alloc::{GlobalAlloc, Layout};
use core::mem::size_of;
use core::ptr;
use core::ptr::NonNull;
use linked_list_allocator::Heap;
use psx::hw::{cop0, Register};

#[global_allocator]
pub static HEAP: Global<Heap> = Global::new(Heap::empty());

struct AllocInfo {
    layout: Layout,
    ptr: *mut u8,
}

/// Provides a C allocator API on top of an underlying GlobalAlloc
/// implementation.
///
/// This allocates more memory than GlobalAlloc alone, so it is only useful
/// for cases where the allocator must have the standard malloc/free interface.
trait CAlloc: GlobalAlloc {
    fn malloc(&self, len: usize) -> *mut u8 {
        if len == 0 {
            return ptr::null_mut()
        }
        let header_layout = Layout::new::<AllocInfo>();
        let object_layout = match Layout::from_size_align(len, 1) {
            Ok(layout) => layout,
            Err(_) => return ptr::null_mut(),
        };
        let (request_layout, offset) = match header_layout.extend(object_layout) {
            Ok(res) => res,
            Err(_) => return ptr::null_mut(),
        };
        // SAFETY: len == 0 is checked at the beginning of this function
        let res_ptr = unsafe { self.alloc(request_layout) };
        if res_ptr.is_null() {
            return ptr::null_mut()
        }
        // SAFETY: res_ptr is the result of an allocation for a Layout which allows
        // adding this offset
        let object_ptr = unsafe { res_ptr.add(offset) };
        // SAFETY: subtracting the size of an AllocInfo from obj_ptr should get us
        // an address greater than or equal to res_ptr
        let header_ptr = unsafe { object_ptr.sub(size_of::<AllocInfo>()) as *mut AllocInfo };
        // SAFETY: header_ptr is valid for an AllocInfo
        unsafe {
            header_ptr.write_unaligned(AllocInfo {
                layout: request_layout,
                ptr: res_ptr,
            });
        }
        object_ptr
    }

    unsafe fn free(&self, ptr: *mut u8) {
        if ptr.is_null() {
            return
        }
        let object_ptr = ptr;
        // SAFETY: If ptr is the object_ptr from malloc, then this is in a valid address
        // range
        let header_ptr = unsafe { object_ptr.sub(size_of::<AllocInfo>()) as *const AllocInfo };
        // SAFETY: If ptr is the object_ptr from malloc, then we can read the
        // AllocInfo we previously wrote
        let header = unsafe { header_ptr.read_unaligned() };
        // SAFETY: the AllocInfo contained the pointer and layout that was obtained
        // from alloc
        unsafe {
            self.dealloc(header.ptr, header.layout);
        }
    }
}

unsafe impl GlobalAlloc for Global<Heap> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = cop0::Status::new().critical_section(|| self.as_mut().allocate_first_fit(layout));
        match ptr {
            Ok(nonnull) => nonnull.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let ptr = match NonNull::new(ptr) {
            Some(ptr) => ptr,
            None => return,
        };
        cop0::Status::new().critical_section(|| self.as_mut().deallocate(ptr, layout))
    }
}

impl CAlloc for Global<Heap> {}

pub fn init_heap(addr: *mut u8, len: usize) -> u32 {
    cop0::Status::new().critical_section(|| {
        // SAFETY: Let's hope the user passed an unused region of memory
        unsafe { HEAP.as_mut().init(addr, len) }
    });
    0
}

pub fn malloc(len: usize) -> *mut u8 {
    HEAP.malloc(len)
}

pub fn free(ptr: *mut u8) -> u32 {
    // SAFETY: Let's hope the user passed in a pointer that we handed out in malloc
    unsafe {
        HEAP.free(ptr);
    }
    0
}
