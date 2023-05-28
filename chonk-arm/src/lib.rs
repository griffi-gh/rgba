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
  pub fn _test(&mut self) {
    use reg::*;
    use gpr::*;
    use mode::*;
    self.reg.get_mode(Gpr::R15.with_mode(Mode::User));
    self.reg.get_user(Gpr::R15);
  }
}
