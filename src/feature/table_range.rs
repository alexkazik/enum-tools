use crate::generator::names::Names;
use crate::generator::{Derive, Mode};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitInt;

#[derive(Default)]
pub(crate) struct FeatureTableRange {
    pub(crate) enabled: bool,
    pub(crate) with_offset: bool,
}

impl FeatureTableRange {
    pub(crate) fn check(&mut self) {}

    pub(crate) fn generate(self, derive: &Derive, names: &Names) -> TokenStream {
        if !self.enabled {
            return TokenStream::new();
        }

        if let Mode::WithHoles { ref value_ranges } = derive.mode {
            let FeatureTableRange { with_offset, .. } = self;
            let Derive { repr, .. } = derive;
            let Names {
                ident_table_range, ..
            } = names;

            let mut ofs = 0;
            let hl = value_ranges.len();
            let mut h = Vec::new();
            for (b0, e0) in value_ranges.iter() {
                let b1 = LitInt::new(&format!("{b0}{repr}"), Span::call_site());
                let e1 = LitInt::new(&format!("{e0}{repr}"), Span::call_site());
                let o1 = LitInt::new(&format!("{ofs}{repr}"), Span::call_site());
                h.push(if with_offset {
                    quote! {(#b1 ..= #e1, #b1.wrapping_sub(#o1))}
                } else {
                    quote! {(#b1 ..= #e1, ())}
                });
                ofs += e0 - b0 + 1;
            }

            if with_offset {
                quote! {
                    const #ident_table_range:[(::core::ops::RangeInclusive<#repr>, #repr); #hl] = [#(#h),*];
                }
            } else {
                quote! {
                    const #ident_table_range:[(::core::ops::RangeInclusive<#repr>, ()); #hl] = [#(#h),*];
                }
            }
        } else {
            TokenStream::new()
        }
    }
}
