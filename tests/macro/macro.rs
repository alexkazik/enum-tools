// This should not be used in the compile-fail tests because the errors will point into this code.

#[allow(unused_macros)]
macro_rules! setup {
    ($( #[$($attrs:tt)*] )+;) => {
        setup_g!( $( #[$($attrs)*] )+ ;);
        setup_h!( $( #[$($attrs)*] )+ ;);
    };
}

macro_rules! setup_g {
    ($( #[$($attrs:tt)*] )+;) => {
        mod eg {
            #![no_implicit_prelude]

            use ::enum_tools::EnumTools;

            #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
            $( #[$($attrs)*] )+
            #[repr(i8)]
            pub(crate) enum EG {
                #[enum_tools(rename = "A*")]
                A,
                B,
                C,
                D,
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! setup_h {
    ($( #[$($attrs:tt)*] )+;) => {
        mod eh {
            #![no_implicit_prelude]

            use ::enum_tools::EnumTools;

            #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
            $( #[$($attrs)*] )+
            #[repr(i8)]
            pub(crate) enum EH {
                #[enum_tools(rename = "A*")]
                A = 0,
                B = 9,
                C = 2,
                D = 1,
            }
        }
    };
}
