use crate::generator::features::Features0;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) struct FeatureTryFromTrait {
    pub(crate) enabled: bool,
}

impl FeatureTryFromTrait {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(params) = feature_parser.get("TryFrom") {
            params.finish(Self { enabled: true })
        } else {
            Self { enabled: false }
        }
    }

    pub(crate) fn check(&self, features: &mut Features0) {
        if !self.enabled {
            return;
        }

        features.min_const.enabled = true;
        features.max_const.enabled = true;
        // range_table is only created when in with holes mode
        features.table_range.enabled = true;
    }

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive {
            repr, ident_enum, ..
        } = derive;
        let Names {
            ident_min,
            ident_max,
            ident_table_range,
            ..
        } = names;

        if derive.mode.is_gapless() {
            quote! {
                impl ::core::convert::TryFrom<#repr> for #ident_enum {
                    type Error = ();

                    #[inline]
                    fn try_from(value: #repr) -> ::core::result::Result<Self, Self::Error> {
                        use ::core::result::Result::{Err, Ok};
                        if (value as #repr) >= (Self::#ident_min as #repr) && (value as #repr) <= (Self::#ident_max as #repr) {
                            // Safety: the number is known to be a valid enum
                            Ok(unsafe { ::core::mem::transmute(value as #repr) })
                        } else {
                            Err(())
                        }
                    }
                }
            }
        } else {
            quote! {
                impl ::core::convert::TryFrom<#repr> for #ident_enum {
                    type Error = ();

                    fn try_from(value: #repr) -> ::core::result::Result<Self, Self::Error> {
                        use ::core::result::Result::{Err, Ok};
                        if (value as #repr) >= (Self::#ident_min as #repr) && (value as #repr) <= (Self::#ident_max as #repr) {
                            for r in Self::#ident_table_range {
                                if r.0.contains(&value){
                                    // Safety: the number is known to be a valid enum
                                    return Ok(unsafe { ::core::mem::transmute(value as #repr) });
                                }
                            }
                        }
                        Err(())
                    }
                }
            }
        }
    }
}
