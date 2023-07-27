macro_rules! lut {
  {
    [$ty:ident; $size:literal] => [$(
      $fn:ident $(<$($generic: ty),*>)* : $d_fn:ident
    $(,)*),*]
  } => {
    {
      use ::seq_macro::seq;
      use ::paste::paste;

      const VALUE: [$ty; $size] = {
        seq!(N in 0..$size {
          [#({
            let mut x = 0;
            paste! {
              $(
                const (DID_MATCH, [<L_ $fn:upper>]): (bool, $ty) = {
                  const DECODE_RESULT: (bool, $($($generic),*)*) = $d_fn::<N>();
                  (DECODE_RESULT.0, exec_panic)
                };
              )*
              // $(
              //   const DECODE_RESULT: ($($($generic),*)*,) = (false,);
              //   const [<L_ $fn:upper>]: $ty = $fn::<seq!(G in 0..lut!(count_generics $($($generic),*)*)) {
              //     DECODE_RESULT.G
              //   })>;
              // )*
            }

            
          },)*]
        })
      };

      VALUE
    }
  };

  (_count_tokens) => (0_usize);
  (_count_tokens $x:tt $($s: tt)*) => (1usize + lut!(_count_tokens $($s)*));
}

const _X: super::ArmInstrHandler = {
  const _Y: super::ArmInstrHandler = super::exec::exec_panic;
  _Y
};

mod lut {
  use super::{lut, super::{ArmInstrHandler, exec::*}};
  
  const fn decode_test_bool<const I: u32>() -> (bool, bool) {
    (false, false)
  }
  
  const ARM_LUT: [ArmInstrHandler; 1] = lut!{
    [ArmInstrHandler; 1] => [
      exec_test_bool<bool>: decode_test_bool
    ]
  };
}
