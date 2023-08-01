use chonk_arm::{Arm7Tdmi, MemoryInterface};

mod scheduler;

struct MemoryBus;
impl MemoryInterface for MemoryBus {
  #[inline(always)]
  fn read_8(&self, addr: u32) -> u8 {
    todo!("read_8")
  }

  #[inline(always)]
  fn read_16(&self, addr: u32) -> u16 {
    todo!("read_16")
  }

  #[inline(always)]
  fn read_32(&self, addr: u32) -> u32 {
    todo!("read_32")
  }

  #[inline(always)]
  fn write_8(&mut self, addr: u32, value: u8) {
    todo!("write_8")
  }

  #[inline(always)]
  fn write_16(&mut self, addr: u32, value: u16) {
    todo!("write_16")
  }

  #[inline(always)]
  fn write_32(&mut self, addr: u32, value: u32) {
    todo!("write_32")
  }
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
