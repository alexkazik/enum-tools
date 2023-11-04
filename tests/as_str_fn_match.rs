include!("macro/macro.rs");

setup!(#[enum_tools(as_str(mode = "match"))];);

#[test]
fn as_str_fn_match_gapless() {
    use eg::EG;
    assert_eq!(EG::A.as_str(), "A*");
    assert_eq!(EG::B.as_str(), "B");
    assert_eq!(EG::C.as_str(), "C");
    assert_eq!(EG::D.as_str(), "D");
}

#[test]
fn as_str_fn_match_with_holes() {
    use eh::EH;
    assert_eq!(EH::A.as_str(), "A*");
    assert_eq!(EH::B.as_str(), "B");
    assert_eq!(EH::C.as_str(), "C");
    assert_eq!(EH::D.as_str(), "D");
}
