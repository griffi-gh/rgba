pub mod gpr;

use gpr::{Gpr, ModeGpr};

pub struct Registers {
  gpr: [u32; ModeGpr::LEN]
}

impl Registers {
  pub fn new() -> Self {
    Self {
      gpr: [0; ModeGpr::LEN]
    }
  }
  pub fn get_mode(&self, gpr: ModeGpr) -> u32 {
    self.gpr[gpr as usize]
  }
  pub fn set_mode(&mut self, gpr: ModeGpr, value: u32) {
    self.gpr[gpr as usize] = value;
  }
  pub fn get_user(&self, gpr: Gpr) -> u32 {
    self.gpr[gpr as usize]
  }
  pub fn set_user(&mut self, gpr: Gpr, value: u32) {
    self.gpr[gpr as usize] = value;
  }
}
