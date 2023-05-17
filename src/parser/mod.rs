use crate::feature::as_str_fn::FeatureAsStrFn;
use crate::feature::debug_trait::FeatureDebugTrait;
use crate::feature::display_trait::FeatureDisplayTrait;
use crate::feature::from_str_fn::FeatureFromStrFn;
use crate::feature::from_str_trait::FeatureFromStrTrait;
use crate::feature::into_fn::FeatureIntoFn;
use crate::feature::into_str_trait::FeatureIntoStrTrait;
use crate::feature::into_trait::FeatureIntoTrait;
use crate::feature::iter::FeatureIter;
use crate::feature::max_const::FeatureMaxConst;
use crate::feature::min_const::FeatureMinConst;
use crate::feature::names::FeatureNames;
use crate::feature::next_back_fn::FeatureNextBackFn;
use crate::feature::next_fn::FeatureNextFn;
use crate::feature::range_fn::FeatureRangeFn;
use crate::feature::sorted::FeatureSorted;
use crate::feature::try_from_fn::FeatureTryFromFn;
use crate::feature::try_from_trait::FeatureTryFromTrait;
use crate::generator::features::Features;
use crate::generator::{Derive, Mode};
use crate::parser::attr::parse_attrs;
use crate::parser::values::parse_values;
use proc_macro_error::abort;
use syn::spanned::Spanned;
use syn::{DeriveInput, Ident};

pub(crate) mod attr;
pub(crate) mod error;
pub(crate) mod feature;
pub(crate) mod params;
pub(crate) mod values;

impl Derive {
    pub(crate) fn parse(input: DeriveInput) -> (Derive, Features) {
        let span = input.span();
        let vis_enum = input.vis;
        let ident_enum = input.ident;

        let (repr, mut feature_parser) = parse_attrs(span, input.attrs);

        let (repr_size_guessed, repr_unsigned) = match repr.to_string().as_str() {
            "u8" | "i8" => (1, "u8"),
            "u16" | "i16" => (2, "u16"),
            "u32" | "i32" => (4, "u32"),
            "usize" | "isize" => (4, "usize"), // the size of u/isize on the target system is not known
            "u64" | "i64" => (8, "u64"),
            "u128" | "i128" => (16, "u128"),
            _ => abort!(repr, "unsupported repr"),
        };
        let repr_unsigned = Ident::new(repr_unsigned, repr.span());

        let sorted = FeatureSorted::parse(&mut feature_parser);

        let values = parse_values(span, input.data, sorted);

        let min_key = values.first().unwrap().0;
        let max_key = values.last().unwrap().0;

        let num_values = values.len();
        if num_values >= u16::MAX.into() {
            abort!(
                span,
                "too many values (at most u16::MAX-1 are currently supported)"
            )
        }

        let value_ranges = {
            let mut value_ranges = Vec::new();
            let mut begin = min_key;
            let mut last = begin;
            for (i, _) in values.iter().skip(1) {
                if *i != last + 1 {
                    value_ranges.push((begin, last));
                    begin = *i;
                }
                last = *i;
            }
            value_ranges.push((begin, last));
            value_ranges
        };

        let mode = if value_ranges.len() == 1 {
            Mode::Gapless
        } else {
            Mode::WithHoles { value_ranges }
        };

        let derive = Derive {
            repr,
            repr_size_guessed,
            repr_unsigned,
            ident_enum,
            vis_enum,
            min_key,
            max_key,
            num_values,
            values,
            mode,
        };

        let features = Features {
            as_str_fn: FeatureAsStrFn::parse(&mut feature_parser),
            debug_trait: FeatureDebugTrait::parse(&mut feature_parser),
            display_trait: FeatureDisplayTrait::parse(&mut feature_parser),
            from_str_fn: FeatureFromStrFn::parse(&mut feature_parser),
            from_str_trait: FeatureFromStrTrait::parse(&mut feature_parser),
            into_fn: FeatureIntoFn::parse(&mut feature_parser),
            into_str_trait: FeatureIntoStrTrait::parse(&mut feature_parser),
            into_trait: FeatureIntoTrait::parse(&mut feature_parser),
            iter: FeatureIter::parse(&mut feature_parser),
            max_const: FeatureMaxConst::parse(&mut feature_parser),
            min_const: FeatureMinConst::parse(&mut feature_parser),
            names: FeatureNames::parse(&mut feature_parser),
            next_back_fn: FeatureNextBackFn::parse(&mut feature_parser),
            next_fn: FeatureNextFn::parse(&mut feature_parser),
            range_fn: FeatureRangeFn::parse(&mut feature_parser),
            table_enum: Default::default(),
            table_name: Default::default(),
            table_range: Default::default(),
            try_from_fn: FeatureTryFromFn::parse(&mut feature_parser),
            try_from_trait: FeatureTryFromTrait::parse(&mut feature_parser),
        };

        feature_parser.finish();

        (derive, features)
    }
}
