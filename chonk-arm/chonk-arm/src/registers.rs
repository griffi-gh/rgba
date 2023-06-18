use crate::mode::Mode;

#[derive(Clone, Copy)]
pub struct Registers {
  /// internal storage for all gprs\
  /// (ranges are inclusive)\
  /// - $00-0f - usermode r0-15
  /// - $10-14 - fiq r8-12
  /// - $15-2a - r13-14 for all modes except user\
  ///   (with some empty space for opt purposes)
  gpr: [u32; 0x2b]
}

impl Registers {
  pub const fn new() -> Self {
    Self {
      gpr: [0; 0x2b]
    }
  }

  #[inline(always)]
  const fn reg_idx(r: u8, mode: Mode) -> usize {
    let r = r as usize & 0xf;
    match r {
      8..=12 if matches!(mode, Mode::Fiq) => 0x10 + (r - 8),
      13..=14 if !mode.userlike() => 0x15 + (r - 13) + (2 * (mode as usize - Mode::Fiq as usize)),
      _ => r
    }
  }

  #[inline]
  pub const fn get(&self, reg: u8, mode: Mode) -> u32 {
    self.gpr[Self::reg_idx(reg, mode)]
  }

  #[inline]
  pub fn get_mut(&mut self, reg: u8, mode: Mode) -> &mut u32 {
    &mut self.gpr[Self::reg_idx(reg, mode)]
  }

  #[inline(always)]
  pub const fn get_user(&self, reg: u8) -> u32 {
    self.gpr[(reg & 0xf) as usize]
  }

  #[inline(always)]
  pub fn get_user_mut(&mut self, reg: u8) -> &mut u32 {
    &mut self.gpr[(reg & 0xf) as usize]
  }

  #[inline(always)]
  pub const fn pc(&self) -> u32 {
    self.gpr[15]
  }
}
