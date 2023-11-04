include!("macro/macro.rs");

setup!(#[enum_tools(MAX, next_back)];);

#[test]
fn next_back_gapless() {
    use eg::EG;
    let mut cur = Some(EG::MAX);
    assert_eq!(
        ::core::iter::from_fn(|| {
            let res = cur;
            cur = cur.and_then(EG::next_back);
            res
        })
        .collect::<Vec<_>>(),
        vec![EG::D, EG::C, EG::B, EG::A]
    );
}

#[test]
fn next_back_with_holes() {
    use eh::EH;
    let mut cur = Some(EH::MAX);
    assert_eq!(
        ::core::iter::from_fn(|| {
            let res = cur;
            cur = cur.and_then(EH::next_back);
            res
        })
        .collect::<Vec<_>>(),
        vec![EH::B, EH::C, EH::D, EH::A]
    );
}
