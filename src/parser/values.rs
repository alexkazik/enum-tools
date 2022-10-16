use crate::feature::sorted::FeatureSorted;
use proc_macro2::Span;
use proc_macro_error::abort;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::{Data, Expr, ExprLit, ExprUnary, Fields, Ident, Lit, UnOp};

pub(crate) fn parse_values(span: &Span, data: Data, sorted: FeatureSorted) -> HashMap<i64, Ident> {
    if let Data::Enum(data) = data {
        let mut values = HashMap::new();
        let mut last = -1i64;
        let mut last_name = None;

        for mut v in data.variants {
            let span = v.span();
            if !matches!(v.fields, Fields::Unit) {
                abort!(span, "only unit field items are allowed");
            }
            if sorted.name {
                let next_name = v.ident.to_string();
                if let Some(last_name) = last_name {
                    if last_name >= next_name {
                        abort!(span, "enums are not sorted by name");
                    }
                }
                last_name = Some(next_name);
            }
            if let Some((_, d)) = v.discriminant.take() {
                // check if number is negated
                let mut negate = false;
                let mut num = &d;
                if let Expr::Unary(ExprUnary { attrs, op, expr }) = &d {
                    if attrs.is_empty() {
                        if let UnOp::Neg(_) = op {
                            num = expr;
                            negate = true;
                        }
                    }
                }

                if let Expr::Lit(ExprLit {
                    lit: Lit::Int(i), ..
                }) = num
                {
                    if let Ok(mut i) = i.base10_parse::<i64>() {
                        if negate {
                            i = -i;
                        }
                        if sorted.value && !values.is_empty() && i < last {
                            abort!(span, "enums are not sorted by value");
                        }
                        if values.insert(i, v.ident).is_some() {
                            abort!(span, "Duplicate value");
                        }
                        last = i;
                    } else {
                        abort!(span, "can't be parsed as i64");
                    }
                } else {
                    abort!(span, "only integer literals are allowed");
                }
            } else {
                if last == i64::MAX {
                    abort!(span, "i64 overflow");
                }
                last += 1;
                if values.insert(last, v.ident).is_some() {
                    abort!(span, "Duplicate value");
                }
            }
        }

        if values.is_empty() {
            abort!(span, "no variants found");
        }

        values
    } else {
        abort!(span, "Only enums are supported");
    }
}
