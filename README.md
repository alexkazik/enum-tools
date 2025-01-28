[![Codecov](https://codecov.io/github/alexkazik/enum-tools/coverage.svg?branch=main)](https://codecov.io/gh/alexkazik/enum-tools)
[![Dependency status](https://deps.rs/repo/github/alexkazik/enum-tools/status.svg)](https://deps.rs/repo/github/alexkazik/enum-tools)
[![crates.io](https://img.shields.io/crates/v/enum-tools.svg)](https://crates.io/crates/enum-tools)
[![Downloads](https://img.shields.io/crates/d/enum-tools.svg)](https://crates.io/crates/enum-tools)
[![Github stars](https://img.shields.io/github/stars/alexkazik/enum-tools.svg?logo=github)](https://github.com/alexkazik/enum-tools/stargazers)
[![License](https://img.shields.io/crates/l/enum-tools.svg)](./LICENSE)

# Crate enum-tools

<!-- cargo-rdme start -->

Automatically derive functions and trait implementations for enums.

The enum must be field-less, has a primitive representation and be `Copy`.

Many helpful function can be derived:

- Into/TryFrom primitive
- iterator over all items and/or names
- next/previous item
- from/to string
- and more

For the full documentation see `EnumTools`.

## Example

```rust
#[derive(Clone, Copy, EnumTools)]
// features which create function/constant
#[enum_tools(as_str, from_str, into, MAX, MIN, next, next_back, try_from)]
// features which implement a trait
#[enum_tools(Debug, Display, FromStr, Into, IntoStr, TryFrom)]
// features which create a iterator (function and struct with impl)
#[enum_tools(iter, names, range)]
#[repr(i8)]
pub enum MyEnum { A=0, B=5, C=1 }
```

Derives something similar as below

```rust
// functions on the enum
impl MyEnum {
    pub const MIN : Self = MyEnum::A;
    pub const MAX : Self = MyEnum::B;
    pub fn as_str(self) -> &'static str
    pub fn from_str(s: &str) -> Option<Self>
    pub const fn into(self) -> i8 { self as i8 }
    pub fn next(self) -> Option<Self> // the next element by value (or None if last)
    pub fn next_back(self) -> Option<Self> // the previous element by value (or None if first)
    pub fn try_from(value: i8) -> Option<Self>

    pub fn iter() -> MyEnumIter // a iterator over all elements by value
    pub fn names() -> MyEnumNames // a iterator over all names by value
    pub fn range(start: Self, end: Self) -> MyEnumIter // similar to `..=`
}

// implementations on the enum
impl Debug for MyEnum // calls `as_str`
impl Display for MyEnum // calls `as_str`
impl From<MyEnum> for i8 // feature "Into"
impl From<MyEnum> for &'static str // feature "IntoStr", calls "as_str"
impl FromStr for MyEnum
impl TryFrom<i8> for MyEnum

// structs and impls for the iterators
pub struct MyEnumIter
impl Iterator for MyEnumIter
impl DoubleEndedIterator for MyEnumIter
impl ExactSizeIterator for MyEnumIter
impl FusedIterator for MyEnumIter
pub struct MyEnumNames
impl Iterator for MyEnumNames
impl DoubleEndedIterator for MyEnumNames
impl ExactSizeIterator for MyEnumNames
impl FusedIterator for MyEnumNames
```

<!-- cargo-rdme end -->
