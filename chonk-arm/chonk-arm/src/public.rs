use crate::cpu::Cpu;
pub use crate::memory::MemoryInterface;

#[repr(transparent)]
pub struct Arm7Tdmi(Cpu);

impl Arm7Tdmi {
  pub fn new(mem: impl MemoryInterface + 'static) -> Self {
    Self(Cpu::new(mem))
  }
  pub fn interface(&self) -> &(dyn MemoryInterface + 'static) {
    self.0.interface()
  }
  pub fn interface_mut(&mut self) -> &mut (dyn MemoryInterface + 'static) {
    self.0.interface_mut()
  }
}
