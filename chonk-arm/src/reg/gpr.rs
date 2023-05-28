use crate::mode::Mode;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Gpr {
  R0 = 0x00,
  R1 = 0x01,
  R2 = 0x02,
  R3 = 0x03,
  R4 = 0x04,
  R5 = 0x05,
  R6 = 0x06,
  R7 = 0x07,
  R8 = 0x08,
  R9 = 0x09,
  R10 = 0x0a,
  R11 = 0x0b,
  R12 = 0x0c,
  R13 = 0x0d,
  R14 = 0x0e,
  R15 = 0x0f,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ModeGpr {
  R0 = 0x00,
  R1 = 0x01,
  R2 = 0x02,
  R3 = 0x03,
  R4 = 0x04,
  R5 = 0x05,
  R6 = 0x06,
  R7 = 0x07,
  R8 = 0x08,
  R9 = 0x09,
  R10 = 0x0a,
  R11 = 0x0b,
  R12 = 0x0c,
  R13 = 0x0d,
  R14 = 0x0e,
  R15 = 0x0f,

  // Banked Fiq Registers
  R8Fiq = 0x10,
  R9Fiq = 0x11,
  R10Fiq = 0x12,
  R11Fiq = 0x13,
  R12Fiq = 0x14,

  //Banked R13/R14 registers
  R13Fiq = 0x15,
  R14Fiq = 0x16,
  R13Svc = 0x17,
  R14Svc = 0x18,
  R13Abt = 0x19,
  R14Abt = 0x20,
  R13Irq = 0x21,
  R14Irq = 0x22,
  R13Und = 0x23,
  R14Und = 0x24,
}
impl ModeGpr {
  const MAX: u8 = 0x24;
  const LEN: usize = Self::MAX as usize + 1;
}

impl Gpr {
  /// Converts [`u8`] into [`Gpr`]\
  /// 4 highest bits are ignored
  #[inline]
  pub const fn from_u8(value: u8) -> Self {
    // Safety: Masks the value to ensure it's within the enum's range
    unsafe { std::mem::transmute(value & 0xf) }
  }

  /// Converts [`u8`] into [`Gpr`]\
  /// Safety: value must be in range: 0 ..= 0xf
  #[inline]
  pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
    debug_assert!(value <= 0xf);
    // Safety: Value must be in range: 0 ..= 0xf
    unsafe { std::mem::transmute(value) }
  }
  
  pub fn with_mode(self, mode: Mode) -> ModeGpr {
    //Safety: trust me bro
    let value = match self as u8 {
      0..=7 | 15 => self as u8,
      8..=14 if matches!(mode, Mode::System | Mode::User) => self as u8,
      8..=12 if matches!(mode, Mode::Fiq) => ModeGpr::R8Fiq as u8 + (self as u8 - 8),
      13..=14 => ModeGpr::R13Fiq as u8 + (2 * (self as u8 - 13) + mode as u8),
      _ => self as u8
    };
    debug_assert!(value <= ModeGpr::MAX);
    unsafe { std::mem::transmute(value) }
  }
}

impl From<u8> for Gpr {
  #[inline]
  fn from(value: u8) -> Self {
    Self::from_u8(value)
  }
}
