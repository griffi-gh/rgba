use bit_field::BitField;
use crate::{ExecArm, ExecThumb};

pub(crate) fn decode_arm(_instr: u32) -> ExecArm {
  //TODO decode_arm
  ExecArm::Panic
}

pub(crate) fn decode_thumb(instr: u32) -> ExecThumb {
  if instr.get_bits(11..=15) == 0b00011 {
    ExecThumb::AddSub {
      op: instr.get_bit(9),
      imm: instr.get_bit(10),
      rn: instr.get_bits(6..=7) as u8,
    }
  } else if instr.get_bits(13..=15) == 0 {
    ExecThumb::Shifted {
      op: instr.get_bits(11..=12) as u8,
      offset: instr.get_bits(6..=10) as u8,
    }
  } else {
    ExecThumb::Panic
  }
}
