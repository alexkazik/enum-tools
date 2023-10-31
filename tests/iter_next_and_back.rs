mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(iter(mode = "next_and_back"))]
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
    #[enum_tools(iter(mode = "next_and_back"))]
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
fn iter_next_and_back_gapless() {
    use eg::EG;
    let it = EG::iter();
    assert_eq!(it.len(), 4);
    assert_eq!(it.collect::<Vec<_>>(), vec![EG::A, EG::B, EG::C, EG::D]);
}

#[test]
fn iter_next_and_back_with_holes() {
    use eh::EH;
    let it = EH::iter();
    assert_eq!(it.len(), 4);
    assert_eq!(it.collect::<Vec<_>>(), vec![EH::A, EH::D, EH::C, EH::B]);
}
