use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

mod decode_impl;

use decode_impl::{decode_arm, decode_thumb};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum ArmHandler {
  #[default]
  Panic,
}

impl ArmHandler {
  pub fn token(&self) -> TokenStream2 {
    #[allow(unreachable_patterns)]
    match self {
      Self::Panic => quote!(orbit::handlers::arm::panic),
    }
  }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum ThumbHandler {
  #[default]
  Panic,
  Shifted {
    op: u8,
    offset: u8,
  },
  AddSub {
    op: bool,
    imm: bool,
    rn: u8
  }
}

impl ThumbHandler {
  pub fn token(self) -> TokenStream2 {
    #[allow(unreachable_patterns)]
    match self {
      Self::Shifted { op, offset } => quote!(orbit::handlers::thumb::shifted::<#op, #offset>),
      Self::AddSub { op, imm, rn} => quote!(orbit::handlers::thumb::add_sub::<#op, #imm, #rn>),
      Self::Panic => quote!(orbit::handlers::thumb::panic),
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
      use crate::_orbit as orbit;
      const VALUE: [orbit::ArmHandlerFn; 4096] = [#(#fns),*];
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
      use crate::_orbit as orbit;
      const VALUE: [orbit::ThumbHandlerFn; 1024] = [#(#fns),*];
      VALUE
    }
  ).into()
}
