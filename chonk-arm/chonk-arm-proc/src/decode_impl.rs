use crate::{ExecArm, ExecThumb};

pub(crate) fn decode_arm(instr: u32) -> ExecArm {
  ExecArm::Panic
}

pub(crate) fn decode_thumb(instr: u32) -> ExecThumb {
  ExecThumb::Panic
}
