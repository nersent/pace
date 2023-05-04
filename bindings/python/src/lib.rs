#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate num;
#[macro_use]
extern crate num_derive;

use py_backtesting::{PyBacktestBarInfo, PyBacktestResult};
use py_strategy::PyStrategySignal;
use pyo3::{prelude::*, types::PyDict};

mod py_asset_provider;
mod py_backtesting;
mod py_strategy;
mod pyo3_utils;
use crate::py_asset_provider::PyDataProvider;
use crate::py_backtesting::run_backtest;

#[pymodule]
#[pyo3(name = "nersent_pace_py")]
fn nersent_pace_py(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyDataProvider>()?;
    m.add_class::<PyBacktestBarInfo>()?;
    m.add_class::<PyBacktestResult>()?;
    m.add_class::<PyStrategySignal>()?;
    m.add_function(wrap_pyfunction!(run_backtest, m)?)?;

    Ok(())
}
