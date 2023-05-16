use crate::parser::error::Error;
use crate::parser::feature::FeatureParser;
use proc_macro2::Span;
use proc_macro_error::abort;
use syn::{Attribute, Ident};

pub(crate) fn parse_attrs(span: &Span, attrs: Vec<Attribute>) -> (Ident, FeatureParser) {
    let mut repr: Option<Ident> = None;
    let mut feature_parser = FeatureParser::new();

    for a in attrs {
        if a.path().is_ident("repr") {
            if let Some(repr) = repr {
                abort!(repr, Error::DuplicateReprAttribute)
            }
            repr = Some(
                a.parse_args::<Ident>()
                    .unwrap_or_else(|e| abort!(a, Error::MetaParseError(e))),
            );
        } else if a.path().is_ident("enum_tools") {
            feature_parser.parse(a.meta)
        }
    }

    let repr = repr.unwrap_or_else(|| abort!(span, Error::MissingReprAttribute));

    (repr, feature_parser)
}
