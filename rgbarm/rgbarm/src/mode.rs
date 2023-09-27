use proc_bitfield::UnsafeFrom;
use strum::FromRepr;

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromRepr)]
#[repr(u8)]
pub enum Mode {
  User = 0x10,
  Fiq = 0x11,
  Irq = 0x12,
  Supervisor = 0x13,
  Abort = 0x17,
  Undefined = 0x1b,
  System = 0x1f,
}

impl Mode {
  /// Returns [`true`](bool) if Mode matches [`System`](Mode::System)/[`User`](Mode::User)
  #[inline]
  pub const fn userlike(&self) -> bool {
    matches!(self, Mode::User | Mode::System)
  }
}

impl From<Mode> for u8 {
  #[inline]
  fn from(value: Mode) -> Self {
    value as u8
  }
}

impl TryFrom<u8> for Mode {
  type Error = ();

  #[inline]
  fn try_from(discriminant: u8) -> Result<Self, Self::Error> {
    Self::from_repr(discriminant).ok_or(())
  }
}

impl UnsafeFrom<u8> for Mode {
  /// SAFETY: must be a valid value for Mode
  #[inline]
  unsafe fn unsafe_from(discriminant: u8) -> Self {
    std::mem::transmute(discriminant)
  }
}
