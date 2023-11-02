#![no_implicit_prelude]

use ::enum_tools::EnumTools;

#[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
#[enum_tools(unknown1)]
#[repr(i8)]
pub(crate) enum EG {
    #[enum_tools(rename = "A*")]
    A,
    B,
    C,
    D,
}

fn main() {}
