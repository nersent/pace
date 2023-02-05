use std::path::Path;

use polars::prelude::{CsvReader, DataFrame, SerReader};

pub fn read_csv(path: &Path) -> DataFrame {
    let mut file = std::fs::File::open(path).unwrap();
    let df = CsvReader::new(&mut file).finish().unwrap();
    return df;
}
