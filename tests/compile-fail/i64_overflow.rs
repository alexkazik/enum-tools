#![no_implicit_prelude]

use ::enum_tools::EnumTools;

#[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u64)]
pub(crate) enum EG {
    #[enum_tools(rename = "A*")]
    A = 0x7fff_ffff_ffff_fffe,
    B,
    C,
    D,
}

fn main() {}
