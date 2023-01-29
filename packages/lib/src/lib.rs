use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::asset::asset_data_provider::AssetDataProvider;
use crate::asset::asset_data_provider_manager::AssetDataProviderManager;
use itertools::Itertools;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString, PyTuple, PyType};
mod asset;
mod components;
mod data;
mod features;
mod math;
mod ml;
mod strategy;
mod ta;
mod testing;
mod utils;
use crate::asset::in_memory_asset_data_provider::InMemoryAssetDataProvider;
use crate::components::execution_context::ExecutionContext;
use crate::data::csv::read_csv;
use crate::{
    asset::timeframe::Timeframe, components::component_context::ComponentContext,
    strategy::action::StrategyActionKind, testing::fixture::Fixture,
};
use components::source::{Source, SourceKind};
use pyo3::types::IntoPyDict;
use strategy::{
    metrics::{
        strategy_equity_metric::{StrategyEquityMetric, StrategyEquityMetricConfig},
        strategy_omega_ratio_metric::{StrategyOmegaRatioMetric, StrategyOmegaRatioMetricConfig},
        strategy_sharpe_ratio_metric::{
            StrategySharpeRatioMetric, StrategySharpeRatioMetricConfig,
        },
        strategy_total_closed_trades_metric::StrategyTotalClosedTradesMetric,
    },
    strategy_context::{StrategyContext, StrategyContextConfig},
};
use ta::relative_strength_index::{
    rsi_indicator::{
        RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        RelativeStrengthIndexIndicatorResult,
    },
    rsi_strategy::{
        RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
        RSI_STRATEGY_THRESHOLD_OVERBOUGHT, RSI_STRATEGY_THRESHOLD_OVERSOLD,
    },
};
#[pyclass(name = "AssetDataProviderManager")]
pub struct PyAssetDataProviderManager {
    // xd: Box<dyn Dupsko + Sync + Send>,
    manager: AssetDataProviderManager,
}

#[pyclass(name = "StrategyResult")]
#[derive(Debug)]
pub struct PyStrategyResult {
    #[pyo3(get)]
    sharpe_ratio: f64,
    #[pyo3(get)]
    omega_ratio: f64,
    #[pyo3(get)]
    total_closed_trades: usize,
    #[pyo3(get)]
    equity: f64,
    #[pyo3(get)]
    elapsed: u128,
    #[pyo3(get)]
    equity_history: Option<Vec<f64>>,
    #[pyo3(get)]
    time_history: Option<Vec<u64>>,
    #[pyo3(get)]
    returns_history: Option<Vec<f64>>,
    #[pyo3(get)]
    fill_size_history: Option<Vec<Option<f64>>>,
}

#[derive(Debug, PartialEq, Clone)]
#[pyclass(name = "StrategyConfig")]
pub struct PyStrategyConfig {
    rsi_length: usize,
}

#[pymethods]
impl PyAssetDataProviderManager {
    #[new]
    fn new() -> Self {
        return PyAssetDataProviderManager {
            manager: AssetDataProviderManager::new(),
        };
    }

    #[pyo3(signature = (path))]
    fn load(&mut self, py: Python<'_>, path: String) -> PyResult<String> {
        let path = PathBuf::from(path);
        let df = read_csv(&path);
        let asset_data_provider = InMemoryAssetDataProvider::from_df(
            &df,
            &"BTC_USD",
            asset::timeframe::Timeframe::OneDay,
        );
        let id = "BTC_USD";
        self.manager.add(&id, Arc::from(asset_data_provider));
        return Ok(id.to_string());
    }

