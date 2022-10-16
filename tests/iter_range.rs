mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(iter(mode = "range"))]
    #[repr(i8)]
    pub(crate) enum EG {
        A,
        B,
        C,
        D,
    }
}

#[test]
fn iter_range() {
    use eg::EG;
    assert_eq!(
        EG::iter().collect::<Vec<_>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
}
