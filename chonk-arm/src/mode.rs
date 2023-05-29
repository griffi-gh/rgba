#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Mode {
  System,
  User,
  Fiq,
  Supervisor,
  Abort,
  Irq,
  Undefined,
}
