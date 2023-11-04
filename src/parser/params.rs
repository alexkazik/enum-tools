use crate::parser::error::Error;
use proc_macro2::{Ident, Span};
use proc_macro_error::{abort, emit_error};
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Lit, Path, PathArguments, PathSegment, VisRestricted, Visibility};

pub(crate) struct Params {
    name: String,
    span: Span,
    params: HashMap<String, (Span, Option<Lit>)>,
}

impl Params {
    pub(crate) fn new(path: Path) -> Params {
        let (name, span) = Self::path_to_name_span(path);
        Params {
            name,
            span,
            params: HashMap::new(),
        }
    }

    pub(crate) fn span(&self) -> Span {
        self.span
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn insert(&mut self, path: Path, lit: Option<Lit>) -> bool {
        let (name, span) = Self::path_to_name_span(path);
        self.params.insert(name, (span, lit)).is_some()
    }

    fn path_to_name_span(path: Path) -> (String, Span) {
        if path.leading_colon.is_none() && path.segments.len() == 1 {
            let first = path.segments.first().unwrap();
            if matches!(first.arguments, PathArguments::None) {
                return (first.ident.to_string(), path.span());
            }
        }
        abort!(path, Error::UnsupportedPath);
    }

    pub(crate) fn get_bool(&mut self, name: &str) -> bool {
        if let Some((_, lit)) = self.params.remove(name) {
            if let Some(lit) = lit {
                emit_error!(lit.span(), Error::UnexpectedLiteral);
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    pub(crate) fn get_str_opt(&mut self, name: &str) -> Option<String> {
        if let Some((path, lit)) = self.params.remove(name) {
            if let Some(Lit::Str(l)) = lit {
                Some(l.value())
            } else {
                emit_error!(path, Error::ExpectedLiteral("string"));
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn get_vis_name(&mut self, default_name: &str) -> (Option<Visibility>, String) {
        let vis = if let Some((path, lit)) = self.params.remove("vis") {
            if let Some(Lit::Str(l)) = lit {
                match l.value().as_str() {
                    "" => Some(vis_inherited()),
                    "pub(crate)" => Some(vis_pub_crate()),
                    "pub" => Some(vis_pub()),
                    _ => {
                        emit_error!(path, Error::UnsupportedVisibility);
                        None
                    }
                }
            } else {
                emit_error!(path, Error::ExpectedLiteral("string"));
                None
            }
        } else {
            None
        };
        let name = self
            .get_str_opt("name")
            .unwrap_or_else(|| default_name.to_string());
        (vis, name)
    }

    pub(crate) fn finish<T>(self, value: T) -> T {
        for (_, (param_path, _)) in self.params.into_iter() {
            emit_error!(param_path, Error::UnknownParameter)
        }
        value
    }
}

// helper to create Visibility

const fn vis_inherited() -> Visibility {
    Visibility::Inherited
}

fn vis_pub_crate() -> Visibility {
    Visibility::Restricted(VisRestricted {
        pub_token: Default::default(),
        paren_token: Default::default(),
        in_token: None,
        path: Box::new(Path {
            leading_colon: None,
            segments: {
                let mut path = Punctuated::new();
                path.push_value(PathSegment {
                    ident: Ident::new("crate", Span::call_site()),
                    arguments: Default::default(),
                });
                path
            },
        }),
    })
}

fn vis_pub() -> Visibility {
    Visibility::Public(Default::default())
}
