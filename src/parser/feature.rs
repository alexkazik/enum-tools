use crate::parser::params::Params;
use proc_macro_error::abort;
use std::collections::HashMap;
use syn::punctuated::Punctuated;
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
                .unwrap_or_else(|e| abort!(meta_list, "meta parse error: {}", e));
            for outer in nested {
                let mut params;
                if let Meta::Path(path) = outer {
                    params = Params::new(path);
                } else if let Meta::List(meta_list) = outer {
                    let nested = meta_list
                        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                        .unwrap_or_else(|e| abort!(meta_list, "meta parse error: {}", e));
                    params = Params::new(meta_list.path);
                    for nested_meta in nested {
                        if let Meta::Path(path) = nested_meta {
                            let span = path.clone();
                            if params.insert(path, None) {
                                abort!(span, "duplicate parameter");
                            }
                        } else if let Meta::NameValue(MetaNameValue {
                            path,
                            value: Expr::Lit(ExprLit { lit, .. }),
                            ..
                        }) = nested_meta
                        {
                            let span = path.clone();
                            if params.insert(path, Some(lit)) {
                                abort!(span, "duplicate parameter");
                            }
                        } else {
                            abort!(nested_meta, "unsupported feature");
                        }
                    }
                } else {
                    abort!(outer, "unsupported feature");
                }

                let span = params.span();
                if self.insert(params) {
                    abort!(span, "duplicate feature");
                }
            }
        } else {
            abort!(meta, "unsupported attribute type");
        }
    }

    pub(crate) fn get(&mut self, key: &str) -> Option<Params> {
        self.0.remove(key)
    }

    pub(crate) fn finish(self) {
        if let Some((_, params)) = self.0.into_iter().next() {
            abort!(params.span(), "unexpected feature")
        }
    }
}
