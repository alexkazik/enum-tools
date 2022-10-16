use crate::generator::features::Features0;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

pub(crate) struct FeatureNextFn {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
}

impl FeatureNextFn {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("next") {
            let (vis, name) = params.get_vis_name("next");
            params.finish(Self {
                enabled: true,
                vis,
                name,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__next".to_string(),
            }
        }
    }

    pub(crate) fn check(&self, features: &mut Features0) {
        if !self.enabled {
            return;
        }

        features.max.enabled = true;
        // range_table is only created when in with holes mode
        features.table_range.enabled = true;
    }

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive { repr, .. } = derive;
        let Names {
            ident_max,
            ident_next,
            ident_table_range,
            ..
        } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        if derive.mode.is_gapless() {
            quote! {
                /// Returns the next element after this in value order
                #[inline]
                #vis fn #ident_next(self) -> ::core::option::Option<Self> {
                    use ::core::option::Option::{None, Some};
                    if (self as #repr) == (Self::#ident_max as #repr) {
                        None
                    } else {
                        Some(unsafe { ::core::mem::transmute((self as #repr) + 1) })
                    }
                }
            }
        } else {
            quote! {
                /// Returns the next element after this in value order
                #vis fn #ident_next(self) -> ::core::option::Option<Self> {
                    use ::core::iter::Iterator;
                    use ::core::option::Option::Some;
                    let mut current = self as #repr;
                    let mut it = Self::#ident_table_range.iter();
                    loop {
                        // Safety: since self/current is an valid enum, one range will match
                        let r = it.next().unwrap();
                        if r.0.contains(&current){
                            // Safety: Only when current is type::MAX: the wrapping will happen, but since
                            // the number will not be in this range and thus the next iter will be used,
                            // which will be None because this must've been the last element.
                            current = current.wrapping_add(1);
                            if r.0.contains(&current){
                                return Some(unsafe { ::core::mem::transmute(current) });
                            }else{
                                return it.next().map(|r|unsafe { ::core::mem::transmute(*r.0.start()) });
                            }
                        }
                    }
                }
            }
        }
    }
}
