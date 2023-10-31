use crate::feature::iter::{extend_common, FeatureIter};
use crate::generator::names::Names;
use crate::generator::Derive;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::LitInt;

impl FeatureIter {
    pub(crate) fn check_table_inline(&self) {}

    pub(crate) fn iter_table_inline(
        &self,
        derive: &Derive,
        names: &Names,
    ) -> (TokenStream, TokenStream) {
        let enums = derive.values.iter().map(|(_, (v, _))| v);
        let num_values = LitInt::new(&derive.num_values.to_string(), Span::call_site());

        let Derive { ident_enum, .. } = derive;
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
                use ::core::iter::IntoIterator;
                #ident_iter_struct {
                    inner: [ #(#ident_enum::#enums),* ].into_iter()
                }
            }
        };

        let mut outer = quote! {
            #[doc=#doc_outer]
            #vis struct #ident_iter_struct {
                inner: ::core::array::IntoIter<#ident_enum, #num_values>
            }
        };

        extend_common(
            &mut outer,
            ident_enum.to_token_stream(),
            ident_iter_struct,
            false,
        );

        (inner, outer)
    }
}
