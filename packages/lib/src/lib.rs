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

use pyo3::prelude::*;

#[pymodule]
fn pace(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // m.add_class::<PyAssetDataProviderManager>()?;
    // m.add_function(wrap_pyfunction!(chuj, m)?)?;

    Ok(())
}
