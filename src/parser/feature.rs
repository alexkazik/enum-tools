use crate::parser::error::Error;
use crate::parser::params::Params;
use proc_macro_error::abort;
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Meta, MetaNameValue, Token};

pub(crate) struct FeatureParser(HashMap<String, Params>);

impl FeatureParser {
    pub(crate) fn new() -> Self {
        Self(HashMap::new())
    }

    pub(crate) fn insert(&mut self, params: Params) -> bool {
        self.0.insert(params.name().to_string(), params).is_some()
    }

    pub(crate) fn parse(&mut self, meta: Meta) {
        if let Meta::List(meta_list) = meta {
            let nested = meta_list
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap_or_else(|e| abort!(meta_list, Error::MetaParseError(e)));
            for outer in nested {
                let mut params;
                if let Meta::Path(path) = outer {
                    params = Params::new(path);
                } else if let Meta::List(meta_list) = outer {
                    let nested = meta_list
                        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                        .unwrap_or_else(|e| abort!(meta_list, Error::MetaParseError(e)));
                    params = Params::new(meta_list.path);
                    for nested_meta in nested {
                        if let Meta::Path(path) = nested_meta {
                            let span = path.span();
                            if params.insert(path, None) {
                                abort!(span, Error::DuplicateParameter);
                            }
                        } else if let Meta::NameValue(MetaNameValue {
                            path,
                            value: Expr::Lit(ExprLit { lit, .. }),
                            ..
                        }) = nested_meta
                        {
                            let span = path.span();
                            if params.insert(path, Some(lit)) {
                                abort!(span, Error::DuplicateParameter);
                            }
                        } else {
                            abort!(nested_meta, Error::UnknownFeature);
                        }
                    }
                } else {
                    abort!(outer, Error::UnknownFeature);
                }

                let span = params.span();
                if self.insert(params) {
                    abort!(span, Error::DuplicateFeature);
                }
            }
        } else {
            abort!(meta, Error::UnsupportedAttributeType);
        }
    }

    pub(crate) fn get(&mut self, key: &str) -> Option<Params> {
        self.0.remove(key)
    }

    pub(crate) fn finish(self) {
        if let Some((_, params)) = self.0.into_iter().next() {
            abort!(params.span(), Error::UnknownFeature)
        }
    }
}
