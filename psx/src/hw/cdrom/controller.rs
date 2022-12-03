use crate::hw::cdrom::{Command, Controller, Parameter};
use crate::hw::Register;

impl Controller {
    pub fn send_cmd(&mut self, cmd: Command) -> &mut Self {
        self.assign(cmd as u8)
    }
}

impl Parameter {
    pub fn set_param(&mut self, param: u8) -> &mut Self {
        self.assign(param as u8)
    }
}
