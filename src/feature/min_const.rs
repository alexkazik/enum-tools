use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

pub(crate) struct FeatureMinConst {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
}

impl FeatureMinConst {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("MIN") {
            let (vis, name) = params.get_vis_name("MIN");
            params.finish(Self {
                enabled: true,
                vis,
                name,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__MIN".to_string(),
            }
        }
    }

    pub(crate) fn check(&mut self) {}

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive { ident_enum, .. } = derive;
        let Names { ident_min, .. } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        let (_, (min_value, _)) = &derive.values.first().unwrap();
        quote! {
            /// The first element of this enum by value
            #vis const #ident_min : #ident_enum = #ident_enum::#min_value;
        }
    }
}
