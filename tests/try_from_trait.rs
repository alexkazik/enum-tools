mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(TryFrom)]
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
    #[enum_tools(TryFrom)]
    #[repr(i8)]
    pub(crate) enum EH {
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

#[test]
fn try_from_gapless() {
    use eg::EG;
    assert_eq!(EG::try_from(0), Ok(EG::A));
    assert_eq!(EG::try_from(1), Ok(EG::B));
    assert_eq!(EG::try_from(2), Ok(EG::C));
    assert_eq!(EG::try_from(3), Ok(EG::D));
    assert_eq!(
        (i8::MIN..=i8::MAX)
            .filter_map(|v| TryFrom::try_from(v).ok())
            .collect::<Vec<EG>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
}

#[test]
fn try_from_with_holes() {
    use eh::EH;
    assert_eq!(EH::try_from(0), Ok(EH::A));
    assert_eq!(EH::try_from(9), Ok(EH::B));
    assert_eq!(EH::try_from(2), Ok(EH::C));
    assert_eq!(EH::try_from(1), Ok(EH::D));
    assert_eq!(
        (i8::MIN..=i8::MAX)
            .filter_map(|v| TryFrom::try_from(v).ok())
            .collect::<Vec<EH>>(),
        vec![EH::A, EH::D, EH::C, EH::B]
    );
}
