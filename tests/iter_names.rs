include!("macro/macro.rs");

setup!(#[enum_tools(names)];);

#[test]
fn iter_names_gapless() {
    use eg::EG;
    assert_eq!(EG::names().collect::<Vec<_>>(), vec!["A*", "B", "C", "D"]);
    let _ = EG::A; // to silence dead-code warning
    let _ = EG::B; // to silence dead-code warning
    let _ = EG::C; // to silence dead-code warning
    let _ = EG::D; // to silence dead-code warning
}

#[test]
fn iter_names_with_holes() {
    use eh::EH;
    assert_eq!(EH::names().collect::<Vec<_>>(), vec!["A*", "D", "C", "B"]);
    let _ = EH::A; // to silence dead-code warning
    let _ = EH::B; // to silence dead-code warning
    let _ = EH::C; // to silence dead-code warning
    let _ = EH::D; // to silence dead-code warning
}
