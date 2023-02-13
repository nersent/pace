use pyo3::{prelude::*, types::PyDict};

use crate::{
    base::{
        components::{component_context::ComponentContext, component_default::ComponentDefault},
        strategy::{
            metrics::{
                omega_ratio_metric::{OmegaRatioMetric, OmegaRatioMetricConfig},
                sharpe_ratio_metric::{SharpeRatioMetric, SharpeRatioMetricConfig},
            },
            strategy_context::{StrategyContext, StrategyContextConfig},
            strategy_runner::{
                StrategyRunner, StrategyRunnerConfig, StrategyRunnerMetricsConfig,
                StrategyRunnerResult,
            },
            trade::TradeDirection,
        },
    },
    content::{
        relative_strength_index_indicator::{
            RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        },
        relative_strength_index_strategy::{
            RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
        },
    },
};

use super::{
    cast::{FromPyRef, PyAnyCast, ToPyDict},
    py_asset_data_provider::PyAssetDataProvider,
};

fn create_strategy_runner(ctx: ComponentContext, config: &PyDict) -> StrategyRunner {
    let strategy_ctx_config = StrategyContextConfig {
        on_bar_close: config
            .get_item("on_bar_close")
            .map(|x| x.to_bool())
            .unwrap_or(false),
        continous: config
            .get_item("continous")
            .map(|x| x.to_bool())
            .unwrap_or(true),
        buy_with_equity: config
            .get_item("buy_with_equity")
            .map(|x| x.to_bool())
            .unwrap_or(false),
        initial_capital: config
            .get_item("initial_capital")
            .map(|x| x.to_f64())
            .unwrap_or(1000.0),
    };

    let metrics = config.get_item("metrics").unwrap();

    let runner_config = StrategyRunnerConfig {
        start_tick: config.get_item("start_tick").map(|x| x.to_usize()),
        end_tick: config.get_item("end_tick").map(|x| x.to_usize()),
        print: config
            .get_item("print")
            .map(|x| x.to_bool())
            .unwrap_or(false),
        metrics: StrategyRunnerMetricsConfig {
            omega_ratio: metrics
                .get_item("omega_ratio")
                .map(|x| {
                    Some(OmegaRatioMetric::new(
                        ctx.clone(),
                        OmegaRatioMetricConfig {
                            risk_free_rate: x.get_item("risk_free_rate").unwrap().to_f64(),
                        },
                    ))
                })
                .unwrap_or(None),
            sharpe_ratio: metrics
                .get_item("sharpe_ratio")
                .map(|x| {
                    Some(SharpeRatioMetric::new(
                        ctx.clone(),
                        SharpeRatioMetricConfig {
                            risk_free_rate: x.get_item("risk_free_rate").unwrap().to_f64(),
                        },
                    ))
                })
                .unwrap_or(None),
            track: metrics
                .get_item("track")
                .map(|x| x.to_bool())
                .unwrap_or(false),
        },
    };

    return StrategyRunner::new(
        ctx.clone(),
        StrategyContext::new(ctx.clone(), strategy_ctx_config),
        runner_config,
    );
}

#[pyfunction]
pub fn run_relative_strength_index(
    asset_data_provider: PyRef<'_, PyAssetDataProvider>,
    runner_config: &PyDict,
    config: &PyDict,
) -> StrategyRunnerResult {
    let ctx = ComponentContext::from_py_ref(asset_data_provider);

    let mut runner = create_strategy_runner(ctx.clone(), runner_config);

    let indicator_config = config.get_item("indicator").unwrap();
    let strategy_config = config.get_item("strategy").unwrap();

    let i_config = RelativeStrengthIndexIndicatorConfig {
        length: indicator_config.get_item("length").unwrap().to_usize(),
        src: indicator_config
            .get_item("src")
            .unwrap()
            .to_src(ctx.clone()),
    };
    let s_config = RelativeStrengthIndexStrategyConfig {
        threshold_overbought: strategy_config
            .get_item("threshold_overbought")
            .unwrap()
            .to_f64(),
        threshold_oversold: strategy_config
            .get_item("threshold_oversold")
            .unwrap()
            .to_f64(),
    };

    let indicator = &mut RelativeStrengthIndexIndicator::new(ctx.clone(), i_config);
    let strategy = &mut RelativeStrengthIndexStrategy::new(ctx.clone(), s_config);

    let result = runner.run(|| {
        return strategy.next(indicator.next());
    });

    return result;
}
