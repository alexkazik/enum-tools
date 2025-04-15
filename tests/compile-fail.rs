use rustc_version::{version_meta, Channel};

#[test]
#[ignore]
fn compile_fail() {
    if version_meta().is_ok_and(|v| v.channel == Channel::Stable) {
        trybuild::TestCases::new().compile_fail("tests/compile-fail/*.rs");
    }
}
