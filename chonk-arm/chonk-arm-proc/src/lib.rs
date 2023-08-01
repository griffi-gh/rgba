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
  pub fn to_fn_token_stream(self) -> TokenStream2 {
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
  },
  Immediate {
    op: u8,
    rd: u8,
  },
  AluOp {
    op: u8,
  },
}

impl ThumbHandler {
  pub fn to_fn_token_stream(self) -> TokenStream2 {
    #[allow(unreachable_patterns)]
    match self {
      Self::Panic => quote!(orbit::handlers::thumb::panic),
      Self::Shifted { op, offset } => quote!(orbit::handlers::thumb::shifted::<#op, #offset>),
      Self::AddSub { op, imm, rn} => quote!(orbit::handlers::thumb::add_sub::<#op, #imm, #rn>),
      Self::Immediate { op, rd } => quote!(orbit::handlers::thumb::immediate::<#op, #rd>),
      Self::AluOp { op } => quote!(orbit::handlers::thumb::alu_op::<#op>),
    }
  }
}

#[proc_macro]
pub fn arm_lut(_: TokenStream) -> TokenStream {
  let fns: Vec<TokenStream2> = (0..4096)
    .map(|x| decode_arm(x).to_fn_token_stream())
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
    .map(|x| decode_thumb(x).to_fn_token_stream())
    .collect();

  quote!(
    {
      use crate::_orbit as orbit;
      const VALUE: [orbit::ThumbHandlerFn; 1024] = [#(#fns),*];
      VALUE
    }
  ).into()
}
