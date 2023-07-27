use crate::cpu::Cpu;

pub fn exec_panic(cpu: &mut Cpu, _: u32) {
  panic!("panic handler; pc: {:08x}", cpu.reg.pc())
}

pub fn exec_test_bool<const _X: bool, const _Y: bool>(cpu: &mut Cpu, _: u32) {
  panic!("panic handler; pc: {:08x}", cpu.reg.pc())
}
