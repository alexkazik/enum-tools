include!("macro/macro.rs");

setup!(#[enum_tools(Into)];);

#[test]
fn into_trait_gapless() {
    use eg::EG;
    assert_eq!(i8::from(EG::A), 0);
    assert_eq!(i8::from(EG::B), 1);
    assert_eq!(i8::from(EG::C), 2);
    assert_eq!(i8::from(EG::D), 3);
}

#[test]
fn into_trait_with_holes() {
    use eh::EH;
    assert_eq!(i8::from(EH::A), 0);
    assert_eq!(i8::from(EH::B), 9);
    assert_eq!(i8::from(EH::C), 2);
    assert_eq!(i8::from(EH::D), 1);
}
