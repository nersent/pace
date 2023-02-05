use std::path::{Path, PathBuf};

use polars::prelude::*;

use crate::{
    base::{
        asset::{
            asset_feature_builder::AssetFeatureBuilder,
            source::{Source, SourceKind},
        },
        components::{component_context::ComponentContext, component_default::ComponentDefault},
        features::{
            feature::{CombinedFeatures, Feature, FeatureNamespace, RawFeature},
            feature_composer::FeatureComposer,
        },
    },
    content::{
        relative_strength_index_feature_builder::RelativeStrengthIndexFeatureBuilder,
        relative_strength_index_indicator::{
            RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        },
        relative_strength_index_strategy::{
            RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
        },
    },
};

pub fn generate_ml_dataset(ctx: ComponentContext, path: &Path) {
    let mut asset_fb = AssetFeatureBuilder::new(ctx.clone());

    let mut rsi_indicator = RelativeStrengthIndexIndicator::new(
        ctx.clone(),
        RelativeStrengthIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut rsi_strategy = RelativeStrengthIndexStrategy::new(
        ctx.clone(),
        RelativeStrengthIndexStrategyConfig::default(ctx.clone()),
    );
    let mut rsi_fb = RelativeStrengthIndexFeatureBuilder::new(ctx.clone());

    let mut composer = FeatureComposer::new();

    for cctx in ctx {
        let ctx = cctx.get();

        let mut combined = CombinedFeatures::new();

        let rsi = rsi_indicator.next();
        let rsi_action = rsi_strategy.next(rsi);
        let rsi_feat = FeatureNamespace::new(
            "rsi".to_string(),
            rsi_fb
                .next(
                    rsi,
                    rsi_indicator.metadata(),
                    rsi_action,
                    &rsi_strategy.config,
                )
                .as_box(),
        );

        combined.push(rsi_feat.as_box());
        combined.push(asset_fb.next().as_box());
        composer.push(combined.as_box());
    }

    let mut df = composer.to_df();
    let mut file = std::fs::File::create(path).unwrap();

    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
