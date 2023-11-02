#![no_implicit_prelude]

use ::enum_tools::EnumTools;

#[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum EG {
    #[enum_tools(rename = "A*")]
    A,
    C,
    B,
    D,
}

fn main() {}
