use chonk_arm::{Arm7Tdmi, MemoryInterface};

struct MemoryBus;
impl MemoryInterface for MemoryBus {
  fn read_8(&self, addr: u32) -> u8 { todo!() }
  fn read_16(&self, addr: u32) -> u16 { todo!() }
  fn read_32(&self, addr: u32) -> u32 { todo!() }
  fn write_8(&mut self, addr: u32, value: u8) { todo!() }
  fn write_16(&mut self, addr: u32, value: u16) { todo!() }
  fn write_32(&mut self, addr: u32, value: u32) { todo!() }
}

pub struct Gba {
  cpu: Arm7Tdmi
}

impl Gba {
  pub fn new() -> Self {
    Self {
      cpu: Arm7Tdmi::new(MemoryBus)
    }
  }
}

impl Default for Gba {
  fn default() -> Self {
    Self::new()
  }
}