    #[pyo3(signature = (id, config, with_history))]
    fn example_strategy(
        &self,
        py: Python<'_>,
        id: String,
        config: &PyDict,
        with_history: bool,
    ) -> PyResult<PyStrategyResult> {
        let asset_data_provider = self.manager.get(&id);
        let ctx = ComponentContext::build(ExecutionContext::from_asset(asset_data_provider));
        let rsi_length = config.get_item("rsi_length").unwrap().extract::<usize>()?;

        let mut rsi_strategy = RelativeStrengthIndexStrategy::new(
            ctx.clone(),
            RelativeStrengthIndexStrategyConfig {
                threshold_oversold: RSI_STRATEGY_THRESHOLD_OVERSOLD,
                threshold_overbought: RSI_STRATEGY_THRESHOLD_OVERBOUGHT,
            },
            RelativeStrengthIndexIndicator::new(
                ctx.clone(),
                RelativeStrengthIndexIndicatorConfig {
                    length: rsi_length,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
        );

        let mut strategy = StrategyContext::new(
            ctx.clone(),
            StrategyContextConfig {
                on_bar_close: false,
            },
        );

        let mut equity = StrategyEquityMetric::new(
            ctx.clone(),
            StrategyEquityMetricConfig {
                initial_capital: 1000.0,
            },
        );
        let mut sharpe_ratio = StrategySharpeRatioMetric::new(
            ctx.clone(),
            StrategySharpeRatioMetricConfig {
                risk_free_rate: 0.0,
            },
        );
        let mut omega_ratio = StrategyOmegaRatioMetric::new(
            ctx.clone(),
            StrategyOmegaRatioMetricConfig {
                risk_free_rate: 0.0,
            },
        );
        let mut total_closed_trades = StrategyTotalClosedTradesMetric::new(ctx.clone());
        let start_time = Instant::now();

        let mut res = PyStrategyResult {
            sharpe_ratio: 0.0,
            omega_ratio: 0.0,
            total_closed_trades: 0,
            equity: 0.0,
            elapsed: 0,
            equity_history: if with_history { Some(Vec::new()) } else { None },
            time_history: if with_history { Some(Vec::new()) } else { None },
            returns_history: if with_history { Some(Vec::new()) } else { None },
            fill_size_history: if with_history { Some(Vec::new()) } else { None },
        };
        let annualized = f64::sqrt(365.0);
        let mut xd = 0.0;

        for cctx in ctx {
            let ctx = cctx.get();
            let tick = ctx.tick();
            let mut action: StrategyActionKind = StrategyActionKind::None;

            let (rsi_action, _) = rsi_strategy.next();

            if tick > 1930 {
                action = rsi_action;
                let current_trade = strategy.next(action);
                let _equity = equity.next(current_trade);

                res.sharpe_ratio = sharpe_ratio.next(_equity) * annualized;
                res.omega_ratio = omega_ratio.next(_equity) * annualized;
                res.total_closed_trades = total_closed_trades.next(current_trade);
                res.equity = _equity.equity;

                if with_history {
                    let time = ctx.time();
                    res.equity_history.as_mut().unwrap().push(_equity.equity);
                    res.time_history
                        .as_mut()
                        .unwrap()
                        .push(time.unwrap().as_secs());
                    res.returns_history.as_mut().unwrap().push(_equity.returns);
                    res.fill_size_history
                        .as_mut()
                        .unwrap()
                        .push(equity.trade_fill_size);
                }

                // if equity.returns > xd {
                //     xd = equity.returns;
                // }
            }
        }

        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        let elapsed_time = elapsed_time.as_micros();
        res.elapsed = elapsed_time;
        // if with_history {
        //     println!("xddddddddddd: {}", xd);
        // }
        return Ok(res);

        // let gil = Python::acquire_gil();
        // let py = gil.python();
        // let dict = PyDict::new(py);
        // dict.set_item("sharpe_ratio", res.sharpe_ratio)?;
        // dict.set_item("omega_ratio", res.omega_ratio)?;
        // dict.set_item("total_closed_trades", res.total_closed_trades)?;
        // dict.set_item("elapsed", res.elapsed)?;
        // return Ok(dict);

        // return Ok(res);
    }
}

#[pymodule]
fn dupa(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyAssetDataProviderManager>()?;
    // m.add_function(wrap_pyfunction!(chuj, m)?)?;

    Ok(())
}
