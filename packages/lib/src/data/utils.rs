use std::path::Path;

use polars::prelude::ParquetReader;
use polars::prelude::*;

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
}
