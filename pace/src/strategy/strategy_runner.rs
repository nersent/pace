use std::{any::Any, collections::HashMap, sync::Arc};

use kdam::{tqdm, BarExt};

use crate::core::{
    asset::{Asset, AssetRegistry},
    context::Context,
    data_provider::{self, AnyDataProvider},
    incremental::{Incremental, RunPeriod},
};

use super::{
    metrics::tradingview_metrics::{TradingViewMetrics, TradingViewMetricsData},
    strategy::Strategy,
    trade::StrategySignal,
};

pub trait StrategyRunnerTarget {
    fn next(&mut self, bar_index: usize);
    fn on_start(&mut self) {}
    fn on_finish(&mut self) {}
    fn as_any(&self) -> &dyn Any;
}

pub struct StrategyRunnerResultItem {
    pub target: Box<dyn StrategyRunnerTarget>,
    pub asset_id: String,
    pub run_period: RunPeriod,
}

pub struct StrategyRunnerTargetFactoryConfig<'a> {
    pub asset: &'a Asset,
    pub data_provider: AnyDataProvider,
    pub run_period: &'a RunPeriod,
}

pub struct StrategyRunnerItem {
    pub assets: Vec<String>,
    pub periods: Option<Vec<RunPeriod>>,
    pub target_fc: Box<dyn Fn(StrategyRunnerTargetFactoryConfig) -> Box<dyn StrategyRunnerTarget>>,
}

pub struct StrategyRunner {}

impl StrategyRunner {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn run(
        &self,
        asset_registry: &AssetRegistry,
        items: Vec<StrategyRunnerItem>,
    ) -> Vec<StrategyRunnerResultItem> {
        let mut total: usize = 0;

        for item in &items {
            total += item.assets.len() + item.periods.as_ref().map(|r| r.len()).unwrap_or(1);
        }

        let mut pb = tqdm!(total = total);

        let mut res: Vec<StrategyRunnerResultItem> = vec![];

        let mut _run =
            |item: &StrategyRunnerItem, asset_id: &str, run_period: Option<RunPeriod>| {
                let asset = asset_registry.map.get(asset_id).unwrap();
                let data_provider = Arc::clone(asset.data_provider.as_ref().unwrap());

                let first_tick = data_provider.get_first_tick();
                let last_tick = data_provider.get_last_tick();

                let run_period = if run_period.is_some() {
                    run_period.unwrap()
                } else {
                    (first_tick, last_tick)
                };

                let fc_config = StrategyRunnerTargetFactoryConfig {
                    asset: &asset.asset,
                    data_provider,
                    run_period: &run_period,
                };

                let mut target = (item.target_fc)(fc_config);

                target.on_start();

                for i in first_tick..=last_tick {
                    target.next(i);
                }

                target.on_finish();

                res.push(StrategyRunnerResultItem {
                    target,
                    asset_id: asset_id.to_string(),
                    run_period: run_period.clone(),
                });

                pb.update(1);
            };

        for item in &items {
            for asset_id in &item.assets {
                if let Some(periods) = &item.periods {
                    for run_period in periods {
                        _run(item, asset_id, Some(*run_period));
                    }
                } else {
                    _run(item, asset_id, None);
                }
            }
        }

        pb.clear();

        return res;
    }
}
