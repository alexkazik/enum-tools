include!("macro/macro.rs");

setup!(#[enum_tools(iter(mode = "table"), range)];);

#[test]
fn range_table_gapless() {
    use eg::EG;
    assert_eq!(
        EG::range(EG::A, EG::D).collect::<Vec<_>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
    let _ = EG::iter(); // to silence dead-code warning
}

#[test]
fn range_table_with_holes() {
    use eh::EH;
    assert_eq!(
        EH::range(EH::A, EH::B).collect::<Vec<_>>(),
        vec![EH::A, EH::D, EH::C, EH::B]
    );
    let _ = EH::iter(); // to silence dead-code warning
}
