#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Mode {
  System = 0x81,
  User = 0x80,
  Fiq = 0,
  Supervisor = 1,
  Abort = 2,
  Irq = 3,
  Undefined = 4,
}
