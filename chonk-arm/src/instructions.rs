use crate::cpu::Cpu;

mod decode;
mod exec;

pub type ArmInstrHandler = fn(&mut Cpu, u32);
pub type ThumbInstrHandler = fn(&mut Cpu, u32);
