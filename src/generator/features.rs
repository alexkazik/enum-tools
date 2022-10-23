use crate::feature::as_str_fn::{AsStrMode, FeatureAsStrFn};
use crate::feature::debug_trait::FeatureDebugTrait;
use crate::feature::display_trait::FeatureDisplayTrait;
use crate::feature::from_str_fn::{FeatureFromStrFn, FromStrFnMode};
use crate::feature::from_str_trait::{FeatureFromStrTrait, FromStrMode};
use crate::feature::into_fn::FeatureIntoFn;
use crate::feature::into_str_trait::FeatureIntoStrTrait;
use crate::feature::into_trait::FeatureIntoTrait;
use crate::feature::iter::{FeatureIter, IterMode};
use crate::feature::max_const::FeatureMaxConst;
use crate::feature::min_const::FeatureMinConst;
use crate::feature::names::FeatureNames;
use crate::feature::next_back_fn::FeatureNextBackFn;
use crate::feature::next_fn::FeatureNextFn;
use crate::feature::table_enum::FeatureTableEnum;
use crate::feature::table_name::FeatureTableName;
use crate::feature::table_range::FeatureTableRange;
use crate::feature::try_from_fn::FeatureTryFromFn;
use crate::feature::try_from_trait::FeatureTryFromTrait;
use crate::generator::Derive;

pub(crate) struct Features {
    pub(crate) as_str_fn: FeatureAsStrFn,
    pub(crate) debug_trait: FeatureDebugTrait,
    pub(crate) display_trait: FeatureDisplayTrait,
    pub(crate) from_str_fn: FeatureFromStrFn,
    pub(crate) from_str_trait: FeatureFromStrTrait,
    pub(crate) into_fn: FeatureIntoFn,
    pub(crate) into_str_trait: FeatureIntoStrTrait,
    pub(crate) into_trait: FeatureIntoTrait,
    pub(crate) iter: FeatureIter,
    pub(crate) max_const: FeatureMaxConst,
    pub(crate) min_const: FeatureMinConst,
    pub(crate) names: FeatureNames,
    pub(crate) next_back_fn: FeatureNextBackFn,
    pub(crate) next_fn: FeatureNextFn,
    pub(crate) table_enum: FeatureTableEnum,
    pub(crate) table_name: FeatureTableName,
    pub(crate) table_range: FeatureTableRange,
    pub(crate) try_from_fn: FeatureTryFromFn,
    pub(crate) try_from_trait: FeatureTryFromTrait,
}

impl Features {
    pub(crate) fn resolve(&mut self, derive: &Derive) {
        self.resolve_enable(derive);
        self.resolve_auto(derive);
        self.resolve_enable(derive);
    }

    fn resolve_auto(&mut self, derive: &Derive) {
        if [
            self.as_str_fn.enabled && self.as_str_fn.mode == AsStrMode::Auto,
            self.from_str_trait.enabled && self.from_str_trait.mode == FromStrMode::Auto,
            self.from_str_fn.enabled && self.from_str_fn.mode == FromStrFnMode::Auto,
        ]
        .into_iter()
        .filter(|b| *b)
        .count()
            > 1
        {
            self.table_name.enabled = true;
        }

        let table_enum = self.table_enum.enabled;
        let table_name = self.table_name.enabled;

        if self.as_str_fn.enabled && self.as_str_fn.mode == AsStrMode::Auto {
            if table_name {
                self.as_str_fn.mode = AsStrMode::Table;
            } else {
                self.as_str_fn.mode = AsStrMode::Match;
            }
        }

        if self.from_str_trait.enabled && self.from_str_trait.mode == FromStrMode::Auto {
            if table_name {
                self.from_str_trait.mode = FromStrMode::Table;
            } else {
                self.from_str_trait.mode = FromStrMode::Match;
            }
        }

        if self.from_str_fn.enabled && self.from_str_fn.mode == FromStrFnMode::Auto {
            if table_name {
                self.from_str_fn.mode = FromStrFnMode::Table;
            } else {
                self.from_str_fn.mode = FromStrFnMode::Match;
            }
        }

        if self.iter.enabled && self.iter.mode == IterMode::Auto {
            if derive.mode.is_gapless() {
                self.iter.mode = IterMode::Range;
            } else {
                // has holes
                if table_enum {
                    self.iter.mode = IterMode::Table;
                } else if derive.num_values * derive.repr_size_guessed <= 8 {
                    self.iter.mode = IterMode::TableInline;
                } else {
                    self.iter.mode = IterMode::NextAndBack;
                }
            }
        }
    }

    fn resolve_enable(&mut self, derive: &Derive) {
        let mut features2 = Features2 {
            as_str_fn: &mut self.as_str_fn,
            min_const: &mut self.min_const,
            max_const: &mut self.max_const,
            next_fn: &mut self.next_fn,
            next_back_fn: &mut self.next_back_fn,
            table_enum: &mut self.table_enum,
            table_name: &mut self.table_name,
            table_range: &mut self.table_range,
        };
        self.debug_trait.check(&mut features2);
        self.display_trait.check(&mut features2);
        self.into_str_trait.check(&mut features2);

        let mut features1 = Features1 {
            min_const: features2.min_const,
            max_const: features2.max_const,
            next_fn: features2.next_fn,
            next_back_fn: features2.next_back_fn,
            table_enum: features2.table_enum,
            table_name: features2.table_name,
            table_range: features2.table_range,
        };
        features2.as_str_fn.check(derive, &mut features1);
        self.from_str_trait.check(derive, &mut features1);
        self.from_str_fn.check(derive, &mut features1);
        self.iter.check(derive, &mut features1);
        self.names.check(&mut features1);

        let mut features0 = Features0 {
            min_const: features1.min_const,
            max_const: features1.max_const,
            table_range: features1.table_range,
        };
        features1.next_fn.check(&mut features0);
        features1.next_back_fn.check(&mut features0);
        self.try_from_trait.check(&mut features0);
        self.try_from_fn.check(&mut features0);

        // totally stand-alone
        self.into_trait.check();
        self.into_fn.check();
        features0.min_const.check();
        features0.max_const.check();
        features1.table_enum.check();
        features1.table_name.check();
        features0.table_range.check();
    }
}

pub(crate) struct Features0<'a> {
    pub(crate) max_const: &'a mut FeatureMaxConst,
    pub(crate) min_const: &'a mut FeatureMinConst,
    pub(crate) table_range: &'a mut FeatureTableRange,
}

pub(crate) struct Features1<'a> {
    pub(crate) max_const: &'a mut FeatureMaxConst,
    pub(crate) min_const: &'a mut FeatureMinConst,
    pub(crate) next_fn: &'a mut FeatureNextFn,
    pub(crate) next_back_fn: &'a mut FeatureNextBackFn,
    pub(crate) table_enum: &'a mut FeatureTableEnum,
    pub(crate) table_name: &'a mut FeatureTableName,
    pub(crate) table_range: &'a mut FeatureTableRange,
}

pub(crate) struct Features2<'a> {
    pub(crate) as_str_fn: &'a mut FeatureAsStrFn,
    pub(crate) max_const: &'a mut FeatureMaxConst,
    pub(crate) min_const: &'a mut FeatureMinConst,
    pub(crate) next_fn: &'a mut FeatureNextFn,
    pub(crate) next_back_fn: &'a mut FeatureNextBackFn,
    pub(crate) table_enum: &'a mut FeatureTableEnum,
    pub(crate) table_name: &'a mut FeatureTableName,
    pub(crate) table_range: &'a mut FeatureTableRange,
}
