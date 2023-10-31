mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(IntoStr)]
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

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(IntoStr)]
    #[repr(i8)]
    pub(crate) enum EH {
        #[enum_tools(rename = "A*")]
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

#[test]
fn into_str_trait_gapless() {
    use eg::EG;
    assert_eq!(<EG as Into<&'static str>>::into(EG::A), "A*");
    assert_eq!(<EG as Into<&'static str>>::into(EG::B), "B");
    assert_eq!(<EG as Into<&str>>::into(EG::C), "C");
    assert_eq!(<EG as Into<&str>>::into(EG::D), "D");
}

#[test]
fn into_str_trait_with_holes() {
    use eh::EH;
    assert_eq!(<EH as Into<&'static str>>::into(EH::A), "A*");
    assert_eq!(<EH as Into<&'static str>>::into(EH::B), "B");
    assert_eq!(<EH as Into<&str>>::into(EH::C), "C");
    assert_eq!(<EH as Into<&str>>::into(EH::D), "D");
}
