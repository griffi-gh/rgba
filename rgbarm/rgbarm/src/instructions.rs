use rgbarm_proc::{arm_lut, thumb_lut};
use crate::cpu::Cpu;

pub mod handlers;

pub type ArmHandlerFn = fn(&mut Cpu, u32);
pub type ThumbHandlerFn = fn(&mut Cpu, u16);

pub const ARM_LUT: [ArmHandlerFn; 4096] = arm_lut!();
pub const THUMB_LUT: [ThumbHandlerFn; 1024] = thumb_lut!();
