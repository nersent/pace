use std::{ffi::OsStr, path::Path, time::Duration};

use polars::{
    prelude::{
        CsvReader, CsvWriter, DataFrame, DataType, IsFloat, ParquetReader, ParquetWriter,
        SerReader, SerWriter, TimeUnit,
    },
    series::Series,
};

use crate::{
    core::trend::Trend,
    strategy::trade::{trade_direction_from_f64, StrategySignal, TradeDirection},
    utils::float::OptionFloatUtils,
};

pub trait SeriesCastUtils {
    fn to_bool(&self) -> Vec<Option<bool>>;
    fn to_f64(&self) -> Vec<f64>;
    fn to_i32(&self) -> Vec<Option<i32>>;
    fn to_usize(&self) -> Vec<Option<usize>>;
    fn to_duration(&self) -> Vec<Option<Duration>>;
    fn to_signal(&self) -> Vec<StrategySignal>;
    fn to_trend(&self) -> Vec<Trend>;
}

impl SeriesCastUtils for Series {
    fn to_bool(&self) -> Vec<Option<bool>> {
        return self
            .cast(&DataType::Boolean)
            .unwrap()
            .bool()
            .unwrap()
            .into_iter()
            .map(|val| {
                if val.is_none() || val.unwrap().is_nan() {
                    None
                } else {
                    val
                }
            })
            .collect::<Vec<_>>();
    }

    fn to_f64(&self) -> Vec<f64> {
        return self
            .cast(&DataType::Float64)
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|val| {
                if val.is_none() || val.unwrap().is_nan() {
                    f64::NAN
                } else {
                    val.unwrap()
                }
            })
            .collect::<Vec<_>>();
    }

    fn to_i32(&self) -> Vec<Option<i32>> {
        return self
            .cast(&DataType::Int32)
            .unwrap()
            .i32()
            .unwrap()
            .into_iter()
            .map(|val| {
                if val.is_none() || val.unwrap().is_nan() {
                    None
                } else {
                    val
                }
            })
            .collect::<Vec<_>>();
    }

    fn to_usize(&self) -> Vec<Option<usize>> {
        return self
            .cast(&DataType::UInt64)
            .unwrap()
            .u64()
            .unwrap()
            .into_iter()
            .map(|val| {
                if val.is_none() || val.unwrap().is_nan() {
                    None
                } else {
                    val.map(|x| x as usize)
                }
            })
            .collect::<Vec<_>>();
    }

    fn to_duration(&self) -> Vec<Option<Duration>> {
        return self
            .cast(&DataType::Float64)
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|val| {
                if val.is_none() || val.unwrap().is_nan() {
                    None
                } else {
                    Some(Duration::from_secs_f64(val.unwrap()))
                }
            })
            .collect::<Vec<_>>();
    }

    fn to_signal(&self) -> Vec<StrategySignal> {
        return self
            .cast(&DataType::Float64)
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|val| {
                let val = val.unwrap_nan();
                if val.is_nan() {
                    return StrategySignal::Hold;
                }
                return StrategySignal::from(val);
            })
            .collect::<Vec<_>>();
    }

    fn to_trend(&self) -> Vec<Trend> {
        return self
            .cast(&DataType::Float64)
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|val| {
                let val = val.unwrap_nan();
                return Trend::from(val);
            })
            .collect::<Vec<_>>();
    }
}
