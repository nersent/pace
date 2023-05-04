use std::{ffi::OsStr, path::Path, time::Duration};

use polars::{
    prelude::{
        CsvReader, CsvWriter, DataFrame, DataType, IsFloat, ParquetReader, ParquetWriter,
        SerReader, SerWriter, TimeUnit,
    },
    series::Series,
};

use crate::utils::fs::{ensure_dir, get_filename_extension};

use super::series::SeriesCastUtils;

pub trait DataFrameUtils {
    fn merge_two_columns(&self, col1: &str, col2: &str) -> Vec<(f64, f64)>;
    fn merge_three_columns(&self, col1: &str, col2: &str, col3: &str) -> Vec<(f64, f64, f64)>;
    fn merge_four_columns(
        &self,
        col1: &str,
        col2: &str,
        col3: &str,
        col4: &str,
    ) -> Vec<(f64, f64, f64, f64)>;
}

impl DataFrameUtils for DataFrame {
    fn merge_two_columns(&self, first: &str, second: &str) -> Vec<(f64, f64)> {
        let first_values = self.column(first).unwrap().to_f64();
        let second_values = self.column(second).unwrap().to_f64();
        let arr: Vec<(f64, f64)> = first_values
            .iter()
            .zip(second_values.iter())
            .map(|(first, second)| (*first, *second))
            .collect();
        return arr;
    }

    fn merge_three_columns(&self, first: &str, second: &str, third: &str) -> Vec<(f64, f64, f64)> {
        let first_values = self.column(first).unwrap().to_f64();
        let second_values = self.column(second).unwrap().to_f64();
        let third_values = self.column(third).unwrap().to_f64();
        let arr: Vec<(f64, f64, f64)> = first_values
            .iter()
            .zip(second_values.iter())
            .zip(third_values.iter())
            .map(|((first, second), third)| (*first, *second, *third))
            .collect();
        return arr;
    }

    fn merge_four_columns(
        &self,
        first: &str,
        second: &str,
        third: &str,
        fourth: &str,
    ) -> Vec<(f64, f64, f64, f64)> {
        let first_values = self.column(first).unwrap().to_f64();
        let second_values = self.column(second).unwrap().to_f64();
        let third_values = self.column(third).unwrap().to_f64();
        let fourth_values = self.column(fourth).unwrap().to_f64();
        let arr: Vec<(f64, f64, f64, f64)> = first_values
            .iter()
            .zip(second_values.iter())
            .zip(third_values.iter())
            .zip(fourth_values.iter())
            .map(|(((first, second), third), fourth)| (*first, *second, *third, *fourth))
            .collect();
        return arr;
    }
}
