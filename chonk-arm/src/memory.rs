pub trait MemoryInterface {
  fn read_8(&self, addr: u32) -> u8;
  fn read_16(&self, addr: u32) -> u16;
  fn read_32(&self, addr: u32) -> u32;
  fn write_8(&mut self, addr: u32, value: u8);
  fn write_16(&mut self, addr: u32, value: u16);
  fn write_32(&mut self, addr: u32, value: u32);
}

pub struct MemoryBus {
  pub interface: Box<dyn MemoryInterface>
}

impl MemoryBus {
  pub fn new(interface: impl MemoryInterface + 'static) -> Self {
    Self {
      interface: Box::new(interface)
    }
  }
}
