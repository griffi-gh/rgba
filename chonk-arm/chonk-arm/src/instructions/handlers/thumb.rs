use proc_bitfield::BitRange;
use crate::cpu::Cpu;

#[allow(clippy::needless_pass_by_ref_mut)]
pub fn panic(cpu: &mut Cpu, instr: u16) {
  panic!("panic handler; pc: {:08x}, instr {instr:04x}", cpu.reg.pc())
}

/// format 1  \
/// move reg shifted \
/// `reg[rd] = reg[rs] shifted (shift op = OP) by OFFSET`
pub fn shifted<
  const OP: u8,
  const OFFSET: u8,
>(cpu: &mut Cpu, instr: u16) {
  todo!("shifted/barrel shifter stuff")
}

/// format 2 \
/// add/sub \
/// `reg[rd] = reg[rs] OP?[-]:[+] IMM?(rn):(reg[rn])`
pub fn add_sub<
  const OP: bool,
  const IMM: bool,
  const RN: u8,
>(cpu: &mut Cpu, instr: u16) {
  let rd: u8 = instr.bit_range::<0, 3>();
  let rs: u8 = instr.bit_range::<3, 6>();

  let lhs = cpu.reg.get(rs);
  let rhs = if IMM { RN as u32 } else { cpu.reg.get(RN) };

  let mut flags = cpu.reg.cpsr().flags();
  let result = if OP {
    flags.alu_sub(lhs, rhs)
  } else {
    flags.alu_add(lhs, rhs)
  };
  cpu.reg.cpsr_mut().set_flags(flags);
  *cpu.reg.get_mut(rd) = result;
}

/// format 3 \
/// data process immediate
pub fn immediate<
  const OP: u8,
  const RD: u8,
>(cpu: &mut Cpu, instr: u16) {
  todo!("data process immediate")
}

/// format 4 \
/// alu operations
pub fn alu_op<
  const OP: u8,
>(cpu: &mut Cpu, instr: u16) {
  todo!("alu_op")
}

/// format 5 \
/// hi reg operation/bx
pub fn hi_reg_bx<
  const OP: u8,
  const HD: bool,
  const HS: bool,
>(cpu: &mut Cpu, instr: u16) {
  todo!("hi_reg_bx")
}

/// format 6 \
pub fn ldr_pc<
  const RD: u8,
>(cpu: &mut Cpu, instr: u16) {
  todo!("ldr_pc")
}

/// format 7 \
/// ldr/str reg offs.
pub fn ldr_str<
  const LOAD: bool,
  const BYTE: bool,
  const RO: u8,
>(cpu: &mut Cpu, instr: u16) {
  todo!("ldr_str")
}
