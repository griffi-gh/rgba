use crate::cpu::Cpu;

pub fn panic(cpu: &mut Cpu, instr: u32) {
  panic!("panic handler; pc: {:08x}, instr {instr:08x}", cpu.reg.pc())
}
