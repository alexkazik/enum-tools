use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

pub(crate) struct FeatureIntoFn {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
}

impl FeatureIntoFn {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("into") {
            let (vis, name) = params.get_vis_name("into");
            params.finish(Self {
                enabled: true,
                vis,
                name,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__into".to_string(),
            }
        }
    }

    pub(crate) fn check(&mut self) {}

    pub(crate) fn generate(&self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive { repr, .. } = derive;
        let Names { ident_to_fn, .. } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        quote! {
            /// Converts this element into the underlying representation (this is a no-op)
            #[inline]
            #vis const fn #ident_to_fn(self) -> #repr {
                self as #repr
            }
        }
    }
}
