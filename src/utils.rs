// https://github.com/rust-lang/rfcs/pull/88#issuecomment-109681279
macro_rules! arity {
    ($e:tt, $($toks:tt)+) => { 1 + arity!($($toks)+) };
    ($e:tt) => { 1 };

    ($e:pat, $($toks:tt)+) => { 1 + arity!($($toks)+) };
    ($e:pat) => { 1 };

    ($e:expr, $($toks:tt)+) => { 1 + arity!($($toks)+) };
    ($e:expr) => { 1 };

    () => { 0 };
}

macro_rules! xenum {
    (
        $( #[$enum_attr:meta] )*
        pub enum $name:ident { $( $variant:ident ),+ }
    ) => {
        use std;
        $( #[$enum_attr] )*
        pub enum  $name {
            $( $variant, )+
        }
        impl $name {
            fn variants_num() -> u8 {
                arity!($($variant),+)
            }
            fn id(self) -> u8 {
                unsafe { std::mem::transmute(self) }
            }
            fn from_id(id: u8) -> Self {
                if id < Self::variants_num() {
                    unsafe { std::mem::transmute(id) }
                } else {
                    panic!(format!("Bad id for {}: {:?}", stringify!($name), id))
                }
            }
        }
    };
}

