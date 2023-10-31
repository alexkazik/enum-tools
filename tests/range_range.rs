mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(iter(mode = "range"), range)]
    #[repr(i8)]
    pub(crate) enum EG {
        #[enum_tools(rename = "A*")]
        A,
        B,
        C,
        D,
    }
}

#[test]
fn range_range() {
    use eg::EG;
    assert_eq!(
        EG::range(EG::A, EG::D).collect::<Vec<_>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
}
