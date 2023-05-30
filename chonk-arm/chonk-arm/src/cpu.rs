use crate::{
  registers::Registers,
  memory::{MemoryInterface, MemoryBus},
};

pub struct Cpu {
  pub reg: Registers,
  pub mem: MemoryBus,
}

impl Cpu {
  pub fn new(mem: impl MemoryInterface + 'static) -> Self {
    Self {
      reg: Registers::new(),
      mem: MemoryBus::new(mem)
    }
  }
}
