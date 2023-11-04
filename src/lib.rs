#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(rustdoc::redundant_explicit_links)]

//! Automatically derive functions and trait implementations for enums.
//!
//! The enum must be field-less, has a primitive representation and be [`Copy`](::std::marker::Copy).
//!
//! Many helpful function can be derived:
//!
//! - Into/TryFrom primitive
//! - iterator over all items and/or names
//! - next/previous item
//! - from/to string
//! - and more
//!
//! For the full documentation see [`EnumTools`](crate::EnumTools).
//!
//! # Example
//!
//! ```
//! # use enum_tools::EnumTools;
//! #[derive(Clone, Copy, EnumTools)]
//! // features which create function/constant
//! #[enum_tools(as_str, from_str, into, MAX, MIN, next, next_back, try_from)]
//! // features which implement a trait
//! #[enum_tools(Debug, Display, FromStr, Into, IntoStr, TryFrom)]
//! // features which create a iterator (function and struct with impl)
//! #[enum_tools(iter, names, range)]
//! #[repr(i8)]
//! pub enum MyEnum { A=0, B=5, C=1 }
//! ```
//!
//! Derives something similar as below
//!
//! ```
//! # pub enum MyEnum { A=0, B=5, C=1 }
//! // functions on the enum
//! impl MyEnum {
//!     pub const MIN : Self = MyEnum::A;
//!     pub const MAX : Self = MyEnum::B;
//!     pub fn as_str(self) -> &'static str
//!     # {todo!()}
//!     pub fn from_str(s: &str) -> Option<Self>
//!     # {todo!()}
//!     pub fn into(self) -> i8 { self as i8 }
//!     pub fn next(self) -> Option<Self> // the next element by value (or None if last)
//!     # {todo!()}
//!     pub fn next_back(self) -> Option<Self> // the previous element by value (or None if first)
//!     # {todo!()}
//!     pub fn try_from(value: i8) -> Option<Self>
//!     # {todo!()}
//!
//!     pub fn iter() -> MyEnumIter // a iterator over all elements by value
//!     # {todo!()}
//!     pub fn names() -> MyEnumNames // a iterator over all names by value
//!     # {todo!()}
//!     pub fn range(start: Self, end: Self) -> MyEnumIter // similar to `..=`
//!     # {todo!()}
//! }
//!
//! # use core::fmt::{Debug, Display, Formatter};
//! # use core::str::FromStr;
//! # use core::iter::FusedIterator;
//! #
//! // implementations on the enum
//! impl Debug for MyEnum // calls `as_str`
//! # {fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { todo!() }}
//! impl Display for MyEnum // calls `as_str`
//! # {fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { todo!() }}
//! impl From<MyEnum> for i8 // feature "Into"
//! # {fn from(_: MyEnum) -> Self { todo!() }}
//! impl From<MyEnum> for &'static str // feature "IntoStr", calls "as_str"
//! # {fn from(_: MyEnum) -> Self { todo!() }}
//! impl FromStr for MyEnum
//! # {type Err = (); fn from_str(s: &str) -> Result<Self, Self::Err> { todo!() }}
//! impl TryFrom<i8> for MyEnum
//! # {type Error = (); fn try_from(value: i8) -> Result<Self, Self::Error> { todo!() }}
//!
//! // structs and impls for the iterators
//! pub struct MyEnumIter
//! # (());
//! impl Iterator for MyEnumIter
//! # {type Item = (); fn next(&mut self) -> Option<Self::Item> { todo!() }}
//! impl DoubleEndedIterator for MyEnumIter
//! # {fn next_back(&mut self) -> Option<Self::Item> { todo!() }}
//! impl ExactSizeIterator for MyEnumIter
//! # {}
//! impl FusedIterator for MyEnumIter
//! # {}
//! pub struct MyEnumNames
//! # (());
//! impl Iterator for MyEnumNames
//! # {type Item = (); fn next(&mut self) -> Option<Self::Item> { todo!() }}
//! impl DoubleEndedIterator for MyEnumNames
//! # {fn next_back(&mut self) -> Option<Self::Item> { todo!() }}
//! impl ExactSizeIterator for MyEnumNames
//! # {}
//! impl FusedIterator for MyEnumNames
//! # {}
//! ```
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod feature;
mod generator;
mod parser;

