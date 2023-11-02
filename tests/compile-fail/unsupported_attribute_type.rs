#![no_implicit_prelude]

use ::enum_tools::EnumTools;

#[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
#[enum_tools(literal = "x", sorted(y=(z)))]
#[enum_tools = "bad"]
#[repr(i8)]
pub(crate) enum EG {
    #[enum_tools(rename = "A*")]
    A,
    #[enum_tools = "rename"]
    B,
    #[enum_tools(other = "A*")]
    C,
    #[enum_tools(rename(y=z))]
    D,
}

fn main() {}
