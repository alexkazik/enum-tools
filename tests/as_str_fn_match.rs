mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(as_str(mode = "match"))]
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
    #[enum_tools(as_str(mode = "match"))]
    #[repr(i8)]
    pub(crate) enum EH {
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

#[test]
fn as_str_match_gapless() {
    use eg::EG;
    assert_eq!(EG::A.as_str(), "A");
    assert_eq!(EG::B.as_str(), "B");
    assert_eq!(EG::C.as_str(), "C");
    assert_eq!(EG::D.as_str(), "D");
}

#[test]
fn as_str_match_with_holes() {
    use eh::EH;
    assert_eq!(EH::A.as_str(), "A");
    assert_eq!(EH::B.as_str(), "B");
    assert_eq!(EH::C.as_str(), "C");
    assert_eq!(EH::D.as_str(), "D");
}
