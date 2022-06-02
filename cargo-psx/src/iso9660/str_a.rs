const A_CHAR_SET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 _!\"%&'()*+,-./:;<=>?";

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StrA<const N: usize>([u8; N]);

#[derive(Debug, Clone)]
pub struct VarStrA(Box<[u8]>);

impl<const N: usize> Default for StrA<N> {
    fn default() -> Self {
        Self([0x20; N])
    }
}

/// A marker trait for types that contain only charset A
pub trait CharSetA {}
impl CharSetA for VarStrA {}
impl<const N: usize> CharSetA for StrA<N> {}

impl VarStrA {
    pub fn new(x: &Vec<u8>) -> Self {
        println!("{:?}", std::str::from_utf8(x));
        Self::new_checked(x).unwrap()
    }
    pub fn new_checked(x: &Vec<u8>) -> Option<Self> {
        for c in x {
            if !A_CHAR_SET.contains(c) {
                return None
            }
        }
        Some(Self(x.clone().into_boxed_slice()))
    }
}

impl<const N: usize> StrA<N> {
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
            if !A_CHAR_SET.contains(c) {
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
