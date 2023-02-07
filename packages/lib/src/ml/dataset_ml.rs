use std::{
    fs, io,
    path::{Path, PathBuf},
};

use kdam::tqdm;
use polars::prelude::*;

use crate::{
    base::{
        asset::{
            asset_feature_builder::AssetFeatureBuilder,
            source::{Source, SourceKind},
            timeframe::Timeframe,
        },
        components::{component_context::ComponentContext, component_default::ComponentDefault},
        features::{
            feature::{CombinedFeatures, Feature, FeatureNamespace, RawFeature},
            feature_composer::FeatureComposer,
        },
    },
    content::{
        aroon_feature_builder::AroonFeatureBuilder,
        aroon_indicator::{AroonIndicator, AroonIndicatorConfig},
        aroon_strategy::AroonStrategy,
        awesome_oscillator_feature_builder::{
            AwesomeOscillatorFeature, AwesomeOscillatorFeatureBuilder,
        },
        awesome_oscillator_indicator::{
            AwesomeOscillatorIndicator, AwesomeOscillatorIndicatorConfig,
        },
        awesome_oscillator_strategy::{AwesomeOscillatorStrategy, AwesomeOscillatorStrategyConfig},
        balance_of_power_feature_builder::BalanceOfPowerFeatureBuilder,
        balance_of_power_indicator::BalanceOfPowerIndicator,
        balance_of_power_strategy::{BalanceOfPowerStrategy, BalanceOfPowerStrategyConfig},
        bollinger_bands_pb_feature_builder::BollingerBandsPercentBFeatureBuilder,
        bollinger_bands_pb_indicator::{
            BollingerBandsPercentBIndicator, BollingerBandsPercentBIndicatorConfig,
        },
        bollinger_bands_pb_strategy::{
            BollingerBandsPercentBStrategy, BollingerBandsPercentBStrategyConfig,
        },
        bollinger_bands_width_feature_builder::BollingerBandsWidthFeatureBuilder,
        bollinger_bands_width_indicator::{
            BollingerBandsWidthIndicator, BollingerBandsWidthIndicatorConfig,
        },
        chaikin_money_flow_feature_builder::ChaikinMoneyFlowFeatureBuilder,
        chaikin_money_flow_indicator::{
            ChaikinMoneyFlowIndicator, ChaikinMoneyFlowIndicatorConfig,
        },
        chaikin_money_flow_strategy::{ChaikinMoneyFlowStrategy, ChaikinMoneyFlowStrategyConfig},
        chande_momentum_oscillator_feature_builder::ChandeMomentumOscillatorFeatureBuilder,
        chande_momentum_oscillator_indicator::{
            ChandeMomentumOscillatorIndicator, ChandeMomentumOscillatorIndicatorConfig,
        },
        chande_momentum_oscillator_strategy::{
            ChandeMomentumOscillatorStrategy, ChandeMomentumOscillatorStrategyConfig,
        },
        choppiness_index_feature_builder::ChoppinessIndexFeatureBuilder,
        choppiness_index_indicator::{ChoppinessIndexIndicator, ChoppinessIndexIndicatorConfig},
        commodity_channel_index_feature_builder::CommodityChannelIndexFeatureBuilder,
        commodity_channel_index_indicator::{
            CommodityChannelIndexIndicator, CommodityChannelIndexIndicatorConfig,
        },
        commodity_channel_index_strategy::{
            CommodityChannelIndexStrategy, CommodityChannelIndexStrategyConfig,
        },
        connors_relative_strength_index_feature_builder::ConnorsRelativeStrengthIndexFeatureBuilder,
        connors_relative_strength_index_indicator::{
            ConnorsRelativeStrengthIndexIndicator, ConnorsRelativeStrengthIndexIndicatorConfig,
        },
        connors_relative_strength_index_strategy::{
            ConnorsRelativeStrengthIndexStrategy, ConnorsRelativeStrengthIndexStrategyConfig,
        },
        coppock_curve_feature_builder::CoppockCurveFeatureBuilder,
        coppock_curve_indicator::{CoppockCurveIndicator, CoppockCurveIndicatorConfig},
        coppock_curve_strategy::{CoppockCurveStrategy, CoppockCurveStrategyConfig},
        directional_movement_index_feature_builder::DirectionalMovementIndexFeatureBuilder,
        directional_movement_index_indicator::{
            DirectionalMovementIndexIndicator, DirectionalMovementIndexIndicatorConfig,
        },
        directional_movement_index_strategy::{
            DirectionalMovementIndexStrategy, DirectionalMovementIndexStrategyConfig,
        },
        price_oscillator_feature_builder::PriceOscillatorFeatureBuilder,
        price_oscillator_indicator::{PriceOscillatorIndicator, PriceOscillatorIndicatorConfig},
        price_oscillator_strategy::{PriceOscillatorStrategy, PriceOscillatorStrategyConfig},
        relative_strength_index_feature_builder::RelativeStrengthIndexFeatureBuilder,
        relative_strength_index_indicator::{
            RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        },
        relative_strength_index_strategy::{
            RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
        },
        relative_vigor_index_feature_builder::RelativeVigorIndexFeatureBuilder,
        relative_vigor_index_indicator::{
            RelativeVigorIndexIndicator, RelativeVigorIndexIndicatorConfig,
        },
        relative_vigor_index_strategy::RelativeVigorIndexStrategy,
        relative_volatility_index_feature_builder::RelativeVolatilityIndexFeatureBuilder,
        relative_volatility_index_indicator::{
            RelativeVolatilityIndexIndicator, RelativeVolatilityIndexIndicatorConfig,
        },
        relative_volatility_index_strategy::{
            RelativeVolatilityIndexStrategy, RelativeVolatilityIndexStrategyConfig,
        },
        stoch_relative_strength_index_feature_builder::StochRelativeStrengthIndexFeatureBuilder,
        stoch_relative_strength_index_indicator::{
            StochRelativeStrengthIndexIndicator, StochRelativeStrengthIndexIndicatorConfig,
        },
        stoch_relative_strength_index_strategy::{
            StochRelativeStrengthIndexStrategy, StochRelativeStrengthIndexStrategyConfig,
        },
        ultimate_oscillator_feature_builder::UltimateOscillatorFeatureBuilder,
        ultimate_oscillator_indicator::{
            UltimateOscillatorIndicator, UltimateOscillatorIndicatorConfig,
        },
        volume_oscillator_feature_builder::VolumeOscillatorFeatureBuilder,
        volume_oscillator_indicator::{VolumeOscillatorIndicator, VolumeOscillatorIndicatorConfig},
        volume_oscillator_strategy::{VolumeOscillatorStrategy, VolumeOscillatorStrategyConfig},
        vortex_feature_builder::VortexFeatureBuilder,
        vortex_indicator::{VortexIndicator, VortexIndicatorConfig},
        vortex_strategy::{self, VortexStrategy},
        williams_percent_range_feature_builder::WilliamsPercentRangeFeatureBuilder,
        williams_percent_range_indicator::{
            WilliamsPercentRangeIndicator, WilliamsPercentRangeIndicatorConfig,
        },
        williams_percent_range_strategy::{
            WilliamsPercentRangeStrategy, WilliamsPercentRangeStrategyConfig,
        },
    },
    utils::{
        fs::get_filename,
        polars::{read_df, save_df},
    },
};

