use std::{path::Path, sync::Arc, time::Duration};

use nersent_pace::{
    core::{
        data_provider::{AnyDataProvider, DataProvider, SymInfo},
        in_memory_data_provider::InMemoryDataProvider,
    },
    polars::io::read_df,
};
use pyo3::{prelude::*, types::PyDict};

use crate::pyo3_utils::PyAnyCast;

#[pyclass(name = "DataProvider")]
pub struct PyDataProvider {
    pub instance: AnyDataProvider,
}

impl PyDataProvider {
    pub fn new(instance: AnyDataProvider) -> Self {
        return Self { instance };
    }

    pub fn get(&self) -> AnyDataProvider {
        return Arc::clone(&self.instance);
    }

    pub fn clone(&self) -> PyDataProvider {
        return PyDataProvider {
            instance: Arc::clone(&self.instance),
        };
    }

    fn get_tick_iterator(&self) -> impl Iterator<Item = usize> {
        return (self.instance.get_first_tick()..=self.instance.get_last_tick()).into_iter();
    }
}

#[pymethods]
impl PyDataProvider {
    #[new]
    pub fn py_new(config: &PyDict) -> Self {
        if config.get_item("path").is_some() {
            let path = config.get_item("path").unwrap().to_str();
            let path = Path::new(&path);

            let df = read_df(&path);
            let instance = InMemoryDataProvider::from_df(&df).to_arc();

            return Self { instance };
        }

        let mut sym_info = SymInfo::default();

        if config.get_item("sym_info").is_some() {
            let sym_info_config = config.get_item("sym_info").unwrap();

            if sym_info_config.get_item("min_qty").is_ok() {
                sym_info.min_qty = sym_info_config.get_item("min_qty").unwrap().to_f64();
            }

            if sym_info_config.get_item("min_tick").is_ok() {
                sym_info.min_tick = sym_info_config.get_item("min_tick").unwrap().to_f64();
            }
        }

        let time = config.get_item("time").unwrap().to_vec_f64();
        let open = config.get_item("open").unwrap().to_vec_f64();
        let high = config.get_item("high").unwrap().to_vec_f64();
        let low = config.get_item("low").unwrap().to_vec_f64();
        let close = config.get_item("close").unwrap().to_vec_f64();
        let volume = config.get_item("volume").unwrap().to_vec_f64();
        // let xd: Result<Vec<Option<Duration>>, _> = config.get_item("time").unwrap().to_vec_f64();

        let time: Vec<Option<Duration>> = time
            .iter()
            .map(|&x| Some(Duration::from_secs(x as u64)))
            .collect();

        let instance = InMemoryDataProvider::new(open, high, low, close, volume, time)
            .with_sym_info(sym_info)
            .to_arc();

        return Self { instance };
    }

    pub fn get_first_tick(&self) -> usize {
        return self.instance.get_first_tick();
    }

    pub fn get_last_tick(&self) -> usize {
        return self.instance.get_last_tick();
    }

    pub fn find_tick(&self, seconds: u64) -> Option<usize> {
        return self.instance.find_tick(seconds);
    }

    pub fn get_open_series(&self) -> Vec<f64> {
        return self
            .get_tick_iterator()
            .map(|i| self.instance.get_open(i))
            .collect();
    }

    pub fn get_high_series(&self) -> Vec<f64> {
        return self
            .get_tick_iterator()
            .map(|i| self.instance.get_high(i))
            .collect();
    }

    pub fn get_low_series(&self) -> Vec<f64> {
        return self
            .get_tick_iterator()
            .map(|i| self.instance.get_low(i))
            .collect();
    }

    pub fn get_close_series(&self) -> Vec<f64> {
        return self
            .get_tick_iterator()
            .map(|i| self.instance.get_close(i))
            .collect();
    }

    pub fn get_volume_series(&self) -> Vec<f64> {
        return self
            .get_tick_iterator()
            .map(|i| self.instance.get_volume(i))
            .collect();
    }

    pub fn get_time_series(&self) -> Vec<f64> {
        return self
            .get_tick_iterator()
            .map(|i| self.instance.get_time(i).unwrap().as_secs_f64())
            .collect();
    }

    pub fn get_tick_series(&self) -> Vec<usize> {
        return self.get_tick_iterator().collect();
    }
}
