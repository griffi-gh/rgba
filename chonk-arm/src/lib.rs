mod cond;
mod reg;
mod mode;

use reg::Registers;

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
