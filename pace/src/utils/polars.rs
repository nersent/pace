use std::{ffi::OsStr, path::Path, time::Duration};

use polars::{
    prelude::{
        CsvReader, CsvWriter, DataFrame, DataType, IsFloat, ParquetReader, ParquetWriter,
        SerReader, SerWriter, TimeUnit,
    },
    series::Series,
};

use crate::strategy::trade::{trade_direction_from_f64, TradeDirection};

use super::fs::{ensure_dir, get_filename_extension};

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

pub trait DataFrameUtils {
    fn merge_two_columns(&self, col1: &str, col2: &str) -> Vec<Option<(Option<f64>, Option<f64>)>>;
    fn merge_three_columns(
        &self,
        col1: &str,
        col2: &str,
        col3: &str,
    ) -> Vec<Option<(Option<f64>, Option<f64>, Option<f64>)>>;
    fn merge_four_columns(
        &self,
        col1: &str,
        col2: &str,
        col3: &str,
        col4: &str,
    ) -> Vec<Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>>;
}

impl DataFrameUtils for DataFrame {
    fn merge_two_columns(
        &self,
        first: &str,
        second: &str,
    ) -> Vec<Option<(Option<f64>, Option<f64>)>> {
        let first_values = self.column(first).unwrap().to_f64();
        let second_values = self.column(second).unwrap().to_f64();
        let arr: Vec<Option<(Option<f64>, Option<f64>)>> = first_values
            .iter()
            .zip(second_values.iter())
            .map(|(first, second)| Some((*first, *second)))
            .collect();
        return arr;
    }

    fn merge_three_columns(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> Vec<Option<(Option<f64>, Option<f64>, Option<f64>)>> {
        let first_values = self.column(first).unwrap().to_f64();
        let second_values = self.column(second).unwrap().to_f64();
        let third_values = self.column(third).unwrap().to_f64();
        let arr: Vec<Option<(Option<f64>, Option<f64>, Option<f64>)>> = first_values
            .iter()
            .zip(second_values.iter())
            .zip(third_values.iter())
            .map(|((first, second), third)| Some((*first, *second, *third)))
            .collect();
        return arr;
    }

    fn merge_four_columns(
        &self,
        first: &str,
        second: &str,
        third: &str,
        fourth: &str,
    ) -> Vec<Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>> {
        let first_values = self.column(first).unwrap().to_f64();
        let second_values = self.column(second).unwrap().to_f64();
        let third_values = self.column(third).unwrap().to_f64();
        let fourth_values = self.column(fourth).unwrap().to_f64();
        let arr: Vec<Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>> = first_values
            .iter()
            .zip(second_values.iter())
            .zip(third_values.iter())
            .zip(fourth_values.iter())
            .map(|(((first, second), third), fourth)| Some((*first, *second, *third, *fourth)))
            .collect();
        return arr;
    }
}

pub fn read_df_csv(path: &Path) -> DataFrame {
    let mut file = std::fs::File::open(path).unwrap();
    let df = CsvReader::new(&mut file).finish().unwrap();
    return df;
}

pub fn read_df_parquet(path: &Path) -> DataFrame {
    let mut file = std::fs::File::open(path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();
    return df;
}

pub fn read_df(path: &Path) -> DataFrame {
    let extension = get_filename_extension(path);
    match extension {
        Some("parquet") => read_df_parquet(path),
        Some("csv") => read_df_csv(path),
        Some(&_) => panic!("Unsupported file type"),
        None => panic!("Unsupported file type"),
    }
}

pub fn save_df(df: &mut DataFrame, path: &Path) {
    let extension = get_filename_extension(path);

    match extension {
        Some("parquet") => save_df_parquet(df, path),
        Some("csv") => save_df_csv(df, path),
        Some(&_) => panic!("Unsupported file type"),
        None => panic!("Unsupported file type"),
    };
}

pub fn save_df_csv(df: &mut DataFrame, path: &Path) {
    ensure_dir(path.parent().unwrap());
    let mut file = std::fs::File::create(path).unwrap();
    CsvWriter::new(&mut file).finish(df).unwrap();
}

pub fn save_df_parquet(df: &mut DataFrame, path: &Path) {
    ensure_dir(path.parent().unwrap());
    let mut file = std::fs::File::create(path).unwrap();
    ParquetWriter::new(&mut file).finish(df).unwrap();
}
