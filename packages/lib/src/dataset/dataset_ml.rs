use std::path::{Path, PathBuf};

use crate::{
    base::{
        component_context::ComponentContext,
        features::{
            feature::{Feature, FeatureNamespace, RawFeature},
            feature_builder::FeatureBuilder,
            feature_composer::FeatureComposer,
        },
        implicit::source::{Source, SourceKind},
    },
    features::feature_builder_rsi::FeatureBuilderRSI,
    indicators::indicator_rsi::{IndicatorRSI, IndicatorRSIConfig},
    strategies::strategy_rsi::{
        StrategyRSI, StrategyRSIConfig, STRATEGY_RSI_DEFAULT_OVERBOUGHT_THRESHOLD,
        STRATEGY_RSI_DEFAULT_OVERSOLD_THRESHOLD,
    },
};
use polars::prelude::*;

pub fn generate_ml_dataset(ctx: ComponentContext, path: &Path) {
    let mut builder_rsi = FeatureBuilderRSI::new(
        ctx.clone(),
        StrategyRSI::new(
            ctx.clone(),
            StrategyRSIConfig {
                overbought_threshold: STRATEGY_RSI_DEFAULT_OVERBOUGHT_THRESHOLD,
                oversold_threshold: STRATEGY_RSI_DEFAULT_OVERSOLD_THRESHOLD,
            },
            IndicatorRSI::new(
                ctx.clone(),
                IndicatorRSIConfig {
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
            Box::new(builder_rsi.next()),
        )));

        composer.push_row(features);
    }

    let mut df = composer.to_df();
    let mut file = std::fs::File::create(path).unwrap();

    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
