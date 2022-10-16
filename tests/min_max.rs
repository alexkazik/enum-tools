mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[allow(dead_code)]
    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(MIN, MAX)]
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

    #[allow(dead_code)]
    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(MIN, MAX)]
    #[repr(i8)]
    pub(crate) enum EH {
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

#[test]
fn min_max_gapless() {
    use eg::EG;
    assert_eq!(EG::MIN, EG::A);
    assert_eq!(EG::MAX, EG::D);
}

#[test]
fn min_max_with_holes() {
    use eh::EH;
    assert_eq!(EH::MIN, EH::A);
    assert_eq!(EH::MAX, EH::B);
}
