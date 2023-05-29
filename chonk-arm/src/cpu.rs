use crate::registers::Registers;
use crate::memory::MemoryInterface;

pub struct Cpu {
  reg: Registers,
  mem: Box<dyn MemoryInterface>
}

impl Cpu {
  pub fn new(mem: impl MemoryInterface + 'static) -> Self {
    Self {
      reg: Registers::new(),
      mem: Box::new(mem)
    }
  }
}
