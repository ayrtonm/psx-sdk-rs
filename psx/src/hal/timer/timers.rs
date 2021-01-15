macro_rules! timer {
    ([$cnt:ident, $mode:ident, $tgt:ident]) => {
        impl<S: State> Counter for $cnt<S> {}

        impl MutCounter for $cnt<Mutable> {}

        impl<S: State> Target for $tgt<S> {}

        impl MutTarget for $tgt<Mutable> {}

        impl<S: State> Mode for $mode<S> {}

        impl MutMode for $mode<Mutable> {}
    };

    ([$cnt:ident, $mode:ident, $tgt:ident], $($others:tt)*) => {
        timer!([$cnt, $mode, $tgt]);
        timer!($($others)*);
    };
}
