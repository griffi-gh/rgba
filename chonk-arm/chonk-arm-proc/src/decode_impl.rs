use const_twiddle::Twiddle;
use crate::{ExecArm, ExecThumb};

pub(crate) fn decode_arm(instr: u32) -> ExecArm {
  ExecArm::Panic
}

pub(crate) fn decode_thumb(instr: u32) -> ExecThumb {
  if instr & 0xf800 == 0x1800 {
    ExecThumb::AddSub {
      sub: instr.bit(9),
      imm: instr.bit(10),
      rn: instr.bits(6..=7) as _,
    }
  } else {
    ExecThumb::Panic
  }
}
