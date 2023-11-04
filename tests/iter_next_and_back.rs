include!("macro/macro.rs");

setup!(#[enum_tools(iter(mode = "next_and_back"))];);

#[test]
fn iter_next_and_back_gapless() {
    use eg::EG;
    let it = EG::iter();
    assert_eq!(it.len(), 4);
    assert_eq!(it.collect::<Vec<_>>(), vec![EG::A, EG::B, EG::C, EG::D]);
}

#[test]
fn iter_next_and_back_with_holes() {
    use eh::EH;
    let it = EH::iter();
    assert_eq!(it.len(), 4);
    assert_eq!(it.collect::<Vec<_>>(), vec![EH::A, EH::D, EH::C, EH::B]);
}
