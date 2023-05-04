use nersent_pace::strategy::trade::StrategySignal;
use pyo3::{prelude::*, types::PyDict};

use crate::pyo3_utils::PyAnyCast;

#[derive(Copy, Clone, Debug)]
#[pyclass(name = "StrategySignal")]
pub struct PyStrategySignal {
    inner: StrategySignal,
}

impl PyStrategySignal {
    pub fn get(&self) -> StrategySignal {
        return self.inner;
    }
}

#[pymethods]
impl PyStrategySignal {
    #[new]
    pub fn new(config: &PyDict) -> Self {
        let id = config.get_item("id").unwrap().to_str().to_lowercase();

        let signal = match id.as_str() {
            "long" => StrategySignal::Long,
            "long_entry" => StrategySignal::LongEntry,
            "long_exit" => StrategySignal::LongExit,
            "short" => StrategySignal::Short,
            "short_entry" => StrategySignal::ShortEntry,
            "short_exit" => StrategySignal::ShortExit,
            "hold" => StrategySignal::Hold,
            _ => panic!("Unknown strategy signal id: {}", id),
        };

        return Self { inner: signal };
    }

    pub fn to_string(&self) -> String {
        return format!("{:?}", self.inner);
    }
}
