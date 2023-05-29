use crate::mode::Mode;

#[derive(Clone, Copy)]
pub struct Registers {
  /// internal storage for all gprs
  /// 0..=0xf - GPR
  /// 0x10..=
  gpr: [u32; 31]
}

impl Registers {
  const fn reg_idx(r: u8, mode: Mode) -> usize {
    let r = r as usize & 0xf;
    match r {
      8..=12 if matches!(mode, Mode::Fiq) => (r - 8) + 0xf,
      13..=14 if !mode.userlike() => (r - 13) + (2 * mode as usize),
      _ => r
    }
  }

  pub fn new() -> Self {
    Self {
      gpr: [0; 31]
    }
  }

  pub fn get_reg(&self, r: u8) -> u32 {
    self.gpr[r as usize & 0xf]
  }

  pub fn get_reg_mode(&self, r: u8) -> u32 {
    self.gpr[r as usize & 0xf]
  }
}
