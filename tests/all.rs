mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[allow(dead_code)]
    #[derive(EnumTools, Clone, Copy, Eq, PartialEq)]
    // everything with an auto mode
    #[enum_tools(as_str, FromStr, from_str, iter)]
    // everything else
    #[enum_tools(Debug, Display, Into, into, MAX, MIN)]
    #[enum_tools(names, next, next_back, range, TryFrom, try_from)]
    #[repr(i8)]
    pub(crate) enum EG {
        #[enum_tools(rename = "A*")]
        A,
        B,
        C,
        D,
    }
}

mod eh {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[allow(dead_code)]
    #[derive(EnumTools, Clone, Copy, Eq, PartialEq)]
    // everything with an auto mode
    #[enum_tools(as_str, FromStr, from_str, iter)]
    // everything else
    #[enum_tools(Debug, Display, Into, into, IntoStr, MAX, MIN)]
    #[enum_tools(names, next, next_back, range, TryFrom, try_from)]
    #[repr(i8)]
    pub(crate) enum EH {
        #[enum_tools(rename = "A*")]
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

// fix while https://github.com/rust-lang/rust/issues/102190 is not fixed
#[allow(dead_code)]
fn no_op() {
    eg::EG::A.into();
    eh::EH::A.into();
}
