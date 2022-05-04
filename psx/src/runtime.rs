use core::mem::{size_of, transmute};

/// Define a constructor that runs before `main`.
#[macro_export]
macro_rules! ctor {
    (fn $name:ident() { $($body:tt)* }) => {
        #[used]
        #[link_section = ".ctors"]
        #[allow(non_upper_case_globals)]
        static $name: fn() = || {
            $($body)*
        };
    };
}

/// Define a destructor that runs after `main`.
///
/// Since the runtime `panic!`s after `main` returns, this is mostly useful with
/// the `loadable_exe` feature.
#[macro_export]
macro_rules! dtor {
    (fn $name:ident() { $($body:tt)* }) => {
        #[used]
        #[link_section = ".dtors"]
        #[allow(non_upper_case_globals)]
        static $name: fn() = || {
            $($body)*
        };
    };
}

#[cfg(feature = "loadable_exe")]
type RtReturn = ();
#[cfg(not(feature = "loadable_exe"))]
type RtReturn = !;

/// The runtime used by the default linker scripts.
#[no_mangle]
extern "C" fn __start() -> RtReturn {
    // SAFETY: If there is no unmangled function named `main` this causes an error
    // at link-time.
    unsafe {
        #[cfg(not(test))]
        extern "Rust" {
            fn main();
        }
        extern "C" {
            static __ctors_start: usize;
            static __ctors_end: usize;
            static __dtors_start: usize;
            static __dtors_end: usize;
        }
        let start = &__ctors_start as *const usize as usize;
        let end = &__ctors_end as *const usize as usize;
        let ctors_range = end - start;
        assert!(
            (ctors_range % 4) == 0,
            ".ctors section is not 4-byte aligned"
        );
        let num_ctors = ctors_range / size_of::<usize>();
        for n in 0..num_ctors {
            let ctor_ptr = start + (n * size_of::<usize>());
            let fn_addr = *(ctor_ptr as *const usize);
            let ctor = transmute::<usize, fn()>(fn_addr);
            ctor();
        }
        #[cfg(not(test))]
        main();

        #[cfg(test)]
        crate::main();

        let start = &__dtors_start as *const usize as usize;
        let end = &__dtors_end as *const usize as usize;
        let dtors_range = end - start;
        assert!(
            (dtors_range % 4) == 0,
            ".dtors section is not 4-byte aligned"
        );
        let num_dtors = dtors_range / size_of::<usize>();
        for n in 0..num_dtors {
            let dtor_ptr = start + (n * size_of::<usize>());
            let fn_addr = *(dtor_ptr as *const usize);
            let dtor = transmute::<usize, fn()>(fn_addr);
            dtor();
        }
    }
    #[cfg(not(feature = "loadable_exe"))]
    panic!("`main` should not return")
}

// Define string-literals to embed in PSEXE header
// Using the same identifier for all regions conveniently makes the crate
// features mutually exclusive
#[cfg(any(
    feature = "NA_region",
    feature = "EU_region",
    feature = "J_region",
    test
))]
macro_rules! as_array {
    ($msg:literal) => {
        // SAFETY: This dereferences a pointer to a literal which has a static lifetime.
        unsafe { *($msg.as_ptr() as *const _) }
    };
}

#[cfg(any(feature = "NA_region", test))]
#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 55] = as_array!("Sony Computer Entertainment Inc. for North America area");

#[cfg(feature = "EU_region")]
#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 48] = as_array!("Sony Computer Entertainment Inc. for Europe area");

#[cfg(feature = "J_region")]
#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 47] = as_array!("Sony Computer Entertainment Inc. for Japan area");
