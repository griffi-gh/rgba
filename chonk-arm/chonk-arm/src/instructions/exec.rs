use crate::cpu::Cpu;
  
pub fn exec_panic(cpu: &mut Cpu) {
  panic!("panic handler; pc: {:08x}", cpu.reg.pc())
}
