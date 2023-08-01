use bit_field::BitField;
use crate::{ArmHandler, ThumbHandler};

pub(crate) fn decode_arm(_instr: u32) -> ArmHandler {
  //TODO decode_arm
  ArmHandler::Panic
}

/*
 THUMB Binary Opcode Format
 (sauce: gbatek; some are arm9 only, we're obviously not handling them!)

 Form|_15|_14|_13|_12|_11|_10|_9_|_8_|_7_|_6_|_5_|_4_|_3_|_2_|_1_|_0_|
 __1_|_0___0___0_|__Op___|_______Offset______|____Rs_____|____Rd_____|Shifted
 __2_|_0___0___0___1___1_|_I,_Op_|___Rn/nn___|____Rs_____|____Rd_____|ADD/SUB
 __3_|_0___0___1_|__Op___|____Rd_____|_____________Offset____________|Immedi.
 __4_|_0___1___0___0___0___0_|______Op_______|____Rs_____|____Rd_____|AluOp
 __5_|_0___1___0___0___0___1_|__Op___|Hd_|Hs_|____Rs_____|____Rd_____|HiReg/BX
 __6_|_0___1___0___0___1_|____Rd_____|_____________Word______________|LDR PC
 __7_|_0___1___0___1_|__Op___|_0_|___Ro______|____Rb_____|____Rd_____|LDR/STR
 __8_|_0___1___0___1_|__Op___|_1_|___Ro______|____Rb_____|____Rd_____|""H/SB/SH
 __9_|_0___1___1_|__Op___|_______Offset______|____Rb_____|____Rd_____|""{B}
 _10_|_1___0___0___0_|Op_|_______Offset______|____Rb_____|____Rd_____|""H
 _11_|_1___0___0___1_|Op_|____Rd_____|_____________Word______________|"" SP
 _12_|_1___0___1___0_|Op_|____Rd_____|_____________Word______________|ADD PC/SP
 _13_|_1___0___1___1___0___0___0___0_|_S_|___________Word____________|ADD SP,nn
 _14_|_1___0___1___1_|Op_|_1___0_|_R_|____________Rlist______________|PUSH/POP
 _17_|_1___0___1___1___1___1___1___0_|___________User_Data___________|BKPT ARM9
 _15_|_1___1___0___0_|Op_|____Rb_____|____________Rlist______________|STM/LDM
 _16_|_1___1___0___1_|_____Cond______|_________Signed_Offset_________|B{cond}
 _U__|_1___1___0___1___1___1___1___0_|_____________var_______________|UndefARM9
 _17_|_1___1___0___1___1___1___1___1_|___________User_Data___________|SWI
 _18_|_1___1___1___0___0_|________________Offset_____________________|B
 _19_|_1___1___1___0___1_|_________________________var___________|_0_|BLX.ARM9
 _U__|_1___1___1___0___1_|_________________________var___________|_1_|UndefARM9
 _19_|_1___1___1___1_|_H_|______________Offset_Low/High______________|BL,BLX
*/

pub(crate) fn decode_thumb(instr: u16) -> ThumbHandler {
  if instr.get_bits(11..=15) == 0b00011 {
    ThumbHandler::AddSub {
      op: instr.get_bit(9),
      imm: instr.get_bit(10),
      rn: instr.get_bits(6..=7) as u8,
    }
  } else if instr.get_bits(13..=15) == 0b000 {
    ThumbHandler::Shifted {
      op: instr.get_bits(11..=12) as u8,
      offset: instr.get_bits(6..=10) as u8,
    }
  } else if instr.get_bits(13..=15) == 0b001 {
    ThumbHandler::Immediate {
      op: instr.get_bits(11..=12) as u8,
      rd: instr.get_bits(8..=10) as u8,
    }
  } else if instr.get_bits(10..=15) == 0b010000 {
    ThumbHandler::AluOp {
      op: instr.get_bits(6..=9) as u8,
    }
  } else {
    ThumbHandler::Panic
  }
}
