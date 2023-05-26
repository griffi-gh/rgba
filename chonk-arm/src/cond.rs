#[repr(u8)]
#[derive(Clone, Copy, Debug, Default)]
/// Represents an ARM condition
pub enum Condition {
  /// EQ; Z=1; equal (zero) (same)
  Equal = 0x0,

  /// NE; Z=0; not equal (nonzero) (not same)
  NotEqual = 0x1,

  /// CS/HS; C=1; unsigned higher or same (carry set)
  UnsignedHigherOrSame = 0x2,

  /// CC/LO; C=0; unsigned lower (carry cleared)
  UnsignedLower = 0x3,

  /// MI; N=1; signed negative (minus)
  SignedNegative = 0x4,

  /// PL; N=0; signed positive or zero (plus)
  SignedPositiveOrZero = 0x5,

  /// VS; V=1; signed overflow (V set)
  SignedOverflow = 0x6,

  /// VC; V=0; signed no overflow (V cleared)
  SignedNoOverflow = 0x7,

  /// HI; C=1 and Z=0; unsigned higher
  UnsignedHigher = 0x8,

  /// LS; C=0 or Z=1; unsigned lower or same
  UnsignedLowerOrSame = 0x9,

  /// GE; N=V; signed greater or equal
  SignedGreaterOrEqual = 0xA,

  /// LT; N<>V; signed less than
  SignedLessThan = 0xB,

  /// GT; Z=0 and N=V; signed greater than
  SignedGreaterThan = 0xC,

  /// LE; Z=1 or N<>V; signed less or equal
  SignedLessOrEqual = 0xD,

  /// AL; always (the "AL" suffix can be omitted)
  #[default]
  Always = 0xE,

  /// NV; never (ARMv1,v2 only) (Reserved ARMv3 and up)\
  ///
  /// - Should be treated like [`Always`](Condition::Always) on ARMv4\
  ///   Added here just for the sake of completeness
  Reserved = 0xF,
}

impl Condition {
  /// Converts [`u8`] into [`Condition`]\
  /// 4 highest bits are ignored
  #[inline]
  pub const fn from_u8(value: u8) -> Self {
    // Safety: Masks the value to ensure it's within the enum's range
    unsafe { std::mem::transmute(value & 0xf) }
  }

  /// Converts [`u8`] into [`Condition`]\
  /// Safety: value must be in range: 0 ..= 0xf
  #[inline]
  pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
    debug_assert!(value <= 0xf);
    // Safety: Value must be in range: 0 ..= 0xf
    unsafe { std::mem::transmute(value) }
  }

  /// Converts [`Condition`] into [`u8`]
  #[inline]
  pub const fn into_u8(self) -> u8 {
    self as u8
  }
}

impl From<u8> for Condition {
  #[inline]
  fn from(value: u8) -> Self {
    Self::from_u8(value)
  }
}

impl From<Condition> for u8 {
  #[inline]
  fn from(value: Condition) -> Self {
    value.into_u8()
  }
}
