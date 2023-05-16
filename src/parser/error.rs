use std::fmt::{Display, Formatter};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub(crate) enum Error<'a> {
    DuplicateReprAttribute,
    MissingReprAttribute,
    MetaParseError(syn::Error),
    DuplicateFeature,
    DuplicateParameter,
    UnsupportedAttributeType,
    UnknownFeature,
    UnknownParameter,
    UnsupportedPath,
    UnexpectedLiteral,
    ExpectedLiteral(&'a str),
    UnsupportedVisibility,
    OnlyUnitField,
    FieldsNotNameSorted,
    FieldsNotValueSorted,
    DuplicateValue,
    NoI64,
    NotInteger,
    I64Overflow,
    NoVariantsFound,
    NoEnum,
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DuplicateReprAttribute => write!(f, "duplicate repr attribute"),
            Error::MissingReprAttribute => write!(f, "missing repr attribute"),
            Error::MetaParseError(e) => write!(f, "meta parse error: {e}"),
            Error::DuplicateFeature => write!(f, "duplicate feature"),
            Error::DuplicateParameter => write!(f, "duplicate parameter"),
            Error::UnsupportedAttributeType => write!(f, "unsupported attribute type"),
            Error::UnknownFeature => write!(f, "unknown feature"),
            Error::UnknownParameter => write!(f, "unknown parameter"),
            Error::UnsupportedPath => write!(f, "unsupported path"),
            Error::UnexpectedLiteral => write!(f, "unexpected literal"),
            Error::ExpectedLiteral(p) => write!(f, "expected literal: {p} (or no literal)"),
            Error::UnsupportedVisibility => write!(f, "unsupported visibility"),
            Error::OnlyUnitField => write!(f, "only unit field items are allowed"),
            Error::FieldsNotNameSorted => write!(f, "variants are not sorted by name"),
            Error::FieldsNotValueSorted => write!(f, "variants are not sorted by value"),
            Error::DuplicateValue => write!(f, "Duplicate value"),
            Error::NoI64 => write!(f, "can't be parsed as i64"),
            Error::NotInteger => write!(f, "only integer literals are allowed"),
            Error::I64Overflow => write!(f, "i64 overflow"),
            Error::NoVariantsFound => write!(f, "no variants found"),
            Error::NoEnum => write!(f, "only enums are supported"),
        }
    }
}
