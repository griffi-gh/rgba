use proc_bitfield::bitfield;
use crate::mode::Mode;

mod alu;

bitfield! {
  #[derive(Clone, Copy, Default)]
  pub struct AluFlags(pub u8): Debug, FromRaw, IntoRaw {
    pub sign: bool @ 3,
    pub zero: bool @ 2,
    pub carry: bool @ 1,
    pub overflow: bool @ 0,
  }
}

bitfield! {
  #[derive(Clone, Copy, Default)]
  pub struct Psr(pub u32): Debug, FromRaw, IntoRaw {
    pub flags: u8 [AluFlags] @ 27..=31,
    pub irq_disable: bool @ 7,
    pub fiq_disable: bool @ 6,
    pub state: bool @ 5,
    pub mode: u8 [try_get Mode, set Mode] @ 0..=4, //unsafe_get?
  }
}

#[derive(Clone, Copy)]
pub struct Registers {
  /// internal storage for all gprs\
  /// (ranges are inclusive)\
  /// - $00-0f - usermode r0-15
  /// - $10-14 - fiq r8-12
  /// - $15-2a - r13-14 for all modes except user\
  ///   (with some empty space for opt purposes)
  gpr: [u32; 0x2b],
  /// internal storage for PSR Registers\
  /// CPSR + SPSR (with some empty space for opt purposes)
  psr: [Psr; 0x10],
}

impl Registers {
  pub const fn new() -> Self {
    Self {
      gpr: [0; 0x2b],
      psr: [Psr(0); 0x10],
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

  #[inline(always)]
  pub const fn get_mode(&self, reg: u8, mode: Mode) -> u32 {
    self.gpr[Self::reg_idx(reg, mode)]
  }

  #[inline(always)]
  pub fn get_mode_mut(&mut self, reg: u8, mode: Mode) -> &mut u32 {
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
  pub fn get(&self, reg: u8) -> u32 {
    self.get_mode(reg, self.current_mode())
  }

  #[inline(always)]
  pub fn get_mut(&mut self, reg: u8) -> &mut u32 {
    self.get_mode_mut(reg, self.current_mode())
  }

  #[inline(always)]
  pub const fn pc(&self) -> u32 {
    self.gpr[15]
  }

  #[inline(always)]
  pub const fn cpsr(&self) -> Psr {
    self.psr[0]
  }

  #[inline(always)]
  pub fn cpsr_mut(&mut self) -> &mut Psr {
    &mut self.psr[0]
  }

  //XXX: maybe return Option<Psr> instead?

  #[inline(always)]
  pub const fn spsr(&self, mode: Mode) -> Psr {
    match mode.userlike() {
      true => self.psr[0],
      false => self.psr[mode as usize]
    }
  }

  #[inline(always)]
  pub fn spsr_mut(&mut self, mode: Mode) -> &mut Psr {
    match mode.userlike() {
      true => &mut self.psr[0],
      false => &mut self.psr[mode as usize]
    }
  }

  #[inline(always)]
  pub fn current_mode(&self) -> Mode {
    self.cpsr().mode().unwrap()
  }
}
