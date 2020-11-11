use crate::gpu::{GP0, GP1};

pub struct IOCtxt {
    gp0: Option<GP0>,
    gp1: Option<GP1>,
}

impl IOCtxt {
    pub fn take_gp0(&mut self) -> Option<GP0> {
        self.gp0.take()
    }
    pub fn take_gp1(&mut self) -> Option<GP1> {
        self.gp1.take()
    }
    pub fn replace_gp0(&mut self, gp0: Option<GP0>) {
        self.gp0 = gp0;
    }
    pub fn replace_gp1(&mut self, gp1: Option<GP1>) {
        self.gp1 = gp1;
    }
}

impl Drop for GP0 {
    fn drop(&mut self) {
        unsafe {
            IOCX.replace_gp0(Some(GP0));
        }
    }
}

impl Drop for GP1 {
    fn drop(&mut self) {
        unsafe {
            IOCX.replace_gp1(Some(GP1));
        }
    }
}

pub static mut IOCX: IOCtxt = IOCtxt {
    gp0: Some(GP0),
    gp1: Some(GP1),
};
