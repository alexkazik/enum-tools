mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(iter(mode = "table_inline"))]
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
    #[enum_tools(iter(mode = "table_inline"))]
    #[repr(i8)]
    pub(crate) enum EH {
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

#[test]
fn iter_table_inline_gapless() {
    use eg::EG;
    assert_eq!(
        EG::iter().collect::<Vec<_>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
}

#[test]
fn iter_table_inline_with_holes() {
    use eh::EH;
    assert_eq!(
        EH::iter().collect::<Vec<_>>(),
        vec![EH::A, EH::D, EH::C, EH::B]
    );
}
