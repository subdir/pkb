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


macro_rules! with_variants_array {
    (
        $( #[$enum_attr:meta] )*
        pub enum $name:ident { $( $variant:ident ),* }
    ) => {
        use std;

        $( #[$enum_attr] )*
        pub enum  $name {
            $( $variant, )*
        }

        impl $name {
            pub const VARIANTS_NUM: usize = arity!($($variant),*);
            pub const VARIANTS: [$name; arity!($($variant),*)] = [ $( $name::$variant, )* ];
        }
    };
}


macro_rules! inverted_match_constructor {
    (
        fn $inverted:ident ($res: ty) -> Self {}
        fn $funcname:ident (&$arg: ident) -> $res2:ty {
            match *$var:ident {
                $( $variant:path => $map:expr ),*
            }
        }
    ) => {
        fn $funcname (&$arg) -> $res {
            match *$var {
                $( $variant => $map, )*
            }
        }
        fn $inverted(from: $res) -> Self {
            match from {
                $( $map => $variant, )*
                _ => panic!("No variant for {:?}", from)
            }
        }
    };
    (
        pub fn $inverted:ident ($res: ty) -> Self {}
        pub fn $funcname:ident (&$arg: ident) -> $res2:ty {
            match *$var:ident {
                $( $variant:path => $map:expr ),*
            }
        }
    ) => {
        pub fn $funcname (&$arg) -> $res {
            match *$var {
                $( $variant => $map, )*
            }
        }
        pub fn $inverted(from: $res) -> Self {
            match from {
                $( $map => $variant, )*
                _ => panic!("No variant for {:?}", from)
            }
        }
    };
}

