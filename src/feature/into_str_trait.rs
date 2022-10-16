use crate::generator::features::Features2;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) struct FeatureIntoStrTrait {
    pub(crate) enabled: bool,
}

impl FeatureIntoStrTrait {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(params) = feature_parser.get("IntoStr") {
            params.finish(Self { enabled: true })
        } else {
            Self { enabled: false }
        }
    }

    pub(crate) fn check(&mut self, features: &mut Features2) {
        if !self.enabled {
            return;
        }

        features.as_str.enabled = true;
    }

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive { ident_enum, .. } = derive;
        let Names { ident_as_str, .. } = names;
        quote! {
            impl ::core::convert::From<#ident_enum> for &'static str {
                #[inline]
                fn from(value: #ident_enum) -> Self {
                    value.#ident_as_str()
                }
            }
        }
    }
}
