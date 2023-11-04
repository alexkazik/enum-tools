include!("macro/macro.rs");

setup!(#[enum_tools(Display)];);

#[test]
fn display_trait_gapless() {
    use eg::EG;
    assert_eq!(format!("{}", EG::A), "A*");
    assert_eq!(format!("{}", EG::B), "B");
    assert_eq!(format!("{}", EG::C), "C");
    assert_eq!(format!("{}", EG::D), "D");
}

#[test]
fn display_trait_with_holes() {
    use eh::EH;
    assert_eq!(format!("{}", EH::A), "A*");
    assert_eq!(format!("{}", EH::B), "B");
    assert_eq!(format!("{}", EH::C), "C");
    assert_eq!(format!("{}", EH::D), "D");
}
