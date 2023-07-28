use crate::cpu::Cpu;

pub fn panic(cpu: &mut Cpu, instr: u16) {
  panic!("panic handler; pc: {:08x}, instr {instr:04x}", cpu.reg.pc())
}

pub fn add_sub<
  const SUB: bool,
  const IMM: bool,
  const RN: u8,
>(cpu: &mut Cpu, instr: u16) {
  todo!()
}
