mod eg {
    #![no_implicit_prelude]

    use ::enum_tools::EnumTools;

    #[allow(dead_code)]
    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(names)]
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

    #[allow(dead_code)]
    #[derive(EnumTools, Debug, Clone, Copy, Eq, PartialEq)]
    #[enum_tools(names)]
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
fn iter_names_gapless() {
    use eg::EG;
    assert_eq!(EG::names().collect::<Vec<_>>(), vec!["A*", "B", "C", "D"]);
}

#[test]
fn iter_names_with_holes() {
    use eh::EH;
    assert_eq!(EH::names().collect::<Vec<_>>(), vec!["A*", "D", "C", "B"]);
}
