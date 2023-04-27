use std::{collections::HashMap, sync::Arc};

use kdam::{tqdm, BarExt};

use crate::core::{
    asset::Asset, context::Context, data_provider::AnyDataProvider, incremental::Incremental,
};

use super::{
    metrics::tradingview_metrics::{TradingViewMetrics, TradingViewMetricsData},
    strategy::Strategy,
    trade::StrategySignal,
};

pub type StrategyRunPeriod = (usize, usize);

pub struct StrategyRunnerTarget<M> {
    pub id: String,
    pub assets: Vec<Asset>,
    pub options: StrategyRunnerTargetOptions<M>,
}

pub trait StrategyRunnerTargetMetricsProvider<M>: for<'a> Incremental<&'a Strategy, ()> {
    fn get_metrics(&self) -> M;
}

impl StrategyRunnerTargetMetricsProvider<TradingViewMetricsData> for TradingViewMetrics {
    fn get_metrics(&self) -> TradingViewMetricsData {
        return self.data.clone();
    }
}

pub type StrategyRunnerTargetFactory = Box<
    dyn Fn(
        Context,
        AnyDataProvider,
        &Strategy,
        &Asset,
    ) -> Box<dyn for<'a> Incremental<(bool, &'a Strategy), StrategySignal>>,
>;

pub type StrategyRunnerTargetMetricsProviderFactory<M> = Box<
    dyn Fn(
        Context,
        AnyDataProvider,
        &Strategy,
        &Asset,
    ) -> Box<dyn for<'a> StrategyRunnerTargetMetricsProvider<M>>,
>;

pub struct StrategyRunnerTargetOptions<M> {
    pub data_provider: Box<dyn Fn(&Asset) -> AnyDataProvider>,
    pub ctx: Box<dyn Fn(AnyDataProvider, &Asset) -> Context>,
    pub strategy: Box<dyn Fn(Context, AnyDataProvider, &Asset) -> Strategy>,
    pub target: StrategyRunnerTargetFactory,
    pub metrics_provider: Option<StrategyRunnerTargetMetricsProviderFactory<M>>,
    pub periods: Box<dyn Fn(Context, AnyDataProvider, &Asset) -> Vec<StrategyRunPeriod>>,
}

pub struct StrategyRunnerItem<M> {
    pub id: String,
    pub asset: Asset,
    pub strategy: Strategy,
    pub period: StrategyRunPeriod,
    pub target: Box<dyn for<'a> Incremental<(bool, &'a Strategy), StrategySignal>>,
    pub metrics: Option<Box<dyn for<'a> StrategyRunnerTargetMetricsProvider<M>>>,
}

pub struct StrategyRunner {}

impl StrategyRunner {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn run<M>(&self, targets: Vec<StrategyRunnerTarget<M>>) -> Vec<StrategyRunnerItem<M>> {
        let mut data_provider_cache: HashMap<String, AnyDataProvider> = HashMap::new();
        let mut total: usize = 0;

        for target in &targets {
            for asset in target.assets.iter() {
                let data_provider: AnyDataProvider = (target.options.data_provider)(asset);
                data_provider_cache.insert(asset.hash.clone(), data_provider);

                total += 1;
            }
        }

        let mut pb = tqdm!(total = total);

        let mut finished_items: Vec<StrategyRunnerItem<M>> = Vec::new();

        for target in &targets {
            for asset in target.assets.iter() {
                let data_provider = data_provider_cache.get(&asset.hash).unwrap();
                let ctx = (target.options.ctx)(data_provider.clone(), asset);

                let periods =
                    (target.options.periods)(ctx.clone(), Arc::clone(&data_provider), asset);

                for period in &periods {
                    let target_id = target.id.clone();

                    let mut strategy =
                        (target.options.strategy)(ctx.clone(), Arc::clone(&data_provider), asset);

                    let mut metrics = (target.options.metrics_provider).as_ref().map(|f| {
                        let mut metrics_provider =
                            f(ctx.clone(), Arc::clone(&data_provider), &strategy, &asset);
                        return metrics_provider;
                    });
                    let mut target = (target.options.target)(
                        ctx.clone(),
                        Arc::clone(&data_provider),
                        &strategy,
                        asset,
                    );

                    for i in ctx.first_bar_index..=ctx.last_bar_index {
                        ctx.bar.index.set(i);

                        let mut signal: StrategySignal = StrategySignal::Neutral;
                        let in_range = i >= period.0 && i <= period.1;

                        signal = target.next((in_range, &strategy));

                        if in_range {
                            strategy.next(signal);

                            if let Some(metrics) = metrics.as_mut() {
                                metrics.next(&strategy);
                            }
                        }
                    }

                    finished_items.push(StrategyRunnerItem {
                        id: target_id,
                        asset: asset.clone(),
                        strategy,
                        period: period.clone(),
                        target,
                        metrics,
                    });

                    pb.update(1);
                }
            }
        }

        pb.clear();

        return finished_items;
    }
}
