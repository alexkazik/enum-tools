use crate::feature::iter::{extend_common, FeatureIter};
use crate::generator::features::Features1;
use crate::generator::names::Names;
use crate::generator::Derive;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl FeatureIter {
    pub(crate) fn check_table(&self, features: &mut Features1) {
        features.table_enum.enabled = true;
    }

    pub(crate) fn iter_table(&self, derive: &Derive, names: &Names) -> (TokenStream, TokenStream) {
        let Derive { ident_enum, .. } = derive;
        let Names {
            ident_iter_fn,
            ident_iter_struct,
            ident_table_enum,
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
                    inner: Self::#ident_table_enum.iter().copied(),
                }
            }
        };

        let mut outer = quote! {
            #[doc=#doc_outer]
            #vis struct #ident_iter_struct {
                inner: ::core::iter::Copied<::core::slice::Iter<'static, #ident_enum>>,
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
