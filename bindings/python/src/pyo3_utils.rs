use std::sync::Arc;

use nersent_pace::{
    common::src::AnySrc,
    core::context::Context,
    strategy::trade::{StrategySignal, TradeDirection},
};
use pyo3::{types::PyDict, PyAny, PyRef};

// use super::py_pace_utils::build_src;

// pub trait FromPyRef<T: pyo3::PyClass> {
//     fn from_py_ref(dict: PyRef<'_, T>) -> Self;
// }

pub trait PyAnyCast {
    fn to_f64(&self) -> f64;
    fn to_i32(&self) -> i32;
    fn to_bool(&self) -> bool;
    fn to_str(&self) -> String;
    fn to_usize(&self) -> usize;
    fn to_trade_direction(&self) -> TradeDirection;
    fn to_vec_f64(&self) -> Vec<f64>;
    // fn to_src(&self, ctx: Context) -> AnySrc;
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

    fn to_str(&self) -> String {
        return self.extract::<String>().unwrap();
    }

    fn to_usize(&self) -> usize {
        return self.extract::<usize>().unwrap();
    }

    fn to_trade_direction(&self) -> TradeDirection {
        let dir: TradeDirection = num::FromPrimitive::from_i32(self.to_i32()).unwrap();
        return dir;
    }

    fn to_vec_f64(&self) -> Vec<f64> {
        return self.extract::<Vec<f64>>().unwrap();
    }

    // fn to_src(&self, ctx: Context) -> AnySrc {
    //     let kind = self.extract::<String>().unwrap();
    //     return build_src(ctx, &kind);
    // }
}
