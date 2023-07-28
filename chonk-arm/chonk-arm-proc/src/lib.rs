use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

mod decode_impl;

use decode_impl::{decode_arm, decode_thumb};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum ExecArm {
  #[default]
  Panic,
}

impl ExecArm {
  pub fn token(&self) -> TokenStream2 {
    #[allow(unreachable_patterns)]
    match self {
      Self::Panic => quote!(orbit::handlers::arm::panic),
    }
  }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum ExecThumb {
  #[default]
  Panic,
  AddSub {
    sub: bool,
    imm: bool,
    rn: u8
  }
}

impl ExecThumb {
  pub fn token(self) -> TokenStream2 {
    #[allow(unreachable_patterns)]
    match self {
      Self::Panic => quote!(orbit::handlers::thumb::panic),
      Self::AddSub { sub, imm, rn} => quote!(orbit::handlers::thumb::add_sub::<#sub, #imm, #rn>),
    }
  }
}

#[proc_macro]
pub fn arm_lut(_: TokenStream) -> TokenStream {
  let fns: Vec<TokenStream2> = (0..4096)
    .map(|x| decode_arm(x).token())
    .collect();

  quote!(
    {
      use crate::orbit;
      const VALUE: [orbit::ArmInstrHandler; 4096] = [#(#fns),*];
      VALUE
    }
  ).into()
}

#[proc_macro]
pub fn thumb_lut(_: TokenStream) -> TokenStream {
  let fns: Vec<TokenStream2> = (0..1024)
    .map(|x| decode_thumb(x << 6).token())
    .collect();

  quote!(
    {
      use crate::orbit;
      const VALUE: [orbit::ThumbInstrHandler; 1024] = [#(#fns),*];
      VALUE
    }
  ).into()
}
