#![allow(
    clippy::needless_return,
    clippy::type_complexity,
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::uninlined_format_args,
    clippy::module_inception,
    clippy::upper_case_acronyms,
    unused
)]

use pyo3::{prelude::*, types::PyDict};
use python::py_asset_data_provider::PyAssetDataProvider;

use crate::python::py_strategies::run_relative_strength_index;
mod base;
mod content;
mod example_strategy;
mod ml;
mod python;
mod utils;

#[pymodule]
fn pace(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyAssetDataProvider>()?;
    m.add_function(wrap_pyfunction!(run_relative_strength_index, m)?)?;

    Ok(())
}
