use chonk_arm_proc::{arm_lut, thumb_lut};
use crate::cpu::Cpu;

pub mod handlers;

pub type ArmInstrHandler = fn(&mut Cpu, u32);
pub type ThumbInstrHandler = fn(&mut Cpu, u16);

pub const ARM_LUT: [ArmInstrHandler; 4096] = arm_lut!();
pub const THUMB_LUT: [ThumbInstrHandler; 1024] = thumb_lut!();
