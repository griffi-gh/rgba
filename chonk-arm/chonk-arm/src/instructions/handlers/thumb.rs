use proc_bitfield::BitRange;
use crate::cpu::Cpu;

#[allow(clippy::needless_pass_by_ref_mut)]
pub fn panic(cpu: &mut Cpu, instr: u16) {
  panic!("panic handler; pc: {:08x}, instr {instr:04x}", cpu.reg.pc())
}

pub fn add_sub<
  const SUB: bool,
  const IMM: bool,
  const RN: u8,
>(cpu: &mut Cpu, instr: u16) {
  let rd: u8 = instr.bit_range::<0, 3>();
  let rs: u8 = instr.bit_range::<3, 6>();

  let lhs = cpu.reg.get(rs);
  let rhs = if IMM { RN as u32 } else { cpu.reg.get(RN) };

  let mut flags = cpu.reg.cpsr().flags();
  let result = if SUB {
    flags.alu_sub(lhs, rhs)
  } else {
    flags.alu_add(lhs, rhs)
  };
  cpu.reg.cpsr_mut().set_flags(flags);
  *cpu.reg.get_mut(rd) = result;
}
