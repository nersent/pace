use polars::prelude::DataFrame;
use pyo3::types::PyType;
use pyo3::{prelude::*, types::PyDict};
use std::{
    path::{Path, PathBuf},
    time::Instant,
};

use chrono::{DateTime, NaiveDateTime};
use colored::Colorize;
use components::source::{Source, SourceKind};
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

use crate::{
    asset::timeframe::Timeframe, components::component_context::ComponentContext,
    data::csv::read_csv, strategy::action::StrategyActionKind, testing::fixture::Fixture,
};
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
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn xd(a: usize, b: usize) -> PyResult<String> {
    Ok((a * b).to_string())
}

// #[pyfunction]
// fn example_strategy(path: String) -> PyResult<String> {
//     let (df, ctx) = Fixture::raw(&path);

//     let mut rsi_strategy = RelativeStrengthIndexStrategy::new(
//         ctx.clone(),
//         RelativeStrengthIndexStrategyConfig {
//             threshold_oversold: RSI_STRATEGY_THRESHOLD_OVERSOLD,
//             threshold_overbought: RSI_STRATEGY_THRESHOLD_OVERBOUGHT,
//         },
//         RelativeStrengthIndexIndicator::new(
//             ctx.clone(),
//             RelativeStrengthIndexIndicatorConfig {
//                 length: 14,
//                 src: Source::from_kind(ctx.clone(), SourceKind::Close),
//             },
//         ),
//     );

//     let mut strategy = StrategyContext::new(
//         ctx.clone(),
//         StrategyContextConfig {
//             on_bar_close: false,
//         },
//     );

//     let mut equity = StrategyEquityMetric::new(
//         ctx.clone(),
//         StrategyEquityMetricConfig {
//             initial_capital: 1000.0,
//         },
//     );
//     let mut sharpe_ratio = StrategySharpeRatioMetric::new(
//         ctx.clone(),
//         StrategySharpeRatioMetricConfig {
//             risk_free_rate: 0.0,
//         },
//     );
//     let mut omega_ratio = StrategyOmegaRatioMetric::new(
//         ctx.clone(),
//         StrategyOmegaRatioMetricConfig {
//             risk_free_rate: 0.0,
//         },
//     );
//     let mut total_closed_trades = StrategyTotalClosedTradesMetric::new(ctx.clone());
//     let start_time = Instant::now();

//     for cctx in ctx {
//         let ctx = cctx.get();
//         let tick = ctx.tick();
//         let price = ctx.open();
//         let time = ctx.time();
//         let mut action: StrategyActionKind = StrategyActionKind::None;

//         let long_ticks = [];
//         let short_ticks = [];

//         if long_ticks.contains(&tick) {
//             action = StrategyActionKind::Long;
//         } else if short_ticks.contains(&tick) {
//             action = StrategyActionKind::Short;
//         }

//         let (rsi_action, _) = rsi_strategy.next();
//         action = rsi_action;

//         // if (current_tick == 4 || current_tick == 7) {
//         //     action = StrategyActionKind::Long;
//         // } else if (current_tick == 10 || current_tick == 14) {
//         //     action = StrategyActionKind::Short;
//         // }

//         let current_trade = strategy.next(action);
//         let equity = equity.next(current_trade);
//         let sharpe_ratio = sharpe_ratio.next(equity) * f64::sqrt(365.0);
//         let omega_ratio = omega_ratio.next(equity) * f64::sqrt(365.0);
//         let total_closed_trades = total_closed_trades.next(current_trade);

//         // println!(
//         //     "\n{}: {}{} | {}\n{}\n{}\n{}\n{}",
//         //     format!("[{}]", tick).bright_cyan().bold(),
//         //     format!("{:?}", price.unwrap_or(0.0)).blue(),
//         //     if current_trade.is_none() || current_trade.unwrap().entry_price.is_none() {
//         //         "".to_string()
//         //     } else {
//         //         format!("| {}", current_trade.unwrap().to_colored_string()).to_string()
//         //     },
//         //     format!(
//         //         "{}",
//         //         NaiveDateTime::from_timestamp_millis(time.unwrap().as_millis() as i64)
//         //             .unwrap()
//         //             .format("%d-%m-%Y %H:%M")
//         //     )
//         //     .bright_black(),
//         //     format!(
//         //         "Equity: {:0.2} | Returns: {:0.2} | Mean returns: {:0.2} | Stdev Returns: {:0.2}",
//         //         equity.equity, equity.returns, equity.returns_mean, equity.returns_stdev
//         //     )
//         //     .bright_black(),
//         //     format!("Sharpe: {:0.2}", sharpe_ratio).bright_black(),
//         //     format!("Omega: {:0.2}", omega_ratio).bright_black(),
//         //     format!("Total closed trades: {}", total_closed_trades).bright_black(),
//         //     // current_trade,
//         // );

//         // if (tick > 450) {
//         //     break;
//         // }
//     }

//     let end_time = Instant::now();
//     let elapsed_time = end_time - start_time;
//     let elapsed_time = elapsed_time.as_micros();

//     return Ok(elapsed_time.to_string());
// }

// #[pyfunction]
// fn chuj(path: String) -> PyResult<String> {
//     let path = PathBuf::from(path);

//     Ok(path.display().to_string())
// }

// #[pyfn(m)]
// fn chuj(path: String) -> PyResult<String> {
//     return Ok(path);
//     // let path = PathBuf::from(path);

//     // let csv: DataFrame = read_csv(&path);
//     // let tensor_as_py = Py::new(py, tensor)?.into_ref(py);

//     // return PyObject::into_ptr(csv);
// }
/// A Python module implemented in Rust.
///

// #[pyfunction]
// #[pyo3(signature = (path))]
// fn chuj(path: String) -> PyResult<String> {
//     return Ok(path);
//     // let path = PathBuf::from(path);

//     // let csv: DataFrame = read_csv(&path);
//     // let tensor_as_py = Py::new(py, tensor)?.into_ref(py);

//     // return PyObject::into_ptr(csv);
// }

#[pyfunction]
#[pyo3(signature = (path))]
fn chuj(path: String) -> PyResult<String> {
    // let path = PathBuf::from(path);
    // let csv: DataFrame = read_csv(&path);
    // let pyref = PyRef::new(py, csv)?;

    return Ok(path);
    // let gil = Python::acquire_gil();
    // let py = gil.python();
    // let xd = Py::new(py, csv);

    // return Ok("xd".to_string());
    // let path = PathBuf::from(path);

    // let csv: DataFrame = read_csv(&path);
    // let tensor_as_py = Py::new(py, tensor)?.into_ref(py);

    // return PyObject::into_ptr(csv);
}

#[pymodule]
fn dupa(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(chuj, m)?)?;
    // #[pyfunction(text_signature = "(path, /)")]

    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    // // m.add_function(wrap_pyfunction!(example_strategy, m)?)?;
    // m.add_function(wrap_pyfunction!(xd, m)?)?;

    Ok(())
}
