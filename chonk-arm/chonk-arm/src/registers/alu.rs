use super::AluFlags;

impl AluFlags {
  ///Updates sign and zero based on result
  #[inline]
  pub fn alu_flags(&mut self, result: u32, carry: bool, overflow: bool) {
    self.set_sign((result as i32) < 0);
    self.set_zero(result == 0);
    self.set_carry(carry);
    self.set_overflow(overflow);
  }
  
  #[inline]
  pub fn alu_add(&mut self, lhs: u32, rhs: u32) -> u32 {
    let (result, carry) = lhs.overflowing_add(rhs);
    let (_, overflow) = (lhs as i32).overflowing_add(rhs as i32);
    self.alu_flags(result, carry, overflow);
    result
  }
  
  #[inline]
  pub fn alu_sub(&mut self, lhs: u32, rhs: u32) -> u32 {
    let (result, carry) = lhs.overflowing_sub(rhs);
    let (_, overflow) = (lhs as i32).overflowing_sub(rhs as i32);
    self.alu_flags(result, carry, overflow);
    result
  }
}
