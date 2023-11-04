include!("macro/macro.rs");

setup!(#[enum_tools(MIN, MAX)];);
#[test]
fn min_max_gapless() {
    use eg::EG;
    assert_eq!(EG::MIN, EG::A);
    let _ = EG::B; // to silence dead-code warning
    let _ = EG::C; // to silence dead-code warning
    assert_eq!(EG::MAX, EG::D);
}

#[test]
fn min_max_with_holes() {
    use eh::EH;
    assert_eq!(EH::MIN, EH::A);
    assert_eq!(EH::MAX, EH::B);
    let _ = EH::C; // to silence dead-code warning
    let _ = EH::D; // to silence dead-code warning
}
