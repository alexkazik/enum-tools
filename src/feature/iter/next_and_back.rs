use crate::feature::iter::FeatureIter;
use crate::generator::features::Features1;
use crate::generator::names::Names;
use crate::generator::Derive;
use proc_macro2::TokenStream;
use quote::quote;

impl FeatureIter {
    pub(crate) fn check_next_and_back(&self, features: &mut Features1) {
        features.next.enabled = true;
        features.next_back.enabled = true;
    }

    pub(crate) fn iter_next_and_back(
        &self,
        derive: &Derive,
        names: &Names,
    ) -> (TokenStream, TokenStream) {
        let Derive {
            ident_enum,
            num_values,
            ..
        } = derive;
        let Names {
            ident_min,
            ident_max,
            ident_next,
            ident_next_back,
            ident_iter_fn,
            ident_iter_struct,
            ..
        } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        let doc_inner = format!(
            " An Iterator over the items of {}, in value order.",
            ident_enum
        );
        let doc_outer = format!(" An Iterator over the items of {}.", ident_enum);

        let inner = quote! {
            #[doc=#doc_inner]
            #[inline]
            #vis fn #ident_iter_fn() -> #ident_iter_struct {
                use ::core::option::Option::Some;
                #ident_iter_struct {
                    fwd: Some(#ident_enum::#ident_min),
                    bwd: Some(#ident_enum::#ident_max),
                    len: #num_values,
                }
            }
        };
        let outer = quote! {
            #[doc=#doc_outer]
            #vis struct #ident_iter_struct {
                fwd: ::core::option::Option<#ident_enum>,
                bwd: ::core::option::Option<#ident_enum>,
                len: usize,
            }

            impl ::core::iter::Iterator for #ident_iter_struct {
                type Item = #ident_enum;

                #[inline]
                fn next(&mut self) -> ::core::option::Option<Self::Item> {
                    use ::core::option::Option::None;
                    if self.len == 0 {
                        None
                    } else {
                        let result = self.fwd;
                        self.fwd = result.and_then(|x| x.#ident_next());
                        self.len -= 1;
                        result
                    }
                }

                #[inline]
                fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
                    use ::core::option::Option::Some;
                    (self.len, Some(self.len))
                }
            }
            impl ::core::iter::DoubleEndedIterator for #ident_iter_struct {
                #[inline]
                fn next_back(&mut self) -> ::core::option::Option<Self::Item> {
                    use ::core::option::Option::None;
                    if self.len == 0 {
                        None
                    } else {
                        let result = self.bwd;
                        self.bwd = result.and_then(|x| x.#ident_next_back());
                        self.len -= 1;
                        result
                    }
                }
            }
            impl ::core::iter::ExactSizeIterator for #ident_iter_struct {
                #[inline]
                fn len(&self) -> usize {
                    self.len
                }
            }

            impl ::core::iter::FusedIterator for #ident_iter_struct { }
        };
        (inner, outer)
    }
}
