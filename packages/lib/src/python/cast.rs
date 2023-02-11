use std::sync::Arc;

use pyo3::{types::PyDict, PyAny, PyRef};

use crate::base::{
    asset::source::{Source, SourceKind},
    components::component_context::ComponentContext,
};

use super::py_asset_data_provider::PyAssetDataProvider;

pub trait FromPyRef<T: pyo3::PyClass> {
    fn from_py_ref(dict: PyRef<'_, T>) -> Self;
}

pub trait PyAnyCast {
    fn to_f64(&self) -> f64;
    fn to_i32(&self) -> i32;
    fn to_bool(&self) -> bool;
    fn to_string(&self) -> String;
    fn to_usize(&self) -> usize;
    fn to_src(&self, ctx: ComponentContext) -> Source;
}

impl PyAnyCast for PyAny {
    fn to_f64(&self) -> f64 {
        return self.extract::<f64>().unwrap();
    }

    fn to_i32(&self) -> i32 {
        return self.extract::<i32>().unwrap();
    }

    fn to_bool(&self) -> bool {
        return self.extract::<bool>().unwrap();
    }

    fn to_string(&self) -> String {
        return self.extract::<String>().unwrap();
    }

    fn to_usize(&self) -> usize {
        return self.extract::<usize>().unwrap();
    }

    fn to_src(&self, ctx: ComponentContext) -> Source {
        let kind = SourceKind::try_from(self.to_usize()).unwrap();
        return Source::from_kind(ctx, kind);
    }
}

impl FromPyRef<PyAssetDataProvider> for ComponentContext {
    fn from_py_ref(asset_data_provider: PyRef<'_, PyAssetDataProvider>) -> Self {
        return Self::from_asset_data_provider(Arc::clone(&asset_data_provider.asset));
    }
}

pub trait ToPyDict {
    fn to_py_dict(self) -> PyDict;
}
