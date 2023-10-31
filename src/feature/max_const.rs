use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

pub(crate) struct FeatureMaxConst {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
}

impl FeatureMaxConst {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("MAX") {
            let (vis, name) = params.get_vis_name("MAX");
            params.finish(Self {
                enabled: true,
                vis,
                name,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__MAX".to_string(),
            }
        }
    }

    pub(crate) fn check(&self) {}

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive { ident_enum, .. } = derive;
        let Names { ident_max, .. } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        let (_, (max_value, _)) = &derive.values.last().unwrap();
        quote! {
            /// The last element of this enum by value
            #vis const #ident_max : #ident_enum = #ident_enum::#max_value;
        }
    }
}
