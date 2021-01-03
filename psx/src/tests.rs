/// Makes a function const iff the no_std_test feature is enabled. The idea is
/// that I want some functions to be const for testing, but I don't want to
/// restrict them to only calling const functions at runtime. The point of being
/// able to call non-const functions at runtime is to conditionally reuse
/// libcore's version of methods I've rewritten to remove runtime sanity checks.
#[macro_export]
macro_rules! const_for_tests {
    ($(#[$($meta:meta)*])* $vis:vis $ident:ident $($tokens:tt)*) => {
        #[cfg(feature = "no_std_test")]
        $(#[$($meta)*])* $vis const $ident $($tokens)*
        #[cfg(not(feature = "no_std_test"))]
        $(#[$($meta)*])* $vis $ident $($tokens)*
    };
}
