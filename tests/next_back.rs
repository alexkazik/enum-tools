mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(MAX, next_back)]
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
    #[enum_tools(MAX, next_back)]
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
fn next_back_gapless() {
    use eg::EG;
    let mut cur = Some(EG::MAX);
    assert_eq!(
        ::core::iter::from_fn(|| {
            let res = cur;
            cur = cur.and_then(EG::next_back);
            res
        })
        .collect::<Vec<_>>(),
        vec![EG::D, EG::C, EG::B, EG::A]
    );
}

#[test]
fn next_back_with_holes() {
    use eh::EH;
    let mut cur = Some(EH::MAX);
    assert_eq!(
        ::core::iter::from_fn(|| {
            let res = cur;
            cur = cur.and_then(EH::next_back);
            res
        })
        .collect::<Vec<_>>(),
        vec![EH::B, EH::C, EH::D, EH::A]
    );
}
