use crate::parser::feature::FeatureParser;

#[derive(Default)]
pub(crate) struct FeatureSorted {
    pub(crate) name: bool,
    pub(crate) value: bool,
}

impl FeatureSorted {
    pub(crate) fn parse(feature_parser: &mut FeatureParser) -> Self {
        if let Some(mut params) = feature_parser.get("sorted") {
            let name = params.get_bool("name");
            let value = params.get_bool("value");
            params.finish(FeatureSorted { name, value })
        } else {
            FeatureSorted {
                name: false,
                value: false,
            }
        }
    }
}
