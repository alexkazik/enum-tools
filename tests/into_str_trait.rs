include!("macro/macro.rs");

setup!(#[enum_tools(IntoStr)];);

#[test]
fn into_str_trait_gapless() {
    use eg::EG;
    assert_eq!(<EG as Into<&'static str>>::into(EG::A), "A*");
    assert_eq!(<EG as Into<&'static str>>::into(EG::B), "B");
    assert_eq!(<EG as Into<&str>>::into(EG::C), "C");
    assert_eq!(<EG as Into<&str>>::into(EG::D), "D");
}

#[test]
fn into_str_trait_with_holes() {
    use eh::EH;
    assert_eq!(<EH as Into<&'static str>>::into(EH::A), "A*");
    assert_eq!(<EH as Into<&'static str>>::into(EH::B), "B");
    assert_eq!(<EH as Into<&str>>::into(EH::C), "C");
    assert_eq!(<EH as Into<&str>>::into(EH::D), "D");
}
