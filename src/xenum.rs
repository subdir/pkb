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
        pub enum $name:ident { $( $variant:ident ),* }
    ) => {
        use std;

        $( #[$enum_attr] )*
        pub enum  $name { $( $variant, )* }

        impl $name {
            pub const fn variants_num() -> usize {
                arity!($($variant),*)
            }
            pub fn variants() -> &'static [$name; arity!($($variant),*)] {
                static VARIANTS: [$name; arity!($($variant),*)] = [ $( $name::$variant, )* ];
                &VARIANTS
            }
            fn id(self) -> u8 {
                unsafe { std::mem::transmute(self) }
            }
            fn from_id(id: u8) -> Self {
                if (id as usize) < Self::variants_num() {
                    unsafe { std::mem::transmute(id) }
                } else {
                    panic!(format!("Bad id for {}: {:?}", stringify!($name), id))
                }
            }
        }
    };
}


macro_rules! inverted_match_constructor {
    (
        pub fn $inverted:ident ($res: ty) -> Self {}
        pub fn $funcname:ident ($arg: ident) -> $res2:ty {
            match $var:ident {
                $( $variant:path => $map:expr ),*
            }
        }
    ) => {
        pub fn $funcname ($arg) -> $res {
            match $var {
                $( $variant => $map, )*
            }
        }
        pub fn $inverted(from: $res) -> Self {
            match from {
                $( $map => $variant, )*
                _ => panic!(format!("No variant for {:?}", from))
            }
        }
    }
}



