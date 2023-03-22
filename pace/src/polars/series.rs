use std::{ffi::OsStr, path::Path, time::Duration};

use polars::{
    prelude::{
        CsvReader, CsvWriter, DataFrame, DataType, IsFloat, ParquetReader, ParquetWriter,
        SerReader, SerWriter, TimeUnit,
    },
    series::Series,
};

use crate::strategy::trade::{trade_direction_from_f64, TradeDirection};

pub trait SeriesCastUtils {
    fn to_f64(&self) -> Vec<Option<f64>>;
    fn to_i32(&self) -> Vec<Option<i32>>;
    fn to_usize(&self) -> Vec<Option<usize>>;
    fn to_duration(&self) -> Vec<Option<Duration>>;
    fn to_trade_dir(&self) -> Vec<Option<TradeDirection>>;
}

impl SeriesCastUtils for Series {
    fn to_f64(&self) -> Vec<Option<f64>> {
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
                    val
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

    fn to_trade_dir(&self) -> Vec<Option<TradeDirection>> {
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
                    trade_direction_from_f64(val)
                }
            })
            .collect::<Vec<_>>();
    }
}
