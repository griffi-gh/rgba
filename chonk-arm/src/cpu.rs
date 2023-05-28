mod condition;
mod registers;
mod mode;

use registers::Registers;

pub struct Cpu {
  reg: Registers
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      reg: Registers::new()
    }
  }
}
