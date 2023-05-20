// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate num;
#[macro_use]
extern crate num_derive;

use py_backtesting::{PyBacktestBarInfo, PyBacktestResult};
use py_strategy::PyStrategySignal;
use pyo3::{prelude::*, types::PyDict};

pub mod py_backtesting;
pub mod py_data_provider;
pub mod py_strategy;
pub mod pyo3_utils;

use crate::py_backtesting::run_backtest;
use crate::py_data_provider::PyDataProvider;

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
