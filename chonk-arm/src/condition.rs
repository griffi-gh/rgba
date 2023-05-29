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
  SignedGreaterOrEqual = 0xa,

  /// LT; N<>V; signed less than
  SignedLessThan = 0xb,

  /// GT; Z=0 and N=V; signed greater than
  SignedGreaterThan = 0xc,

  /// LE; Z=1 or N<>V; signed less or equal
  SignedLessOrEqual = 0xd,

  /// AL; always (the "AL" suffix can be omitted)
  #[default]
  Always = 0xe,

  /// NV; never (ARMv1,v2 only) (Reserved ARMv3 and up)\
  ///
  /// - Should be treated like [`Always`](Condition::Always) on ARMv4\
  ///   Added here just for the sake of completeness
  Reserved = 0xf,
}

impl From<u8> for Condition {
  #[inline]
  fn from(value: u8) -> Self {
    unsafe { std::mem::transmute(value & 0xf) }
  }
}

impl From<Condition> for u8 {
  #[inline]
  fn from(value: Condition) -> Self {
    value as u8
  }
}
