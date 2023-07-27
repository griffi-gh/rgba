macro_rules! lut {
  {
    [$ty:ident; $size:literal] => [$(
      $fn:ident $(<$($index: literal $generic: ty),*>)* : $decode_fn:ident
    $(,)*),*]
  } => {
    
    {
      use ::seq_macro::seq;

      #[allow(unused)]
      const fn always<const I: u8>() -> (bool,) { (true,) }

      const VALUE: [$ty; $size] = {
        seq!(N in 0..$size {
          [#({
            let mut resolv: Option<$ty> = None;
            $(
              #[allow(clippy::redundant_pattern_matching)]
              if matches!(resolv, None) {
                const DECODE_RESULT: (bool, $(($($generic),*,))*) = $decode_fn::<{N as _}>();
                
                if (DECODE_RESULT.0) {
                  resolv = Some(
                    $fn::<$($(
                      {
                        stringify!($generic);
                        DECODE_RESULT.1.$index
                      },
                    )*)*>
                  );
                }
              }
            )*
            match resolv {
              Some(x) => x,
              None => panic!(""),
            }
          },)*]
        })
      };

      VALUE
    }
  };

  // (_count_tokens) => (0_usize);
  // (_count_tokens $x:tt $($s: tt)*) => (1usize + lut!(_count_tokens $($s)*));
}

mod lut {
  use super::{lut, super::{ArmInstrHandler, exec::*}};

  const fn decode_test_bool<const I: u32>() -> (bool, (bool, bool)) {
    (false, (false, false))
  }
  
  const ARM_LUT: [ArmInstrHandler; 1] = lut!{
    [ArmInstrHandler; 1] => [
      exec_test_bool<0 bool, 1 bool>: decode_test_bool,
      exec_panic: always
    ]
  };
}
