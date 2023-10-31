use crate::generator::names::Names;
use crate::generator::Derive;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Default)]
pub(crate) struct FeatureTableEnum {
    pub(crate) enabled: bool,
}

impl FeatureTableEnum {
    pub(crate) fn check(&mut self) {}

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        let Derive {
            ident_enum,
            num_values,
            ..
        } = derive;
        let Names {
            ident_table_enum, ..
        } = names;

        let table = derive
            .values
            .iter()
            .map(|(_, (v, _))| {
                quote! {#ident_enum::#v}
            })
            .collect::<Vec<_>>();

        quote! {
            const #ident_table_enum:[#ident_enum; #num_values] = [#(#table),*];
        }
    }
}
