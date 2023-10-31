mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(from_str(mode = "table"))]
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
    #[enum_tools(from_str(mode = "table"))]
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
fn from_str_fn_table_gapless() {
    use eg::EG;
    assert_eq!(EG::from_str("A*"), Some(EG::A));
    assert_eq!(EG::from_str("B"), Some(EG::B));
    assert_eq!(EG::from_str("C"), Some(EG::C));
    assert_eq!(EG::from_str("D"), Some(EG::D));
    assert_eq!(EG::from_str("E"), None);
}

#[test]
fn from_str_fn_table_with_holes() {
    use eh::EH;
    assert_eq!(EH::from_str("A*"), Some(EH::A));
    assert_eq!(EH::from_str("B"), Some(EH::B));
    assert_eq!(EH::from_str("C"), Some(EH::C));
    assert_eq!(EH::from_str("D"), Some(EH::D));
    assert_eq!(EH::from_str("E"), None);
}
