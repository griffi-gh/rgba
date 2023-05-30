use crate::cpu::Cpu;

pub type ArmInstrHandler = fn(&mut Cpu, u32);
pub type ThumbInstrHandler = fn(&mut Cpu, u32);

pub fn exec_panic(cpu: &mut Cpu) {
  
}