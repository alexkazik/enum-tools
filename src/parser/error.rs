use std::fmt::{Display, Formatter};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub(crate) enum Error<'a> {
    DuplicateFeature,
    DuplicateParameter,
    DuplicateReprAttribute,
    DuplicateValue,
    ExpectedLiteral(&'a str),
    FieldsNotNameSorted,
    FieldsNotValueSorted,
    I64Overflow,
    MetaParseError(syn::Error),
    MissingReprAttribute,
    NoEnum,
    NoI64,
    NoVariantsFound,
    NotInteger,
    OnlyUnitField,
    UnexpectedLiteral,
    UnknownFeature,
    UnknownParameter,
    UnsupportedAttributeType,
    UnsupportedPath,
    UnsupportedVisibility,
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DuplicateFeature => write!(f, "duplicate feature"),
            Error::DuplicateParameter => write!(f, "duplicate parameter"),
            Error::DuplicateReprAttribute => write!(f, "duplicate repr attribute"),
            Error::DuplicateValue => write!(f, "Duplicate value"),
            Error::ExpectedLiteral(p) => write!(f, "expected literal: {p} (or no literal)"),
            Error::FieldsNotNameSorted => write!(f, "variants are not sorted by name"),
            Error::FieldsNotValueSorted => write!(f, "variants are not sorted by value"),
            Error::I64Overflow => write!(f, "i64 overflow"),
            Error::MetaParseError(e) => write!(f, "meta parse error: {e}"),
            Error::MissingReprAttribute => write!(f, "missing repr attribute"),
            Error::NoEnum => write!(f, "only enums are supported"),
            Error::NoI64 => write!(f, "can't be parsed as i64"),
            Error::NoVariantsFound => write!(f, "no variants found"),
            Error::NotInteger => write!(f, "only integer literals are allowed"),
            Error::OnlyUnitField => write!(f, "only unit field items are allowed"),
            Error::UnexpectedLiteral => write!(f, "unexpected literal"),
            Error::UnknownFeature => write!(f, "unknown feature"),
            Error::UnknownParameter => write!(f, "unknown parameter"),
            Error::UnsupportedAttributeType => write!(f, "unsupported attribute type"),
            Error::UnsupportedPath => write!(f, "unsupported path"),
            Error::UnsupportedVisibility => write!(f, "unsupported visibility"),
        }
    }
}
