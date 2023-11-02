#![no_implicit_prelude]

use ::enum_tools::EnumTools;

#[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i8)]
pub(crate) enum EG {
    #[enum_tools(rename = "A*")]
    A = 0,
    B = 9,
    C = 2,
    D = 9,
}

fn main() {}
