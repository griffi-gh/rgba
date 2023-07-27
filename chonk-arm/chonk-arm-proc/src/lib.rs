use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

mod decode_impl;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum Exec {
  #[default]
  Panic
}
impl Exec {
  pub fn tok(&self) -> TokenStream2 {
    #[allow(unreachable_patterns)]
    match self {
      Self::Panic => quote!(exec::exec_panic),
      _ => unimplemented!("Instruction handler for {self:?} is not implemented")
    }
  }
}

#[proc_macro]
pub fn generate_arm_lut(_: TokenStream) -> TokenStream {
  "fn answer() -> u32 { 42 }".parse().unwrap()
}
