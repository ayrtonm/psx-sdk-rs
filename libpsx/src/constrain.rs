#[macro_export]
macro_rules! constrain {
    () => {
            use core::mem::size_of;
            trait Constrain<const N: usize, const M: usize> {
                const ASSERT: bool;
                const __ASSERT: () = if !Self::ASSERT {
                    panic!("Unable to constrain sum")
                } else { };
            }
            
            type NonZST = u32;
    };
    ($N:tt) => {
        {
            type NonZST = u32;
            size_of::<[NonZST; $N]>()
        }
    };
    ($N:tt + $M:tt = $S:tt) => {
        {
            constrain!();
            impl<const N: usize, const M: usize, const S: usize> Constrain<N, M> for [NonZST; S] {
                const ASSERT: bool = constrain!(N) + constrain!(M) == constrain!(S);
            }
            let _: () = <[NonZST; $S] as Constrain<$N, $M>>::__ASSERT;
        }
    };
    ($S:tt = $N:tt + $M:tt) => { constrain!($N + $M = $S) };
    ($N:tt - $M:tt = $S:tt) => { constrain!($S + $M = $N) };
    ($S:tt = $N:tt - $M:tt) => { constrain!($S + $M = $N) };
    ($N:tt + $M:tt < $S:tt) => {
        {
            constrain!();
            impl<const N: usize, const M: usize, const S: usize> Constrain<N, M> for [NonZST; S] {
                const ASSERT: bool = constrain!(N) + constrain!(M) < constrain!(S);
            }
            let _: () = <[NonZST; $S] as Constrain<$N, $M>>::__ASSERT;
        }
    };
    ($N:tt + $M:tt <= $S:tt) => {
        {
            constrain!();
            impl<const N: usize, const M: usize, const S: usize> Constrain<N, M> for [NonZST; S] {
                const ASSERT: bool = constrain!(N) + constrain!(M) <= constrain!(S);
            }
            let _: () = <[NonZST; $S] as Constrain<$N, $M>>::__ASSERT;
        }
    };
    ($N:tt < $S:tt) => { constrain!($N + 0 < $S) };
    ($N:tt > $S:tt) => { constrain!($S < $N) };
    ($N:tt <= $S:tt) => { constrain!($N + 0 <= $S) };
    ($N:tt >= $S:tt) => { constrain!($S <= $N) };
    ($S:tt between $N:tt and $M:tt) => {
        {
            constrain!($N <= $S);
            constrain!($S <= $M);
        }
    };
}
