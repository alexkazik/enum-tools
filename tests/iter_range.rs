mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(iter(mode = "range"))]
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
fn iter_range() {
    use eg::EG;
    let it = EG::iter();
    assert_eq!(it.len(), 4);
    assert_eq!(it.collect::<Vec<_>>(), vec![EG::A, EG::B, EG::C, EG::D]);
}
