use crate::generator::names::Names;
use crate::generator::Derive;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Default)]
pub(crate) struct FeatureTableName {
    pub(crate) enabled: bool,
}

impl FeatureTableName {
    pub(crate) fn check(&mut self) {}

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive { num_values, .. } = derive;
        let Names {
            ident_table_name, ..
        } = names;

        let table = derive
            .values
            .iter()
            .map(|(_, v)| {
                let name = v.to_string();
                quote! {#name}
            })
            .collect::<Vec<_>>();

        quote! {
            const #ident_table_name:[&'static str; #num_values] = [#(#table),*];
        }
    }
}
