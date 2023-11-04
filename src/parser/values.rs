use crate::feature::sorted::FeatureSorted;
use crate::parser::error::Error;
use proc_macro2::Span;
use proc_macro_error::{abort, emit_error};
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::{Data, Expr, ExprLit, ExprUnary, Fields, Ident, Lit, Meta, MetaNameValue, UnOp};

pub(crate) fn parse_values(
    span: Span,
    data: Data,
    sorted: FeatureSorted,
) -> Vec<(i64, (Ident, String))> {
    if let Data::Enum(data) = data {
        let mut values = HashMap::new();
        let mut last = -1i64;
        let mut last_name = None;

        for mut v in data.variants {
            let span = v.span();
            if !matches!(v.fields, Fields::Unit) {
                emit_error!(span, Error::OnlyUnitField);
            }
            let mut name = v.ident.to_string();
            for a in v.attrs {
                if a.path().is_ident("enum_tools") {
                    if let Meta::List(meta_list) = &a.meta {
                        let nested = meta_list
                            .parse_args()
                            .unwrap_or_else(|e| abort!(meta_list, Error::MetaParseError(e)));
                        if let Meta::NameValue(MetaNameValue {
                            path,
                            value:
                                Expr::Lit(ExprLit {
                                    lit: Lit::Str(lit_str),
                                    ..
                                }),
                            ..
                        }) = nested
                        {
                            if !path.is_ident("rename") {
                                emit_error!(a, Error::UnsupportedAttributeType);
                            } else {
                                name = lit_str.value();
                            }
                        } else {
                            emit_error!(a, Error::UnsupportedAttributeType);
                        }
                    } else {
                        emit_error!(a, Error::UnsupportedAttributeType);
                    }
                }
            }
            if sorted.name {
                if let Some(last_name) = last_name {
                    if last_name >= name {
                        emit_error!(span, Error::FieldsNotNameSorted);
                    }
                }
                last_name = Some(name.clone());
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
                            emit_error!(span, Error::FieldsNotValueSorted);
                        }
                        if values.insert(i, (v.ident, name)).is_some() {
                            emit_error!(span, Error::DuplicateValue);
                        }
                        last = i;
                    } else {
                        emit_error!(span, Error::NoI64);
                    }
                } else {
                    emit_error!(span, Error::NotInteger);
                }
            } else {
                if last == i64::MAX {
                    emit_error!(span, Error::I64Overflow);
                }
                last = last.wrapping_add(1);
                if values.insert(last, (v.ident, name)).is_some() {
                    emit_error!(span, Error::DuplicateValue);
                }
            }
        }

        let mut values = values
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect::<Vec<_>>();
        values.sort_by_key(|v| v.0);

        if values.is_empty() {
            abort!(span, Error::NoVariantsFound);
        }

        values
    } else {
        abort!(span, Error::NoEnum);
    }
}
