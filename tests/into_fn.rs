mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(into)]
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
    #[enum_tools(into)]
    #[repr(i8)]
    pub(crate) enum EH {
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

#[test]
fn to_fn_gapless() {
    use eg::EG;
    assert_eq!(EG::A.into(), 0);
    assert_eq!(EG::B.into(), 1);
    assert_eq!(EG::C.into(), 2);
    assert_eq!(EG::D.into(), 3);
}

#[test]
fn to_fn_with_holes() {
    use eh::EH;
    assert_eq!(EH::A.into(), 0);
    assert_eq!(EH::B.into(), 9);
    assert_eq!(EH::C.into(), 2);
    assert_eq!(EH::D.into(), 1);
}
