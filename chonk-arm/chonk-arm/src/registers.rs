use crate::mode::Mode;

#[derive(Clone, Copy)]
pub struct Registers {
  /// internal storage for all gprs\
  /// (ranges are inclusive)\
  /// - $00-0f - usermode r0-15
  /// - $10-14 - fiq r8-12
  /// - $15-1f - r13-14 for all modes except user
  gpr: [u32; 31]
}

impl Registers {
  pub const fn new() -> Self {
    Self {
      gpr: [0; 31]
    }
  }

  #[inline(always)]
  const fn reg_idx(r: u8, mode: Mode) -> usize {
    let r = r as usize & 0xf;
    let m = mode as usize;

    debug_assert!(m <= 4);
    debug_assert!(Mode::Fiq as u8 == 0);

    match r {
      8..=12 if matches!(mode, Mode::Fiq) => 0x10 + (r - 8),
      13..=14 if !mode.userlike() => 0x15 + (r - 13) + (2 * m),
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
}
