macro_rules! timer {
    ([$cnt:ident, $mode:ident, $tgt:ident]) => {
        impl<S: State> SharedCurrent for $cnt<S> {}

        impl Current for $cnt<Mutable> {}

        impl<S: State> SharedTarget for $tgt<S> {}

        impl Target for $tgt<Mutable> {}

        impl<S: State> SharedMode for $mode<S> {}

        impl Mode for $mode<Mutable> {}

        impl<S: State> Debug for $mode<S> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($mode))
                    .field("sync_enabled", &self.sync_enabled())
                    .field("sync_mode", &self.get_sync_mode())
                    .field("source", &self.get_source())
                    .field("target_resets", &self.target_resets())
                    .field("target_irqs", &self.target_irqs())
                    .field("overflow_irqs", &self.overflow_irqs())
                    .field("reached_target", &self.reached_target())
                    .field("overflowed", &self.overflowed())
                    .field("pulsed_irq", &self.pulsed_irq())
                    .finish()
            }
        }
    };

    ([$cnt:ident, $mode:ident, $tgt:ident], $($others:tt)*) => {
        timer!([$cnt, $mode, $tgt]);
        timer!($($others)*);
    };
}
