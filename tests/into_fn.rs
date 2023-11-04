include!("macro/macro.rs");

setup!(#[enum_tools(into)];);

#[test]
fn into_fn_gapless() {
    use eg::EG;
    assert_eq!(EG::A.into(), 0);
    assert_eq!(EG::B.into(), 1);
    assert_eq!(EG::C.into(), 2);
    assert_eq!(EG::D.into(), 3);
}

#[test]
fn into_fn_with_holes() {
    use eh::EH;
    assert_eq!(EH::A.into(), 0);
    assert_eq!(EH::B.into(), 9);
    assert_eq!(EH::C.into(), 2);
    assert_eq!(EH::D.into(), 1);
}
