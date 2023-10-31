use crate::generator::features::Features1;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::Visibility;

#[derive(PartialEq, Eq)]
pub(crate) enum AsStrMode {
    Auto,
    Match,
    Table,
}

pub(crate) struct FeatureAsStrFn {
    pub(crate) enabled: bool,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: String,
    pub(crate) mode: AsStrMode,
}

impl FeatureAsStrFn {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("as_str") {
            let (vis, name) = params.get_vis_name("as_str");
            let mode = match params
                .get_str_opt("mode")
                .unwrap_or_else(|| "auto".to_string())
                .as_str()
            {
                "auto" => AsStrMode::Auto,
                "match" => AsStrMode::Match,
                "table" => AsStrMode::Table,
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
                name: "__as_str".to_string(),
                mode: AsStrMode::Auto,
            }
        }
    }

    pub(crate) fn check(&mut self, derive: &Derive, features: &mut Features1) {
        if !self.enabled {
            return;
        }

        if self.mode == AsStrMode::Table {
            if derive.mode.is_gapless() {
                features.min_const.enabled = true;
            }
            features.table_name.enabled = true;
            // range_table is only created when in with holes mode
            features.table_range.enabled = true;
            features.table_range.with_offset = true;
        }
    }

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive {
            repr,
            repr_unsigned,
            ident_enum,
            ..
        } = derive;
        let Names {
            ident_min,
            ident_table_name,
            ident_table_range,
            ident_as_str,
            ..
        } = names;
        let vis = self.vis.as_ref().unwrap_or(&derive.vis_enum);

        match self.mode {
            AsStrMode::Auto => panic!("AsStrMode should've been resolved"),
            AsStrMode::Match => {
                let mut matches = Vec::new();
                for (_, (v, name)) in &derive.values {
                    matches.push(quote! {
                        #ident_enum::#v => #name,
                    });
                }
                quote! {
                    /// Returns the name of this element
                    #vis fn #ident_as_str(self) -> &'static str {
                        match self {
                            #(#matches)*
                        }
                    }
                }
            }
            AsStrMode::Table => {
                if derive.mode.is_gapless() {
                    quote! {
                        /// Returns the name of this element
                        #[inline]
                        #vis fn #ident_as_str(self) -> &'static str {
                            Self::#ident_table_name[(self as #repr).wrapping_sub(Self::#ident_min as #repr) as #repr_unsigned as usize]
                        }
                    }
                } else {
                    quote! {
                        /// Returns the name of this element
                        #vis fn #ident_as_str(self) -> &'static str {
                            use ::core::iter::Iterator;
                            // Safety: all enums are in that table and thus find will succeed
                            let t = unsafe {
                                Self::#ident_table_range
                                    .iter()
                                    .find(|t| t.0.contains(&(self as #repr)))
                                    .unwrap_unchecked()
                            };
                            Self::#ident_table_name[(self as #repr).wrapping_sub(t.1) as #repr_unsigned as usize]
                        }
                    }
                }
            }
        }
    }
}