/// Derive Macro for enums
///
/// # Requirements
///
/// This derive macro only works on enum
/// - must be [`Copy`](::std::marker::Copy)
/// - must be [field-less](https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations)
/// - must have a [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations)
/// - all values must be within `[i64::MIN, i64::MAX]` (independent of the internal representation)
/// - at most `u16::MAX-1` items
///
/// # Example
///
/// ```
/// # use enum_tools::EnumTools;
/// #[derive(Clone, Copy, EnumTools, PartialEq)]
/// #[enum_tools(as_str, Debug, next, next_back(name = "pred"))]
/// #[repr(usize)]
/// pub enum MyEnum { A, #[enum_tools(rename = "B*")] B, C }
///
/// assert_eq!(MyEnum::B.as_str(), "B*");
/// assert_eq!(MyEnum::A.next(), MyEnum::C.pred());
/// ```
///
/// # Intro
///
/// Many features have different modes (built in or to chose) based on the type of the
/// enum: whether it has holes or not.
///
/// Examples for "gapless" (i.e. without holes)
/// - `enum Name { A, B, C }`
/// - `enum Name { A=5, B=7, C=6 }`
///
/// Examples for "with holes"
/// - `enum Name { A=0, B=1, C=9 }`
/// - `enum Name { A=0, B=5, C=1 }`
///
/// The modes "with holes" are not as performant and/or small as the "gapless" version,
/// but functionally identical.
///
/// Also please note that all functions which have an order do order based on values
/// (and not the order of there occurrence, and thus identical to derived `Ord` and `PartialOrd`).
///
/// # Common parameter
///
/// All features which create a function (or constant) have the two optional parameters:
/// - `name`: the name of the function (the default is shown in the description below)
/// - `vis`: the visibility, either `""`, `"pub(crate)"` or `"pub"`, defaults to the same as the enum itself
///
/// # Dependencies
///
/// Some features depend on others. When the other feature is not enabled by the user then it will
/// be automatically enabled. The `vis` is `""` (inherent/private), the `name` starts with two underscores
/// (this may change at any time and is not considered a breaking change as no other code should depend on it).
///
/// # Auto modes
///
/// Some features have an `"auto"` mode (which is always default).
/// What implementation is chosen by `"auto"` may change at any time, this will not change the
/// function but probably the size or speed of the code. This is not considered a breaking change.
///
/// If you want a specific mode, specify it.
///
/// # Compile time features
///
/// ## sorted
///
/// Ensure that the enum is sorted by name and/or value.
///
/// It is a compile-error when the enum is not in order.
///
/// Parameter:
/// - `name` (optional)
/// - `value` (optional)
///
/// Example:
/// `#[enum_tools(sorted(name, value))]`
///
/// # Value attributes
///
/// The only supported value attribute is `rename` (see example above).
/// This will affect all from/to str(ing) functions including the `names` iterator.  
///
/// # Function/Constant Features
///
/// The name of the function is identical to the feature, but can be changed.
///
/// ## as_str
///
/// `$vis fn as_str(self) -> &'static str {..}`
///
/// Parameter:
/// - `mode`:
///     - `"auto"`: (default)
///     - `"match"`: match statement
///     - `"table"`: a table with all names, will be shared with `FromStr`, `from_str` and `names` if they also use a table.
/// - `name`, `vis`: see common parameter
///
/// ## from_str
///
/// `$vis fn from_str(s: &str) -> Option<Self> {..}`
///
/// Parameter:
/// - `mode`:
///     - `"auto"`: (default)
///     - `"match"`: match statement
///     - `"table"`: a table with all names, will be shared with `as_str`, `FromStr` and `names` if they also use a table.
///       And possibly a second table with all values if the enum has holes, will be shared with `FromStr` and `iter` if they also use a table.
/// - `name`, `vis`: see common parameter
///
/// ## into
///
/// `$vis fn into(self) -> $repr`
///
/// Converts the enum into the primitive.
///
/// ## MAX
///
/// `$vis const MAX : Self = ..`
///
/// The enum with the maximum value.
///
/// ## MIN
///
/// `$vis const MIN : Self = ..`
///
/// The enum with the minimum value.
///
/// ## next
///
/// `$vis fn next(self) -> Option<Self> {..}`
///
/// The next value after this, or `None` if `Self::MAX`.
///
/// ## next_back
///
/// `$vis fn next_back(self) -> Option<Self> {..}`
///
/// The value before this, or `None` if `Self::MIN`.
///
/// ## try_from
///
/// `$vis fn try_from(value: $repr) -> Option<Self>`
///
/// Converts the primitive type into the enum.
///
/// # Trait Features
///
/// The features are similar to the function above and/or the trait.
///
/// The error type is always `()` since this crate can't create one and it's not known
/// if the user does provide one or one should be created.
/// In the future there may be options to fine tune this.
///
/// ## Debug
///
/// Implement [`Debug`](core::fmt::Debug) by calling `as_str` (see above).
///
/// ## Display
///
/// Implement [`Display`](core::fmt::Display) by calling `as_str` (see above).
///
/// ## FromStr
///
/// Implement [`FromStr`](core::str::FromStr).
///
/// Parameter:
/// - `mode`:
///     - `"auto"`: (default)
///     - `"match"`: match statement
///     - `"table"`: a table with all names, will be shared with `as_str`, `from_str` and `names` if they also use a table.
///       And possibly a second table with all values if the enum has holes, will be shared with `from_str` and `iter` if they also use a table.
///
/// The error is `()`.
///
/// ## Into
///
/// Implement `From<Self> for $repr`.
///
/// Converts the enum into the primitive.
///
/// ## IntoStr
///
/// Implement `From<Self> for &'static str`.
///
/// Converts the enum into a str by calling `as_str` (see above).
///
/// ## TryFrom
///
/// Implement `TryFrom<$repr> for Self`.
///
/// Converts the primitive type into the enum.
///
/// Error: `()`
///
/// # Iterator Features
///
/// These features implement a function for the enum and a struct for the iterator.
///
/// Example:
/// ```
/// # use enum_tools::EnumTools;
/// #[derive(Clone, Copy, EnumTools, PartialEq)]
/// #[enum_tools(iter, Debug)]
/// #[repr(usize)]
/// pub enum MyEnum { A, B, C }
///
/// let mut it : MyEnumIter = MyEnum::iter();
/// assert_eq!(it.next(), Some(MyEnum::A));
/// assert_eq!(it.next_back(), Some(MyEnum::C));
/// assert_eq!(it.next(), Some(MyEnum::B));
/// assert_eq!(it.next_back(), None);
/// assert_eq!(it.next(), None);
/// ```
///
/// ## iter
///
/// `$vis fn iter() -> SelfIter {..}`
///
/// An iterator over the values of the enum (in value order).
///
/// The struct implements: [`Iterator`](::core::iter::Iterator), [`DoubleEndedIterator`](::core::iter::DoubleEndedIterator), [`ExactSizeIterator`](::core::iter::ExactSizeIterator), [`FusedIterator`](::core::iter::FusedIterator).
///
/// Parameter:
/// - `struct_name`: name of the generated struct, defaults to the name of the enum and 'Iter'.
/// - `mode`:
///   - `"auto"`: (default) will pick range for gapless and something appropriate otherwise
///   - `"range"`: only available on gapless enums, simply iterate over a range
///              with a conversion (as a no-op) to the enum-value
///   - `"next_and_back"`: use `next` and `next_back` (see below) for the iteration
///   - `"match"`: match statement
///   - `"table"`: a table with all enums, will be shared with `FromStr` and `from_str` if they use a table.
///   - `"table_inline"`: use an array (not a reference) of all enums to generate an iterator.
///        Similar to `[Self::A, Self::B, ...].into_iter()`.
///        Since the table is loaded every time into ram this is only a good pick for enums with few values.
/// - `name`, `vis`: see common parameter
///
/// ## names
///
/// `$vis fn names() -> SelfNames {..}`
///
/// An iterator over the names of the enum (in value order).
///
/// The struct implements: [`Iterator`](::core::iter::Iterator), [`DoubleEndedIterator`](::core::iter::DoubleEndedIterator), [`ExactSizeIterator`](::core::iter::ExactSizeIterator), [`FusedIterator`](::core::iter::FusedIterator).
///
/// Parameter:
/// - `struct_name`: name of the generated struct, defaults to the name of the enum and 'Names'.
/// - `name`, `vis`: see common parameter
///
/// This always generates a table, will be shared with `as_str`, `FromStr` and `from_str` if they also use a table.
///
/// ## range
///
/// `$vis fn range(start: Self, end: Self) -> SelfIter {..}`
///
/// An Iterator over a inclusive range of the enum, in value order, similar to `..=`.
///
/// This feature requires that the feature `iter` is activated and the mode `"table_inline"` is not used.
///
/// For enums with holes the function to create the range may be not very performant.
///
#[proc_macro_error]
#[proc_macro_derive(EnumTools, attributes(enum_tools))]
pub fn enum_tools(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let (derive, features) = generator::Derive::parse(input);

    derive.generate(features).into()
}
