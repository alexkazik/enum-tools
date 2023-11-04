include!("macro/macro.rs");

setup!(#[enum_tools(FromStr(mode = "match"))];);

use core::str::FromStr;

#[test]
fn from_str_trait_match_gapless() {
    use eg::EG;
    assert_eq!(EG::from_str("A*"), Ok(EG::A));
    assert_eq!(EG::from_str("B"), Ok(EG::B));
    assert_eq!(EG::from_str("C"), Ok(EG::C));
    assert_eq!(EG::from_str("D"), Ok(EG::D));
    assert_eq!(EG::from_str("E"), Err(()));
}

#[test]
fn from_str_trait_match_holes() {
    use eh::EH;
    assert_eq!(EH::from_str("A*"), Ok(EH::A));
    assert_eq!(EH::from_str("B"), Ok(EH::B));
    assert_eq!(EH::from_str("C"), Ok(EH::C));
    assert_eq!(EH::from_str("D"), Ok(EH::D));
    assert_eq!(EH::from_str("E"), Err(()));
}
