use std::{path::Path, time::Duration};

use polars::{
    prelude::{CsvReader, DataFrame, DataType, IsFloat, ParquetReader, SerReader, TimeUnit},
    series::Series,
};

use crate::base::strategy::types::StrategyActionKind;

pub fn read_parquet(path: &Path) -> DataFrame {
    let mut file = std::fs::File::open(path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();
    return df;
}

pub fn read_csv(path: &Path) -> DataFrame {
    let mut file = std::fs::File::open(path).unwrap();
    let df = CsvReader::new(&mut file).finish().unwrap();
    return df;
}

pub trait SeriesUtils {
    fn to_f64(&self) -> Vec<Option<f64>>;
    fn to_i32(&self) -> Vec<Option<i32>>;
    fn to_strategy_action(&self) -> Vec<Option<StrategyActionKind>>;
    fn to_duration(&self) -> Vec<Option<Duration>>;
}

impl SeriesUtils for Series {
    fn to_f64(&self) -> Vec<Option<f64>> {
        return self
            .cast(&DataType::Float64)
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|val| if val.unwrap().is_nan() { None } else { val })
            .collect::<Vec<_>>();
    }

    fn to_i32(&self) -> Vec<Option<i32>> {
        return self
            .cast(&DataType::Int32)
            .unwrap()
            .i32()
            .unwrap()
            .into_iter()
            .map(|val| if val.unwrap().is_nan() { None } else { val })
            .collect::<Vec<_>>();
    }

    fn to_strategy_action(&self) -> Vec<Option<StrategyActionKind>> {
        return self
            .to_f64()
            .into_iter()
            .map(|value| {
                value?;
                let value = value.unwrap();
                if value == 1.0 {
                    return Some(StrategyActionKind::Long);
                }
                if value == -1.0 {
                    return Some(StrategyActionKind::Short);
                }
                return Some(StrategyActionKind::None);
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
                if val.unwrap().is_nan() {
                    None
                } else {
                    Some(Duration::from_secs_f64(val.unwrap()))
                }
            })
            .collect::<Vec<_>>();
    }
}
