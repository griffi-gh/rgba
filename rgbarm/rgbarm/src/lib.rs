mod condition;
mod registers;
mod memory;
mod mode;
mod cpu;
mod instructions;

/// Internal usage only
pub mod _orbit {
  pub use crate::instructions::{handlers, ArmHandlerFn, ThumbHandlerFn};
}

mod public {
  use crate::cpu::Cpu;
  pub use crate::memory::MemoryInterface;

  #[repr(transparent)]
  pub struct Arm7Tdmi(Cpu);

  impl Arm7Tdmi {
    pub fn new(mem: impl MemoryInterface + 'static) -> Self {
      Self(Cpu::new(mem))
    }
    pub fn interface(&self) -> &(dyn MemoryInterface + 'static) {
      self.0.mem.interface.as_ref()
    }
    pub fn interface_mut(&mut self) -> &mut (dyn MemoryInterface + 'static) {
      self.0.mem.interface.as_mut()
    }
  }
}

pub use public::*;
