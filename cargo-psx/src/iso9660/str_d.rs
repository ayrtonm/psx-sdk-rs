const D_CHAR_SET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

impl<const N: usize> Default for StrD<N> {
    fn default() -> Self {
        Self([0x20; N])
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StrD<const N: usize>([u8; N]);

#[derive(Debug, Clone)]
pub struct VarStrD(Box<[u8]>);

impl VarStrD {
    pub fn new(x: &Vec<u8>) -> Self {
        println!("{:?}", std::str::from_utf8(x));
        Self::new_checked(x).unwrap()
    }
    pub fn new_checked(x: &Vec<u8>) -> Option<Self> {
        for c in x {
            if !D_CHAR_SET.contains(c) {
                return None
            }
        }
        Some(Self(x.clone().into_boxed_slice()))
    }
}
impl<const N: usize> StrD<N> {
    pub fn new(x: &[u8]) -> Self {
        Self::new_checked(x).unwrap()
    }
    pub fn new_checked(x: &[u8]) -> Option<Self> {
        if x.len() > N {
            return None
        }
        let n = if x.len() < N { x.len() } else { N };
        let mut s = [0; N];
        for c in &x[0..n] {
            if !D_CHAR_SET.contains(c) {
                return None
            }
        }
        s[0..n].copy_from_slice(&x[0..n]);
        if n < N {
            s[n..N].fill(0x20);
        }
        Some(Self(s))
    }
}
