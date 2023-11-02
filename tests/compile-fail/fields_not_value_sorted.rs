#![no_implicit_prelude]

use ::enum_tools::EnumTools;

#[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
#[enum_tools(sorted(value))]
#[repr(i8)]
pub(crate) enum EG {
    #[enum_tools(rename = "A*")]
    A = 0,
    B = 1,
    C = 2,
    D = -3,
}

fn main() {}
