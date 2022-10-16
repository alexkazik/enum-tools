use crate::generator::features::Features;
use crate::generator::names::Names;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Visibility;

pub(crate) mod features;
pub(crate) mod names;

pub(crate) enum Mode {
    Gapless,
    WithHoles { value_ranges: Vec<(i64, i64)> },
}

impl Mode {
    #[inline]
    pub(crate) fn is_gapless(&self) -> bool {
        matches!(self, Mode::Gapless)
    }

    #[inline]
    pub(crate) fn is_with_holes(&self) -> bool {
        matches!(self, Mode::WithHoles { .. })
    }
}

pub(crate) struct Derive {
    pub(crate) repr: Ident,
    pub(crate) repr_size_guessed: usize,
    pub(crate) repr_unsigned: Ident,
    pub(crate) ident_enum: Ident,
    pub(crate) vis_enum: Visibility,
    pub(crate) min_key: i64,
    pub(crate) max_key: i64,
    pub(crate) num_values: usize,
    pub(crate) values: Vec<(i64, Ident)>,
    pub(crate) mode: Mode,
}

impl Derive {
    pub(crate) fn generate(self, mut features: Features) -> TokenStream {
        features.resolve(&self);

        // names
        let names = Names::new(&features, &self);

        // generated features
        let as_str_fn = features.as_str_fn.generate(&self, &names);
        let debug_trait = features.debug_trait.generate(&self, &names);
        let display_trait = features.display_trait.generate(&self, &names);
        let from_str_fn = features.from_str_fn.generate(&self, &names);
        let from_str_trait = features.from_str_trait.generate(&self, &names);
        let into_fn = features.into_fn.generate(&self, &names);
        let into_str_trait = features.into_str_trait.generate(&self, &names);
        let into_trait = features.into_trait.generate(&self);
        let (iter_inner, iter_outer) = features.iter.generate(&self, &names);
        let max_const = features.max_const.generate(&self, &names);
        let min_const = features.min_const.generate(&self, &names);
        let (names_inner, names_outer) = features.names.generate(&self, &names);
        let next_back_fn = features.next_back_fn.generate(&self, &names);
        let next_fn = features.next_fn.generate(&self, &names);
        let table_enum = features.table_enum.generate(&self, &names);
        let table_name = features.table_name.generate(&self, &names);
        let table_range = features.table_range.generate(&self, &names);
        let try_from_fn = features.try_from_fn.generate(&self, &names);
        let try_from_trait = features.try_from_trait.generate(&self, &names);

        // other things
        let Derive { ident_enum, .. } = self;

        quote! {
            impl #ident_enum where #ident_enum : ::core::marker::Copy {
                // const
                #min_const
                #max_const
                #table_enum
                #table_name
                #table_range
                // functions
                #as_str_fn
                #from_str_fn
                #into_fn
                #iter_inner
                #names_inner
                #next_fn
                #next_back_fn
                #try_from_fn
            }
            // impl/struct
            #into_str_trait
            #debug_trait
            #display_trait
            #from_str_trait
            #into_trait
            #iter_outer
            #names_outer
            #try_from_trait
        }
    }
}
