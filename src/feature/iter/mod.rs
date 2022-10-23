use crate::generator::features::Features1;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use syn::Visibility;

mod next_and_back;
mod range;
mod table;
mod table_inline;

#[derive(Eq, PartialEq, Copy, Clone)]
pub(crate) enum IterMode {
    Auto,
    Range,
    NextAndBack,
    Table,
    TableInline,
}

pub(crate) struct FeatureIter {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
    pub(crate) span: Span,
    pub(crate) struct_name: Option<String>,
    pub(crate) mode: IterMode,
}

impl FeatureIter {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("iter") {
            let (vis, name) = params.get_vis_name("iter");

            let span = params.span();
            let struct_name = params.get_str_opt("struct");
            let mode = match params
                .get_str_opt("mode")
                .unwrap_or_else(|| "auto".to_string())
                .as_str()
            {
                "auto" => IterMode::Auto,
                "range" => IterMode::Range,
                "next_and_back" => IterMode::NextAndBack,
                "table" => IterMode::Table,
                "table_inline" => IterMode::TableInline,
                _ => abort!(params.span(), "invalid mode"),
            };
            params.finish(Self {
                enabled: true,
                vis,
                name,
                span,
                struct_name,
                mode,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__iter".to_string(),
                span: Span::call_site(),
                struct_name: None,
                mode: IterMode::Auto,
            }
        }
    }

    pub(crate) fn check(&self, derive: &Derive, features: &mut Features1) {
        if !self.enabled {
            return;
        }

        match self.mode {
            IterMode::Auto => {}
            IterMode::Range => self.check_range(derive),
            IterMode::NextAndBack => self.check_next_and_back(features),
            IterMode::Table => self.check_table(features),
            IterMode::TableInline => self.check_table_inline(),
        }
    }

    pub(crate) fn generate(&self, derive: &Derive, names: &Names) -> (TokenStream, TokenStream) {
        if !self.enabled {
            return (TokenStream::new(), TokenStream::new());
        }

        match self.mode {
            IterMode::Auto => panic!("IterMode should've been resolved"),
            IterMode::Range => self.iter_range(derive, names),
            IterMode::NextAndBack => self.iter_next_and_back(derive, names),
            IterMode::Table => self.iter_table(derive, names),
            IterMode::TableInline => self.iter_table_inline(derive, names),
        }
    }
}

pub(crate) fn extend_common(
    outer: &mut TokenStream,
    ident_item: TokenStream,
    ident_struct: &Ident,
    range_inclusive_iterator: bool,
) {
    outer.extend(quote! {
        impl ::core::iter::Iterator for #ident_struct {
            type Item = #ident_item;

            #[inline]
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                self.inner.next()
            }

            #[inline]
            fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
                self.inner.size_hint()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> ::core::option::Option<Self::Item> {
               self.inner.nth(n)
            }

            #[inline]
            fn fold<B, F>(self, init: B, f: F) -> B
                where
                    Self: ::core::marker::Sized,
                    F: ::core::ops::FnMut(B, Self::Item) -> B,
            {
                self.inner.fold(init,f)
            }

            #[inline]
            fn last(self) -> ::core::option::Option<Self::Item> {
                self.inner.last()
            }
        }

        impl ::core::iter::DoubleEndedIterator for #ident_struct {
            #[inline]
            fn next_back(&mut self) -> ::core::option::Option<Self::Item> {
                self.inner.next_back()
            }

            #[inline]
            fn nth_back(&mut self, n: usize) -> ::core::option::Option<Self::Item> {
               self.inner.nth_back(n)
            }

            #[inline]
            fn rfold<B, F>(self, init: B, f: F) -> B
                where
                    Self: ::core::marker::Sized,
                    F: ::core::ops::FnMut(B, Self::Item) -> B,
            {
                self.inner.rfold(init,f)
            }
        }

        impl ::core::iter::FusedIterator for #ident_struct { }
    });

    if range_inclusive_iterator {
        // RangeInclusive is not necessary a ExactSizeIterator but we can guarantee it due to
        // requirements on the enum, and thus it has to create it's own instance instead of
        // simply forwarding it, like the other modes.
        outer.extend(quote! {
            impl ::core::iter::ExactSizeIterator for #ident_struct {
                #[inline]
                fn len(&self) -> usize {
                    use ::core::iter::Iterator;
                    self.inner.size_hint().0
                }
            }
        });
    } else {
        outer.extend(quote! {
            impl ::core::iter::ExactSizeIterator for #ident_struct {
                #[inline]
                fn len(&self) -> usize {
                    self.inner.len()
                }
            }
        });
    }
}
