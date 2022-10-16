use crate::generator::features::Features0;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

pub(crate) struct FeatureTryFromFn {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
}

impl FeatureTryFromFn {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("try_from") {
            let (vis, name) = params.get_vis_name("try_from");
            params.finish(Self {
                enabled: true,
                vis,
                name,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__try_from".to_string(),
            }
        }
    }

    pub(crate) fn check(&self, features: &mut Features0) {
        if !self.enabled {
            return;
        }

        features.min.enabled = true;
        features.max.enabled = true;
        // range_table is only created when in with holes mode
        features.table_range.enabled = true;
    }

    pub(crate) fn generate(&self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive { repr, .. } = derive;
        let Names {
            ident_min,
            ident_max,
            ident_table_range,
            ident_try_from_fn,
            ..
        } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        if derive.mode.is_gapless() {
            quote! {
                /// Try to convert a value into this enum
                #[inline]
                #vis fn #ident_try_from_fn(value: #repr) -> ::core::option::Option<Self> {
                    use ::core::option::Option::{None, Some};
                    if (value as #repr) >= (Self::#ident_min as #repr) && (value as #repr) <= (Self::#ident_max as #repr) {
                        Some(unsafe { ::core::mem::transmute(value as #repr) })
                    } else {
                        None
                    }
                }
            }
        } else {
            quote! {
                /// Try to convert a value into this enum
                #vis fn #ident_try_from_fn(value: #repr) -> ::core::option::Option<Self> {
                    use ::core::option::Option::{None, Some};
                    if (value as #repr) >= (Self::#ident_min as #repr) && (value as #repr) <= (Self::#ident_max as #repr) {
                        for r in Self::#ident_table_range {
                            if r.0.contains(&value){
                                return Some(unsafe { ::core::mem::transmute(value as #repr) });
                            }
                        }
                    }
                    None
                }
            }
        }
    }
}
