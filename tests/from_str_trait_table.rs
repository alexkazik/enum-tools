mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(FromStr(mode = "table"))]
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
    #[enum_tools(FromStr(mode = "table"))]
    #[repr(i8)]
    pub(crate) enum EH {
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

use core::str::FromStr;

#[test]
fn as_str_match_gapless() {
    use eg::EG;
    assert_eq!(EG::from_str("A"), Ok(EG::A));
    assert_eq!(EG::from_str("B"), Ok(EG::B));
    assert_eq!(EG::from_str("C"), Ok(EG::C));
    assert_eq!(EG::from_str("D"), Ok(EG::D));
    assert_eq!(EG::from_str("E"), Err(()));
}

#[test]
fn as_str_match_with_holes() {
    use eh::EH;
    assert_eq!(EH::from_str("A"), Ok(EH::A));
    assert_eq!(EH::from_str("B"), Ok(EH::B));
    assert_eq!(EH::from_str("C"), Ok(EH::C));
    assert_eq!(EH::from_str("D"), Ok(EH::D));
    assert_eq!(EH::from_str("E"), Err(()));
}