pub fn generate_ml_dataset_from_ctx(ctx: ComponentContext, out_path: &Path) {
    let mut composer = FeatureComposer::new();
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

    let mut aroon_indicator =
        AroonIndicator::new(ctx.clone(), AroonIndicatorConfig::default(ctx.clone()));
    let mut aroon_strategy = AroonStrategy::new(ctx.clone());
    let mut aroon_fb = AroonFeatureBuilder::new(ctx.clone());

    let mut ao_indicator = AwesomeOscillatorIndicator::new(
        ctx.clone(),
        AwesomeOscillatorIndicatorConfig::default(ctx.clone()),
    );
    let mut ao_strategy = AwesomeOscillatorStrategy::new(
        ctx.clone(),
        AwesomeOscillatorStrategyConfig::default(ctx.clone()),
    );
    let mut ao_fb = AwesomeOscillatorFeatureBuilder::new(ctx.clone());

    let mut bp_indicator = BalanceOfPowerIndicator::new(ctx.clone());
    let mut bp_strategy = BalanceOfPowerStrategy::new(
        ctx.clone(),
        BalanceOfPowerStrategyConfig::default(ctx.clone()),
    );
    let mut bp_fb = BalanceOfPowerFeatureBuilder::new(ctx.clone());

    let mut bbpb_indicator = BollingerBandsPercentBIndicator::new(
        ctx.clone(),
        BollingerBandsPercentBIndicatorConfig::default(ctx.clone()),
    );
    let mut bbpb_strategy = BollingerBandsPercentBStrategy::new(
        ctx.clone(),
        BollingerBandsPercentBStrategyConfig::default(ctx.clone()),
    );
    let mut bbpb_fb = BollingerBandsPercentBFeatureBuilder::new(ctx.clone());

    let mut bbw_indicator = BollingerBandsWidthIndicator::new(
        ctx.clone(),
        BollingerBandsWidthIndicatorConfig::default(ctx.clone()),
    );
    let mut bbw_fb = BollingerBandsWidthFeatureBuilder::new(ctx.clone());

    let mut cmf_indicator = ChaikinMoneyFlowIndicator::new(
        ctx.clone(),
        ChaikinMoneyFlowIndicatorConfig::default(ctx.clone()),
    );
    let mut cmf_strategy = ChaikinMoneyFlowStrategy::new(
        ctx.clone(),
        ChaikinMoneyFlowStrategyConfig::default(ctx.clone()),
    );
    let mut cmf_fb = ChaikinMoneyFlowFeatureBuilder::new(ctx.clone());

    let mut cmo_indicator = ChandeMomentumOscillatorIndicator::new(
        ctx.clone(),
        ChandeMomentumOscillatorIndicatorConfig::default(ctx.clone()),
    );
    let mut cmo_strategy = ChandeMomentumOscillatorStrategy::new(
        ctx.clone(),
        ChandeMomentumOscillatorStrategyConfig::default(ctx.clone()),
    );
    let mut cmo_fb = ChandeMomentumOscillatorFeatureBuilder::new(ctx.clone());

    let mut ci_indicator = ChoppinessIndexIndicator::new(
        ctx.clone(),
        ChoppinessIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut ci_fb = ChoppinessIndexFeatureBuilder::new(ctx.clone());

    let mut cci_indicator = CommodityChannelIndexIndicator::new(
        ctx.clone(),
        CommodityChannelIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut cci_strategy = CommodityChannelIndexStrategy::new(
        ctx.clone(),
        CommodityChannelIndexStrategyConfig::default(ctx.clone()),
    );
    let mut cci_fb = CommodityChannelIndexFeatureBuilder::new(ctx.clone());

    let mut connors_rsi_indicator = ConnorsRelativeStrengthIndexIndicator::new(
        ctx.clone(),
        ConnorsRelativeStrengthIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut connors_rsi_strategy = ConnorsRelativeStrengthIndexStrategy::new(
        ctx.clone(),
        ConnorsRelativeStrengthIndexStrategyConfig::default(ctx.clone()),
    );
    let mut connors_rsi_fb = ConnorsRelativeStrengthIndexFeatureBuilder::new(ctx.clone());

    let mut cc_indicator = CoppockCurveIndicator::new(
        ctx.clone(),
        CoppockCurveIndicatorConfig::default(ctx.clone()),
    );
    let mut cc_strategy = CoppockCurveStrategy::new(
        ctx.clone(),
        CoppockCurveStrategyConfig::default(ctx.clone()),
    );
    let mut cc_fb = CoppockCurveFeatureBuilder::new(ctx.clone());

    let mut dmi_indicator = DirectionalMovementIndexIndicator::new(
        ctx.clone(),
        DirectionalMovementIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut dmi_strategy = DirectionalMovementIndexStrategy::new(
        ctx.clone(),
        DirectionalMovementIndexStrategyConfig::default(ctx.clone()),
    );
    let mut dmi_fb = DirectionalMovementIndexFeatureBuilder::new(ctx.clone());

    let mut po_indicator = PriceOscillatorIndicator::new(
        ctx.clone(),
        PriceOscillatorIndicatorConfig::default(ctx.clone()),
    );
    let mut po_strategy = PriceOscillatorStrategy::new(
        ctx.clone(),
        PriceOscillatorStrategyConfig::default(ctx.clone()),
    );
    let mut po_fb = PriceOscillatorFeatureBuilder::new(ctx.clone());

    let mut rvgi_indicator = RelativeVigorIndexIndicator::new(
        ctx.clone(),
        RelativeVigorIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut rvgi_strategy = RelativeVigorIndexStrategy::new(ctx.clone());
    let mut rvgi_fb = RelativeVigorIndexFeatureBuilder::new(ctx.clone());

    let mut rvi_indicator = RelativeVolatilityIndexIndicator::new(
        ctx.clone(),
        RelativeVolatilityIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut rvi_strategy = RelativeVolatilityIndexStrategy::new(
        ctx.clone(),
        RelativeVolatilityIndexStrategyConfig::default(ctx.clone()),
    );
    let mut rvi_fb = RelativeVolatilityIndexFeatureBuilder::new(ctx.clone());

    let mut stoch_rsi_indicator = StochRelativeStrengthIndexIndicator::new(
        ctx.clone(),
        StochRelativeStrengthIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut stoch_rsi_strategy = StochRelativeStrengthIndexStrategy::new(
        ctx.clone(),
        StochRelativeStrengthIndexStrategyConfig::default(ctx.clone()),
    );
    let mut stoch_rsi_fb = StochRelativeStrengthIndexFeatureBuilder::new(ctx.clone());

    let mut uo_indicator = UltimateOscillatorIndicator::new(
        ctx.clone(),
        UltimateOscillatorIndicatorConfig::default(ctx.clone()),
    );
    let mut uo_fb = UltimateOscillatorFeatureBuilder::new(ctx.clone());

    let mut vo_indicator = VolumeOscillatorIndicator::new(
        ctx.clone(),
        VolumeOscillatorIndicatorConfig::default(ctx.clone()),
    );
    let mut vo_strategy = VolumeOscillatorStrategy::new(
        ctx.clone(),
        VolumeOscillatorStrategyConfig::default(ctx.clone()),
    );
    let mut vo_fb = VolumeOscillatorFeatureBuilder::new(ctx.clone());

    let mut vortex_indicator =
        VortexIndicator::new(ctx.clone(), VortexIndicatorConfig::default(ctx.clone()));
    let mut vortex_strategy = VortexStrategy::new(ctx.clone());
    let mut vortex_fb = VortexFeatureBuilder::new(ctx.clone());

    let mut wpr_indicator = WilliamsPercentRangeIndicator::new(
        ctx.clone(),
        WilliamsPercentRangeIndicatorConfig::default(ctx.clone()),
    );
    let mut wpr_strategy = WilliamsPercentRangeStrategy::new(
        ctx.clone(),
        WilliamsPercentRangeStrategyConfig::default(ctx.clone()),
    );
    let mut wpr_fb = WilliamsPercentRangeFeatureBuilder::new(ctx.clone());

    for cctx in ctx {
        let ctx = cctx.get();

        if !ctx.is_ohlcv_valid() {
            break;
        }

        let mut combined = CombinedFeatures::new();

        let rsi = rsi_indicator.next();
        let rsi_trade = rsi_strategy.next(rsi);
        let rsi_feat = FeatureNamespace::new(
            "rsi",
            rsi_fb
                .next(
                    rsi,
                    rsi_indicator.metadata(),
                    rsi_trade,
                    &rsi_strategy.config,
                )
                .to_box(),
        );
        combined.push(rsi_feat.to_box());

        let aroon = aroon_indicator.next();
        let aroon_trade = aroon_strategy.next(&aroon);
        let aroon_feat = FeatureNamespace::new(
            "aroon",
            aroon_fb
                .next(&aroon, aroon_trade, aroon_strategy.metadata())
                .to_box(),
        );
        combined.push(aroon_feat.to_box());

        let ao = ao_indicator.next();
        let ao_trade = ao_strategy.next(ao);
        let ao_feat =
            FeatureNamespace::new("ao", ao_fb.next(ao, ao_trade, &ao_strategy.config).to_box());
        combined.push(ao_feat.to_box());

        let bp = bp_indicator.next();
        let bp_trade = bp_strategy.next(bp);
        let bp_feat =
            FeatureNamespace::new("bp", bp_fb.next(bp, bp_trade, &bp_strategy.config).to_box());
        combined.push(bp_feat.to_box());

        let bbpb = bbpb_indicator.next();
        let bbpb_trade = bbpb_strategy.next(bbpb);
        let bbpb_feat = FeatureNamespace::new(
            "bbpb",
            bbpb_fb
                .next(bbpb, bbpb_trade, &bbpb_strategy.config)
                .to_box(),
        );
        combined.push(bbpb_feat.to_box());

        let bbw = bbw_indicator.next();
        let bbw_feat = FeatureNamespace::new("bbw", bbw_fb.next(bbw).to_box());
        combined.push(bbw_feat.to_box());

        let cmf = cmf_indicator.next();
        let cmf_trade = cmf_strategy.next(cmf);
        let cmf_feat = FeatureNamespace::new(
            "cmf",
            cmf_fb.next(cmf, cmf_trade, &cmf_strategy.config).to_box(),
        );
        combined.push(cmf_feat.to_box());

        let cmo = cmo_indicator.next();
        let cmo_trade = cmo_strategy.next(cmo);
        let cmo_feat = FeatureNamespace::new(
            "cmo",
            cmo_fb.next(cmo, cmo_trade, &cmo_strategy.config).to_box(),
        );
        combined.push(cmo_feat.to_box());

        let ci = ci_indicator.next();
        let ci_feat = FeatureNamespace::new("ci", ci_fb.next(ci).to_box());
        combined.push(ci_feat.to_box());

        let cci = cci_indicator.next();
        let cci_trade = cci_strategy.next(cci);
        let cci_feat = FeatureNamespace::new(
            "cci",
            cci_fb.next(cci, cci_trade, &cci_strategy.config).to_box(),
        );
        combined.push(cci_feat.to_box());

        let connors_rsi = connors_rsi_indicator.next();
        let connors_rsi_trade = connors_rsi_strategy.next(connors_rsi);
        let connors_rsi_feat = FeatureNamespace::new(
            "connors_rsi",
            connors_rsi_fb
                .next(connors_rsi, connors_rsi_trade, &connors_rsi_strategy.config)
                .to_box(),
        );
        combined.push(connors_rsi_feat.to_box());

        let cc = cc_indicator.next();
        let cc_trade = cc_strategy.next(cc);
        let cc_feat =
            FeatureNamespace::new("cc", cc_fb.next(cc, cc_trade, &cc_strategy.config).to_box());
        combined.push(cc_feat.to_box());

        let dmi = dmi_indicator.next();
        let dmi_trade = dmi_strategy.next(&dmi);
        let dmi_feat = FeatureNamespace::new(
            "dmi",
            dmi_fb.next(&dmi, dmi_trade, &dmi_strategy.config).to_box(),
        );
        combined.push(dmi_feat.to_box());

        let po = po_indicator.next();
        let po_trade = po_strategy.next(po);
        let po_feat =
            FeatureNamespace::new("po", po_fb.next(po, po_trade, &po_strategy.config).to_box());
        combined.push(po_feat.to_box());

        let rvgi = rvgi_indicator.next();
        let rvgi_trade = rvgi_strategy.next(&rvgi);
        let rvgi_feat = FeatureNamespace::new("rvgi", rvgi_fb.next(&rvgi, rvgi_trade).to_box());
        combined.push(rvgi_feat.to_box());

        let rvi = rvi_indicator.next();
        let rvi_trade = rvi_strategy.next(rvi);
        let rvi_feat = FeatureNamespace::new(
            "rvi",
            rvi_fb.next(rvi, rvi_trade, &rvi_strategy.config).to_box(),
        );
        combined.push(rvi_feat.to_box());

        let stoch_rsi = stoch_rsi_indicator.next();
        let stoch_rsi_trade = stoch_rsi_strategy.next(&stoch_rsi);
        let stoch_rsi_feat = FeatureNamespace::new(
            "stoch_rsi",
            stoch_rsi_fb
                .next(&stoch_rsi, stoch_rsi_trade, &stoch_rsi_strategy.config)
                .to_box(),
        );
        combined.push(stoch_rsi_feat.to_box());

        let uo = uo_indicator.next();
        let uo_feat = FeatureNamespace::new("uo", uo_fb.next(uo).to_box());
        combined.push(uo_feat.to_box());

        let vo = vo_indicator.next();
        let vo_trade = vo_strategy.next(vo);
        let vo_feat = FeatureNamespace::new("vo", vo_fb.next(vo, vo_trade).to_box());
        combined.push(vo_feat.to_box());

        let vortex = vortex_indicator.next();
        let vortex_trade = vortex_strategy.next(&vortex);
        let vortex_feat =
            FeatureNamespace::new("vortex", vortex_fb.next(&vortex, vortex_trade).to_box());
        combined.push(vortex_feat.to_box());

        let wpr = wpr_indicator.next();
        let wpr_trade = wpr_strategy.next(wpr);
        let wpr_feat = FeatureNamespace::new("wpr", wpr_fb.next(wpr, wpr_trade).to_box());
        combined.push(wpr_feat.to_box());

        combined.push(asset_fb.next().to_box());
        composer.push(combined.to_box());
    }

    let mut df = composer.to_df_strip(&[&"_raw"]);
    save_df(&mut df, out_path);
}

pub fn generate_ml_dataset_from_ohlcv(
    asset_name: &str,
    timeframe: Timeframe,
    ohlcv_path: &Path,
    outpath: &Path,
) {
    let df = read_df(ohlcv_path);
    let ctx = ComponentContext::build_from_df(&df, asset_name, timeframe);
    generate_ml_dataset_from_ctx(ctx, outpath);
}

pub fn generate_ml_datasets() {
    let paths = fs::read_dir("C:\\projects\\trw-masterclass\\dupa\\.input\\ml\\crypto")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    // let paths = paths.iter().take(10).collect::<Vec<_>>();

    for df_path in tqdm!(paths.iter()) {
        let df_path = df_path;

        let asset_name = get_filename(&df_path).unwrap();
        let time_frame = Timeframe::FourHours;
        let out_df_path = Path::new("C:\\projects\\trw-masterclass\\dupa\\.out\\ml")
            .join(Path::new(asset_name).with_extension("parquet"));

        generate_ml_dataset_from_ohlcv(asset_name, time_frame, df_path, &out_df_path);
    }

    println!("[process] exit");
}
