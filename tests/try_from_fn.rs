include!("macro/macro.rs");

setup!(#[enum_tools(try_from)];);

#[test]
fn try_from_fn_gapless() {
    use eg::EG;
    assert_eq!(EG::try_from(0), Some(EG::A));
    assert_eq!(EG::try_from(1), Some(EG::B));
    assert_eq!(EG::try_from(2), Some(EG::C));
    assert_eq!(EG::try_from(3), Some(EG::D));
    assert_eq!(
        (i8::MIN..=i8::MAX)
            .filter_map(EG::try_from)
            .collect::<Vec<_>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
}

#[test]
fn try_from_fn_with_holes() {
    use eh::EH;
    assert_eq!(EH::try_from(0), Some(EH::A));
    assert_eq!(EH::try_from(9), Some(EH::B));
    assert_eq!(EH::try_from(2), Some(EH::C));
    assert_eq!(EH::try_from(1), Some(EH::D));
    assert_eq!(
        (i8::MIN..=i8::MAX)
            .filter_map(EH::try_from)
            .collect::<Vec<_>>(),
        vec![EH::A, EH::D, EH::C, EH::B]
    );
}
