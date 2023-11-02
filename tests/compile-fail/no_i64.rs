#![no_implicit_prelude]

use ::enum_tools::EnumTools;

#[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u64)]
pub(crate) enum EG {
    #[enum_tools(rename = "A*")]
    A = 0,
    B = 9,
    C = 2,
    D = 0xf000_0000_0000_0000,
}

fn main() {}
