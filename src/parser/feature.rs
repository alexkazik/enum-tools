use crate::parser::params::Params;
use proc_macro_error::abort;
use std::collections::HashMap;
use syn::{Meta, MetaList, MetaNameValue, NestedMeta};

pub(crate) struct FeatureParser(HashMap<String, Params>);

impl FeatureParser {
    pub(crate) fn new() -> Self {
        Self(HashMap::new())
    }

    pub(crate) fn insert(&mut self, params: Params) -> bool {
        self.0.insert(params.name().to_string(), params).is_some()
    }

    pub(crate) fn parse(&mut self, meta: Meta) {
        if let Meta::List(MetaList { nested, .. }) = meta {
            for outer in nested {
                let mut params;
                if let NestedMeta::Meta(Meta::Path(path)) = outer {
                    params = Params::new(path);
                } else if let NestedMeta::Meta(Meta::List(MetaList { path, nested, .. })) = outer {
                    params = Params::new(path);
                    for nested_meta in nested {
                        if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
                            let span = path.clone();
                            if params.insert(path, None) {
                                abort!(span, "duplicate parameter");
                            }
                        } else if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                            path,
                            lit,
                            ..
                        })) = nested_meta
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
