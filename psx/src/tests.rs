// These macros are workarounds for the limitations of const evaluation and const traits in rust at
// the moment.

/// Makes a function const iff the no_std_test feature is enabled.
#[macro_export]
macro_rules! const_for_tests {
    ($(#[$($meta:meta)*])* $vis:vis $ident:ident $($tokens:tt)*) => {
        #[cfg(feature = "no_std_test")]
        $(#[$($meta)*])* $vis const $ident $($tokens)*
        #[cfg(not(feature = "no_std_test"))]
        $(#[$($meta)*])* $vis $ident $($tokens)*
    };
}

/// Makes a stream of tokens public only for tests.
#[macro_export]
macro_rules! pub_for_tests {
    ($(#[$($meta:meta)*])* $vis:vis $ident:ident $($tokens:tt)*) => {
        #[cfg(feature = "no_std_test")]
        $(#[$($meta)*])* pub $ident $($tokens)*
        #[cfg(not(feature = "no_std_test"))]
        $(#[$($meta)*])* $ident $($tokens)*
    };
}
