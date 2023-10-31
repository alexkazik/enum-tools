mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(iter(mode = "next_and_back"), range)]
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
    #[enum_tools(iter(mode = "next_and_back"), range)]
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
fn range_next_and_back_gapless() {
    use eg::EG;
    assert_eq!(
        EG::range(EG::A, EG::D).collect::<Vec<_>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
}

#[test]
fn range_next_and_back_with_holes() {
    use eh::EH;
    assert_eq!(
        EH::range(EH::A, EH::B).collect::<Vec<_>>(),
        vec![EH::A, EH::D, EH::C, EH::B]
    );
}
