use pyo3::{types::PyDict, PyRef};

use super::component_context::ComponentContext;

pub trait ComponentFromPyDict {
    fn from_py_dict(ctx: ComponentContext, dict: &PyDict) -> Self;
}
