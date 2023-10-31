use crate::generator::features::Features1;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::Visibility;

#[derive(PartialEq, Eq)]
pub(crate) enum FromStrFnMode {
    Auto,
    Match,
    Table,
}

pub(crate) struct FeatureFromStrFn {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
    pub(crate) mode: FromStrFnMode,
}

impl FeatureFromStrFn {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("from_str") {
            let (vis, name) = params.get_vis_name("from_str");
            let mode = match params
                .get_str_opt("mode")
                .unwrap_or_else(|| "auto".to_string())
                .as_str()
            {
                "auto" => FromStrFnMode::Auto,
                "match" => FromStrFnMode::Match,
                "table" => FromStrFnMode::Table,
                _ => abort!(params.span(), "invalid mode"),
            };
            params.finish(Self {
                enabled: true,
                vis,
                name,
                mode,
            })
        } else {
            Self {
                enabled: false,
                vis: Some(Visibility::Inherited),
                name: "__from_str".to_string(),
                mode: FromStrFnMode::Auto,
            }
        }
    }

    pub(crate) fn check(&mut self, derive: &Derive, features: &mut Features1) {
        if !self.enabled {
            return;
        }

        if self.mode == FromStrFnMode::Table {
            features.table_name.enabled = true;
            if derive.mode.is_with_holes() {
                features.table_enum.enabled = true;
            } else {
                features.min_const.enabled = true;
            }
        }
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
            ident_table_enum,
            ident_table_name,
            ident_from_str_fn,
            ..
        } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        match self.mode {
            FromStrFnMode::Auto => panic!("FromStrFnMode should've been resolved"),
            FromStrFnMode::Match => {
                let mut matches = Vec::new();
                for (_, (v, name)) in &derive.values {
                    matches.push(quote! {
                        #name => Some(#ident_enum::#v),
                    });
                }
                quote! {
                    /// Parses a string `s` to return a value of this type
                    #vis fn #ident_from_str_fn(s: &str) -> ::core::option::Option<#ident_enum> {
                        use ::core::option::Option::{None, Some};
                        match s {
                            #(#matches)*
                            _ => None,
                        }
                    }
                }
            }
            FromStrFnMode::Table => {
                if derive.mode.is_gapless() {
                    quote! {
                        /// Parses a string `s` to return a value of this type
                        #vis fn #ident_from_str_fn(s: &str) -> ::core::option::Option<#ident_enum> {
                            use ::core::iter::Iterator;
                            use ::core::option::Option::{None, Some};
                            for (i, n) in Self::#ident_table_name.iter().enumerate() {
                                if s == *n {
                                    // Safety: the number is known to be a valid enum
                                    return Some(unsafe { ::core::mem::transmute((i as #repr).wrapping_add(#ident_enum::#ident_min as #repr)) });
                                }
                            }
                            None
                        }
                    }
                } else {
                    quote! {
                        /// Parses a string `s` to return a value of this type
                        #vis fn #ident_from_str_fn(s: &str) -> ::core::option::Option<#ident_enum> {
                            use ::core::iter::Iterator;
                            use ::core::option::Option::{None, Some};
                            for (e, n) in Self::#ident_table_enum.iter().zip(Self::#ident_table_name.iter()) {
                                if s == *n {
                                    return Some(*e);
                                }
                            }
                            None
                        }
                    }
                }
            }
        }
    }
}
