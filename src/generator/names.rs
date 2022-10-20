use crate::generator::{Derive, Features};
use proc_macro2::{Ident, Span};

pub(crate) struct Names {
    pub(crate) ident_as_str: Ident,
    pub(crate) ident_from_str_fn: Ident,
    pub(crate) ident_iter_fn: Ident,
    pub(crate) ident_iter_struct: Ident,
    pub(crate) ident_max: Ident,
    pub(crate) ident_min: Ident,
    pub(crate) ident_names_fn: Ident,
    pub(crate) ident_names_struct: Ident,
    pub(crate) ident_next: Ident,
    pub(crate) ident_next_back: Ident,
    pub(crate) ident_table_enum: Ident,
    pub(crate) ident_table_name: Ident,
    pub(crate) ident_table_range: Ident,
    pub(crate) ident_to_fn: Ident,
    pub(crate) ident_try_from_fn: Ident,
}

impl Names {
    pub(crate) fn new(features: &Features, derive: &Derive) -> Self {
        let Derive { ident_enum, .. } = derive;
        Self {
            ident_as_str: Ident::new(&features.as_str_fn.name, Span::call_site()),
            ident_from_str_fn: Ident::new(&features.from_str_fn.name, Span::call_site()),
            ident_iter_fn: Ident::new(&features.iter.name, Span::call_site()),
            ident_iter_struct: match features.iter.struct_name {
                None => Ident::new(&format!("{ident_enum}Iter"), Span::call_site()),
                Some(ref name) => Ident::new(name, Span::call_site()),
            },
            ident_max: Ident::new(&features.max_const.name, Span::call_site()),
            ident_min: Ident::new(&features.min_const.name, Span::call_site()),
            ident_names_fn: Ident::new(&features.names.name, Span::call_site()),
            ident_names_struct: match features.names.struct_name {
                None => Ident::new(&format!("{ident_enum}Names"), Span::call_site()),
                Some(ref name) => Ident::new(name, Span::call_site()),
            },
            ident_next: Ident::new(&features.next_fn.name, Span::call_site()),
            ident_next_back: Ident::new(&features.next_back_fn.name, Span::call_site()),
            ident_table_enum: Ident::new("__ENUM", Span::call_site()),
            ident_table_name: Ident::new("__NAME", Span::call_site()),
            ident_table_range: Ident::new("__RANGES", Span::call_site()),
            ident_to_fn: Ident::new(&features.into_fn.name, Span::call_site()),
            ident_try_from_fn: Ident::new(&features.try_from_fn.name, Span::call_site()),
        }
    }
}
