mod cond;
mod reg;
mod mode;

pub mod Cpu {
    
}

//for asm debug purposes
#[inline(never)]
#[no_mangle]
pub fn _with_mode_(x: reg::gpr::Gpr, y: mode::Mode) -> reg::gpr::ModeGpr {
  reg::gpr::Gpr::with_mode(x, y)
}
