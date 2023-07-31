use bit_field::BitField;
use crate::{ExecArm, ExecThumb};

pub(crate) fn decode_arm(_instr: u32) -> ExecArm {
  //TODO decode_arm
  ExecArm::Panic
}

pub(crate) fn decode_thumb(instr: u32) -> ExecThumb {
  if instr & 0xf800 == 0x1800 {
    ExecThumb::AddSub {
      op: instr.get_bit(9),
      imm: instr.get_bit(10),
      rn: instr.get_bits(6..=7) as _,
    }
  } else {
    ExecThumb::Panic
  }
}
