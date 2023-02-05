use std::path::Path;

use polars::prelude::{DataFrame, ParquetReader, SerReader};

pub fn read_parquet(path: &Path) -> DataFrame {
    let mut file = std::fs::File::open(path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();
    return df;
}
