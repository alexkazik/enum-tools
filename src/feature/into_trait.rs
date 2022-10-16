use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) struct FeatureIntoTrait {
    pub(crate) enabled: bool,
}

impl FeatureIntoTrait {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(params) = feature_parser.get("Into") {
            params.finish(Self { enabled: true })
        } else {
            Self { enabled: false }
        }
    }

    pub(crate) fn check(&mut self) {}

    pub(crate) fn generate(self, derive: &Derive) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive {
            repr, ident_enum, ..
        } = derive;

        quote! {
            impl ::core::convert::From<#ident_enum> for #repr {
                #[inline]
                fn from(value: #ident_enum) -> Self {
                    value as #repr
                }
            }
        }
    }
}
