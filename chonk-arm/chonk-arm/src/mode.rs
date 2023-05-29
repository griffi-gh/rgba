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
impl Mode {
  /// Returns [`true`](bool) if Mode matches [`System`](Mode::System)/[`User`](Mode::User)
  pub const fn userlike(&self) -> bool {
    matches!(self, Mode::User | Mode::System)
  }
}
