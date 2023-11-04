include!("macro/macro.rs");

setup!(#[enum_tools(MIN, next(vis = "pub(crate)"))];);

#[test]
fn manual_vis_crate_gapless() {
    use eg::EG;
    let mut cur = Some(EG::MIN);
    assert_eq!(
        ::core::iter::from_fn(|| {
            let res = cur;
            cur = cur.and_then(EG::next);
            res
        })
        .collect::<Vec<_>>(),
        vec![EG::A, EG::B, EG::C, EG::D]
    );
}

#[test]
fn manual_vis_crate_with_holes() {
    use eh::EH;
    let mut cur = Some(EH::MIN);
    assert_eq!(
        ::core::iter::from_fn(|| {
            let res = cur;
            cur = cur.and_then(EH::next);
            res
        })
        .collect::<Vec<_>>(),
        vec![EH::A, EH::D, EH::C, EH::B]
    );
}
