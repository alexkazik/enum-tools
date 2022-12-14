mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(MIN, next)]
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
    #[enum_tools(MIN, next)]
    #[repr(i8)]
    pub(crate) enum EH {
        A = 0,
        B = 9,
        C = 2,
        D = 1,
    }
}

#[test]
fn next_gapless() {
    use eg::EG;
    let mut cur = Some(EG::MIN);
    assert_eq!(
        ::core::iter::from_fn(|| {
            let res = cur;
            cur = cur.and_then(EG::next);
            res
        })
        .collect::<Vec<_>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
}

#[test]
fn next_with_holes() {
    use eh::EH;
    let mut cur = Some(EH::MIN);
    assert_eq!(
        ::core::iter::from_fn(|| {
            let res = cur;
            cur = cur.and_then(EH::next);
            res
        })
        .collect::<Vec<_>>(),
        vec![EH::A, EH::D, EH::C, EH::B]
    );
}
