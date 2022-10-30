use crate::generator::features::Features1;
use crate::generator::names::Names;
use crate::generator::Derive;
use crate::parser::feature::FeatureParser;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;

#[derive(PartialEq, Eq)]
pub(crate) enum FromStrMode {
    Auto,
    Match,
    Table,
}

pub(crate) struct FeatureFromStrTrait {
    pub(crate) enabled: bool,
    pub(crate) mode: FromStrMode,
}

impl FeatureFromStrTrait {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("FromStr") {
            let mode = match params
                .get_str_opt("mode")
                .unwrap_or_else(|| "auto".to_string())
                .as_str()
            {
                "auto" => FromStrMode::Auto,
                "match" => FromStrMode::Match,
                "table" => FromStrMode::Table,
                _ => abort!(params.span(), "invalid mode"),
            };
            params.finish(Self {
                enabled: true,
                mode,
            })
        } else {
            Self {
                enabled: false,
                mode: FromStrMode::Auto,
            }
        }
    }

    pub(crate) fn check(&mut self, derive: &Derive, features: &mut Features1) {
        if !self.enabled {
            return;
        }

        if self.mode == FromStrMode::Table {
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
            ..
        } = names;

        match self.mode {
            FromStrMode::Auto => panic!("FromStrMode should've been resolved"),
            FromStrMode::Match => {
                let mut matches = Vec::new();
                for (_, v) in derive.values.iter() {
                    let name = v.to_string();
                    matches.push(quote! {
                        #name => Ok(#ident_enum::#v),
                    });
                }
                quote! {
                    impl ::core::str::FromStr for #ident_enum {
                        type Err = ();

                        fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                            use ::core::result::Result::{Ok, Err};
                            match s {
                                #(#matches)*
                                _ => Err(()),
                            }
                        }
                    }
                }
            }
            FromStrMode::Table => {
                if derive.mode.is_gapless() {
                    quote! {
                        impl ::core::str::FromStr for #ident_enum {
                            type Err = ();

                            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                                use ::core::iter::Iterator;
                                use ::core::result::Result::{Ok, Err};
                                for (i, n) in Self::#ident_table_name.iter().enumerate() {
                                    if s == *n {
                                        // Safety: the number is known to be a valid enum
                                        return Ok(unsafe { ::core::mem::transmute((i as #repr).wrapping_add(#ident_enum::#ident_min as #repr)) });
                                    }
                                }
                                Err(())
                            }
                        }
                    }
                } else {
                    quote! {
                        impl ::core::str::FromStr for #ident_enum {
                            type Err = ();

                            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                                use ::core::iter::Iterator;
                                use ::core::result::Result::{Ok, Err};
                                for (e, n) in Self::#ident_table_enum.iter().zip(Self::#ident_table_name.iter()) {
                                    if s == *n {
                                        return Ok(*e);
                                    }
                                }
                                Err(())
                            }
                        }
                    }
                }
            }
        }
    }
}
