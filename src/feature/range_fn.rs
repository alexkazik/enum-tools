use crate::feature::iter::IterMode;
use crate::generator::features::Features2;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use syn::Visibility;

pub(crate) struct FeatureRangeFn {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
    pub(crate) span: Span,
}

impl FeatureRangeFn {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("range") {
            let (vis, name) = params.get_vis_name("range");
            let span = params.span();
            params.finish(Self {
                enabled: true,
                vis,
                name,
                span,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__range".to_string(),
                span: Span::call_site(),
            }
        }
    }

    pub(crate) fn check(&self, derive: &Derive, features: &mut Features2) {
        if !self.enabled {
            return;
        }

        if !features.iter.enabled {
            abort!(
                self.span,
                "feature range requires the feature iter to be enabled"
            );
        }
        match features.iter.mode {
            IterMode::Auto => {}
            IterMode::Range => {}
            IterMode::NextAndBack | IterMode::Table => {
                if derive.mode.is_gapless() {
                    features.min_const.enabled = true;
                } else {
                    features.table_range.enabled = true;
                    features.table_range.with_offset = true;
                }
            }
            IterMode::TableInline => abort!(
                self.span,
                "feature range requires the feature iter to not use mode table_inline"
            ),
        }

        if derive.mode.is_with_holes() {
            features.table_range.enabled = true;
        }
    }

    pub(crate) fn generate(
        &self,
        derive: &Derive,
        names: &Names,
        iter_mode: IterMode,
    ) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive {
            repr,
            repr_unsigned,
            ident_enum,
            ..
        } = derive;
        let Names {
            ident_iter_struct,
            ident_min,
            ident_range_fn,
            ident_table_enum,
            ident_table_range,
            ..
        } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        let doc_inner = format!(
            " An Iterator over a inclusive range of {ident_enum}, in value order, similar to `..=`."
        );

        if derive.mode.is_gapless() {
            match iter_mode {
                IterMode::Range => quote! {
                    #[doc=#doc_inner]
                    #[inline]
                    #vis fn #ident_range_fn(start: Self, end: Self) -> #ident_iter_struct {
                        use ::core::iter::Iterator;
                        #ident_iter_struct {
                            inner: ((start as #repr)..=(end as #repr)).map(|x| unsafe { ::core::mem::transmute(x) }),
                        }
                    }
                },
                IterMode::NextAndBack => quote! {
                    #[doc=#doc_inner]
                    #vis const fn #ident_range_fn(start: Self, end: Self) -> #ident_iter_struct {
                        let start_idx = (start as #repr).wrapping_sub(Self::#ident_min as #repr) as #repr_unsigned as usize;
                        let end_idx = (end as #repr).wrapping_sub(Self::#ident_min as #repr) as #repr_unsigned as usize;

                        use ::core::option::Option::Some;
                        #ident_iter_struct {
                            fwd: Some(start),
                            bwd: Some(end),
                            len: if start_idx > end_idx {
                                0
                            } else {
                                end_idx - start_idx + 1
                            },
                        }
                    }
                },
                IterMode::Table => quote! {
                    #[doc=#doc_inner]
                    #vis fn #ident_range_fn(start: Self, end: Self) -> #ident_iter_struct {
                        let start_idx = (start as #repr).wrapping_sub(Self::#ident_min as #repr) as #repr_unsigned as usize;
                        let end_idx = (end as #repr).wrapping_sub(Self::#ident_min as #repr) as #repr_unsigned as usize;

                        use ::core::iter::Iterator;
                        #ident_iter_struct {
                            inner: Self::#ident_table_enum[start_idx..=end_idx].iter().copied(),
                        }
                    }
                },
                _ => panic!("IterMode should've been resolved and/or checked"),
            }
        } else {
            match iter_mode {
                IterMode::NextAndBack => quote! {
                    #[doc=#doc_inner]
                    #vis fn #ident_range_fn(start: Self, end: Self) -> #ident_iter_struct {
                        let start_repr = start as #repr;
                        let end_repr = end as #repr;

                        let mut start_idx = ::core::mem::MaybeUninit::<usize>::uninit();
                        let mut end_idx = ::core::mem::MaybeUninit::<usize>::uninit();

                        for r in Self::#ident_table_range.iter() {
                            if r.0.contains(&start_repr) {
                                start_idx.write(start_repr.wrapping_sub(r.1) as #repr_unsigned as usize);
                            }
                            if r.0.contains(&end_repr) {
                                end_idx.write(end_repr.wrapping_sub(r.1) as #repr_unsigned as usize);
                            }
                        }

                        // Safety: all enums are within the list and thus start_idx/end_idx will be set
                        let start_idx = unsafe { start_idx.assume_init() };
                        let end_idx = unsafe { end_idx.assume_init() };

                        use ::core::option::Option::Some;
                        #ident_iter_struct {
                            fwd: Some(start),
                            bwd: Some(end),
                            len: if start_idx > end_idx {
                                0
                            } else {
                                end_idx - start_idx + 1
                            },
                        }
                    }
                },
                IterMode::Table => quote! {
                    #[doc=#doc_inner]
                    #vis fn #ident_range_fn(start: Self, end: Self) -> #ident_iter_struct {
                        let start_repr = start as #repr;
                        let end_repr = end as #repr;

                        let mut start_idx = ::core::mem::MaybeUninit::<usize>::uninit();
                        let mut end_idx = ::core::mem::MaybeUninit::<usize>::uninit();

                        for r in Self::#ident_table_range.iter() {
                            if r.0.contains(&start_repr) {
                                start_idx.write(start_repr.wrapping_sub(r.1) as #repr_unsigned as usize);
                            }
                            if r.0.contains(&end_repr) {
                                end_idx.write(end_repr.wrapping_sub(r.1) as #repr_unsigned as usize);
                            }
                        }

                        // Safety: all enums are within the list and thus start_idx/end_idx will be set
                        let start_idx = unsafe { start_idx.assume_init() };
                        let end_idx = unsafe { end_idx.assume_init() };

                        use ::core::iter::Iterator;
                        #ident_iter_struct {
                            inner: Self::#ident_table_enum[start_idx..=end_idx].iter().copied(),
                        }
                    }
                },
                _ => panic!("IterMode should've been resolved and/or checked"),
            }
        }
    }
}
