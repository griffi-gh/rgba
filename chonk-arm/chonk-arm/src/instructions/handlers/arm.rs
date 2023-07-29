use crate::cpu::Cpu;

#[allow(clippy::needless_pass_by_ref_mut)]
pub fn panic(cpu: &mut Cpu, instr: u32) {
  panic!("panic handler; pc: {:08x}, instr {instr:08x}", cpu.reg.pc())
}
