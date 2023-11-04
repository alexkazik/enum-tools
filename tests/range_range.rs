include!("macro/macro.rs");

setup_g!(#[enum_tools(iter(mode = "range"), range)];);

#[test]
fn range_range() {
    use eg::EG;
    assert_eq!(
        EG::range(EG::A, EG::D).collect::<Vec<_>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
    let _ = EG::iter(); // to silence dead-code warning
}
