macro_rules! channel {
    ([$madr:ident, $bcr:ident, $chcr:ident]) => {
        impl<S: State> MemoryAddress for $madr<S> {}

        impl MutMemoryAddress for $madr<Mutable> {}

        impl<S: State> BlockControl for $bcr<S> {}

        impl MutBlockControl for $bcr<Mutable> {}

        impl<S: State> ChannelControl for $chcr<S> {}

        impl MutChannelControl for $chcr<Mutable> {}
    };

    ([$madr:ident, $bcr:ident, $chcr:ident], $($others:tt)*) => {
        channel!([$madr, $bcr, $chcr]);
        channel!($($others)*);
    };
}
