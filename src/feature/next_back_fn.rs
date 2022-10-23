use crate::generator::features::Features0;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

pub(crate) struct FeatureNextBackFn {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
}

impl FeatureNextBackFn {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("next_back") {
            let (vis, name) = params.get_vis_name("next_back");
            params.finish(Self {
                enabled: true,
                vis,
                name,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__next_back".to_string(),
            }
        }
    }

    pub(crate) fn check(&self, features: &mut Features0) {
        if !self.enabled {
            return;
        }

        features.min_const.enabled = true;
        // range_table is only created when in with holes mode
        features.table_range.enabled = true;
    }

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive { repr, .. } = derive;
        let Names {
            ident_min,
            ident_next_back,
            ident_table_range,
            ..
        } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        if derive.mode.is_gapless() {
            quote! {
                /// Returns the previous element before this in value order
                #[inline]
                #vis fn #ident_next_back(self) -> ::core::option::Option<Self> {
                    use ::core::option::Option::{None, Some};
                    if (self as #repr) == (Self::#ident_min as #repr) {
                        None
                    } else {
                        Some(unsafe { ::core::mem::transmute((self as #repr) - 1) })
                    }
                }
            }
        } else {
            quote! {
                /// Returns the previous element before this in value order
                #vis fn #ident_next_back(self) -> ::core::option::Option<Self> {
                    use ::core::iter::DoubleEndedIterator;
                    use ::core::option::Option::Some;
                    let mut current = self as #repr;
                    let mut it = Self::#ident_table_range.iter();
                    loop {
                        // Safety: since self/current is an valid enum, one range will match
                        let r = it.next_back().unwrap();
                        if r.0.contains(&current){
                            // Safety: Only when current is type::MIN: the wrapping will happen, but since
                            // the number will not be in this range and thus the next iter will be used,
                            // which will be None because this must've been the first element.
                            current = current.wrapping_sub(1);
                            if r.0.contains(&current){
                                return Some(unsafe { ::core::mem::transmute(current) });
                            }else{
                                return it.next_back().map(|r|unsafe { ::core::mem::transmute(*r.0.end()) });
                            }
                        }
                    }
                }
            }
        }
    }
}
