#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Mode {
  User = 0x10,
  Fiq = 0x11,
  Irq = 0x12,
  Supervisor = 0x13,
  Abort = 0x17,
  Undefined = 0x1B,
  System = 0x1f,
}
impl Mode {
  /// Returns [`true`](bool) if Mode matches [`System`](Mode::System)/[`User`](Mode::User)
  pub const fn userlike(&self) -> bool {
    matches!(self, Mode::User | Mode::System)
  }
}
