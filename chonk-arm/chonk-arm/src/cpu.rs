use crate::{
  registers::Registers,
  memory::{MemoryInterface, MemoryBus},
};

pub struct Cpu {
  reg: Registers,
  mem: MemoryBus,
}

impl Cpu {
  pub fn new(mem: impl MemoryInterface + 'static) -> Self {
    Self {
      reg: Registers::new(),
      mem: MemoryBus::new(mem)
    }
  }
  pub fn interface(&self) -> &(dyn MemoryInterface + 'static) {
    self.mem.interface.as_ref()
  }
  pub fn interface_mut(&mut self) -> &mut (dyn MemoryInterface + 'static) {
    self.mem.interface.as_mut()
  }
}
