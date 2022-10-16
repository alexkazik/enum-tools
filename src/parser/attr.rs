use crate::parser::feature::FeatureParser;
use proc_macro2::Span;
use proc_macro_error::abort;
use syn::{Attribute, Ident};

pub(crate) fn parse_attrs(span: &Span, attrs: Vec<Attribute>) -> (Ident, FeatureParser) {
    let mut repr: Option<Ident> = None;
    let mut feature_parser = FeatureParser::new();

    for a in attrs {
        if a.path.is_ident("repr") {
            if let Some(repr) = repr {
                abort!(repr, "duplicate repr attribute")
            }
            match a.parse_args::<Ident>() {
                Ok(m) => repr = Some(m),
                Err(e) => abort!(a, e),
            }
        } else if a.path.is_ident("enum_tools") {
            match a.parse_meta() {
                Ok(m) => feature_parser.parse(m),
                Err(e) => abort!(a, e),
            }
        }
    }

    let repr = match repr {
        Some(r) => r,
        None => abort!(span, "expected one repr attribute"),
    };

    (repr, feature_parser)
}
