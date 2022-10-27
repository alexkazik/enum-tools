use crate::feature::iter::{extend_common, FeatureIter};
use crate::generator::names::Names;
use crate::generator::Derive;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::LitInt;

impl FeatureIter {
    pub(crate) fn check_range(&self, derive: &Derive) {
        if derive.mode.is_with_holes() {
            abort!(
                self.span,
                "mode range is only valid when the enum is gapless"
            );
        }
    }

    pub(crate) fn iter_range(&self, derive: &Derive, names: &Names) -> (TokenStream, TokenStream) {
        let start = LitInt::new(
            &format!("{}{}", derive.min_key, derive.repr),
            Span::call_site(),
        );
        let end = LitInt::new(
            &format!("{}{}", derive.max_key, derive.repr),
            Span::call_site(),
        );

        let Derive {
            repr, ident_enum, ..
        } = derive;
        let Names {
            ident_iter_fn,
            ident_iter_struct,
            ..
        } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        let doc_inner = format!(" An Iterator over the items of {ident_enum}, in value order.");
        let doc_outer = format!(" An Iterator over the items of {ident_enum}.");

        let inner = quote! {
            #[doc=#doc_inner]
            #[inline]
            #vis fn #ident_iter_fn() -> #ident_iter_struct {
                use ::core::iter::Iterator;
                #ident_iter_struct {
                    inner: (#start..=#end).map(|x| unsafe { ::core::mem::transmute(x) }),
                }
            }
        };

        let mut outer = quote! {
            #[doc=#doc_outer]
            #vis struct #ident_iter_struct {
                inner: ::core::iter::Map<::core::ops::RangeInclusive<#repr>, fn(#repr) -> #ident_enum>
            }
        };

        extend_common(&mut outer, ident_enum.to_token_stream(), ident_iter_struct);

        (inner, outer)
    }
}
