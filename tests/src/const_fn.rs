#![allow(non_upper_case_globals)]
macro_rules! cmp_primitive {
    ($name:ident, $type:ty) => {
        pub const fn $name(a: &[$type], b: &[$type]) -> bool {
            let min_idx = if a.len() != b.len() {
                return false
            } else {
                a.len()
            };
            let mut ret = true;
            let mut i = 0;
            while i < min_idx {
                ret = ret && a[i] == b[i];
                i += 1;
            }
            ret
        }
    };
}

cmp_primitive!(cmp_u8, u8);
cmp_primitive!(cmp_u32, u32);

pub const pi: f32 = 3.14159;
pub const e: f32 = 2.718281828;
