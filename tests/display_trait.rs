mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(Display)]
    #[repr(i8)]
    pub(crate) enum EG {
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
    #[enum_tools(Display)]
    #[repr(i8)]
    pub(crate) enum EH {
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

#[test]
fn display_trait_gapless() {
    use eg::EG;
    assert_eq!(format!("{}", EG::A), "A");
    assert_eq!(format!("{}", EG::B), "B");
    assert_eq!(format!("{}", EG::C), "C");
    assert_eq!(format!("{}", EG::D), "D");
}

#[test]
fn display_trait_with_holes() {
    use eh::EH;
    assert_eq!(format!("{}", EH::A), "A");
    assert_eq!(format!("{}", EH::B), "B");
    assert_eq!(format!("{}", EH::C), "C");
    assert_eq!(format!("{}", EH::D), "D");
}
