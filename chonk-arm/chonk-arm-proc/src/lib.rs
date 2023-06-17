extern crate proc_macro;
use proc_macro::TokenStream;

//TODO

#[proc_macro]
pub fn generate_arm_lut(_: TokenStream) -> TokenStream {
  "fn answer() -> u32 { 42 }".parse().unwrap()
}
