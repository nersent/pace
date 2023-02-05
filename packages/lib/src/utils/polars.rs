use std::{path::Path, time::Duration};

use polars::{
    prelude::{CsvReader, DataFrame, DataType, IsFloat, ParquetReader, SerReader, TimeUnit},
    series::Series,
};

pub trait SeriesCastUtils {
    fn to_f64(&self) -> Vec<Option<f64>>;
    fn to_i32(&self) -> Vec<Option<i32>>;
    fn to_duration(&self) -> Vec<Option<Duration>>;
}

impl SeriesCastUtils for Series {
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
