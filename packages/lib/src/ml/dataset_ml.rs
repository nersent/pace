use std::path::{Path, PathBuf};

use polars::prelude::*;

use crate::{
    components::{
        component_context::ComponentContext,
        source::{Source, SourceKind},
    },
    features::{
        feature::{Feature, FeatureNamespace, RawFeature},
        feature_builder::FeatureBuilder,
        feature_composer::FeatureComposer,
    },
    ta::relative_strength_index::{
        rsi_feature_builder::RelativeStrengthIndexFeatureBuilder,
        rsi_indicator::{RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig},
        rsi_strategy::{
            RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
            RSI_STRATEGY_THRESHOLD_OVERBOUGHT, RSI_STRATEGY_THRESHOLD_OVERSOLD,
        },
    },
};

pub fn generate_ml_dataset(ctx: ComponentContext, path: &Path) {
    let mut rsi_builder = RelativeStrengthIndexFeatureBuilder::new(
        ctx.clone(),
        RelativeStrengthIndexStrategy::new(
            ctx.clone(),
            RelativeStrengthIndexStrategyConfig {
                threshold_overbought: RSI_STRATEGY_THRESHOLD_OVERBOUGHT,
                threshold_oversold: RSI_STRATEGY_THRESHOLD_OVERSOLD,
            },
            RelativeStrengthIndexIndicator::new(
                ctx.clone(),
                RelativeStrengthIndexIndicatorConfig {
                    length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
        ),
    );

    let mut composer = FeatureComposer::new();

    for cctx in ctx {
        let ctx = cctx.get();

        let mut features: Vec<Box<dyn Feature>> = Vec::new();
        let current_time = ctx.time();

        features.push(Box::new(RawFeature::new(
            "time",
            Some(current_time.unwrap().as_secs_f64()),
        )));

        features.push(Box::new(FeatureNamespace::new(
            String::from("rsi"),
            Box::new(rsi_builder.next()),
        )));

        composer.push_row(features);
    }

    let mut df = composer.to_df();
    let mut file = std::fs::File::create(path).unwrap();

    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
