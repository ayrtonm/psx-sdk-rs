pub struct Result {
    pub pass: bool,
    pub msg: Option<&'static str>,
}

macro_rules! ok {
    () => {
        $crate::test::Result {
            pass: true,
            msg: None,
        }
    };
    ($msg:literal) => {
        $crate::test::Result {
            pass: true,
            msg: Some(concat!("Passed `", $msg, "` test")),
        }
    }
}

macro_rules! fail {
    () => {
        $crate::test::Result {
            pass: false,
            msg: None,
        }
    };
    ($msg:literal) => {
        $crate::test::Result {
            pass: false,
            msg: Some(concat!("Failed `", $msg, "` test")),
        }
    };
}

macro_rules! test {
    {const fn $name:ident() $body:block} => {
        #[allow(dead_code)]
        const fn $name() -> $crate::test::Result {
            $body
        }
        const _: () = {
            let res = $name();
            if !res.pass {
                let msg = match res.msg {
                    Some(msg) => msg,
                    None => stringify!($name),
                };
                panic!(msg);
            }
        };
    };
    {const fn $name:ident() -> bool $body:block} => {
        #[allow(dead_code)]
        const fn $name() -> bool {
            $body
        }
        const _: () = {
            let res = if $name() {
                ok!()
            } else {
                fail!()
            };
            if !res.pass {
                let msg = match res.msg {
                    Some(msg) => msg,
                    None => stringify!($name),
                };
                panic!(msg);
            }
        };
    };
}

macro_rules! slice_cmp {
    ($a:expr, $b:expr) => {
        {
            let n = $a.len();
            assert!(n == $b.len());
            let mut res = true;
            const_for! {
                i in 0, n => {
                    if $a[i] != $b[i] {
                        res = false;
                    }
                }
            }
            res
        }
    };
}
