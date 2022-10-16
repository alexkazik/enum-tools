use crate::feature::iter::extend_common;
use crate::generator::features::Features1;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

pub(crate) struct FeatureNames {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
    pub(crate) struct_name: Option<String>,
}

impl FeatureNames {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("names") {
            let (vis, name) = params.get_vis_name("names");

            let struct_name = params.get_str_opt("struct");

            params.finish(Self {
                enabled: true,
                vis,
                name,
                struct_name,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__names".to_string(),
                struct_name: None,
            }
        }
    }

    pub(crate) fn check(&self, features: &mut Features1) {
        if !self.enabled {
            return;
        }

        features.table_name.enabled = true;
    }

    pub(crate) fn generate(&self, derive: &Derive, names: &Names) -> (TokenStream, TokenStream) {
        if !self.enabled {
            return (TokenStream::new(), TokenStream::new());
        }

        let Derive { ident_enum, .. } = derive;
        let Names {
            ident_names_fn,
            ident_names_struct,
            ident_table_name,
            ..
        } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        let doc_inner = format!(
            " An Iterator over the names of {}, in value order.\n\
            \n\
            Use `Self::iter().zip(Self::names())` to iterate over pairs of\n\
            the item and it's name.",
            ident_enum
        );
        let doc_outer = format!(" An Iterator over the names of {}.", ident_enum);

        let inner = quote! {
            #[doc=#doc_inner]
            #[inline]
            #vis fn #ident_names_fn() -> #ident_names_struct {
                use ::core::iter::Iterator;
                #ident_names_struct{inner: Self::#ident_table_name.iter().copied()}
            }
        };

        let mut outer = quote! {
            #[doc=#doc_outer]
            #vis struct #ident_names_struct {
                inner: ::core::iter::Copied<::core::slice::Iter<'static, &'static str>>,
            }
        };

        extend_common(&mut outer, quote! {&'static str}, ident_names_struct);

        (inner, outer)
    }
}
