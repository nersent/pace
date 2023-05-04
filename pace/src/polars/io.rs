use std::path::Path;

use polars::prelude::{
    CsvReader, CsvWriter, DataFrame, ParquetReader, ParquetWriter, SerReader, SerWriter,
};

use crate::utils::fs::{ensure_dir, get_filename_extension};

pub fn read_df_csv(path: &Path) -> DataFrame {
    let mut file = std::fs::File::open(path).unwrap();
    let df = CsvReader::new(&mut file)
        .infer_schema(None)
        .finish()
        .unwrap();
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
        None => panic!("Unsupported file type for path {:?}", path.display()),
    }
}

pub fn save_df(df: &mut DataFrame, path: &Path) {
    let extension = get_filename_extension(path);

    match extension {
        Some("parquet") => save_df_parquet(df, path),
        Some("csv") => save_df_csv(df, path),
        Some(&_) => panic!("Unsupported file type"),
        None => panic!("Unsupported file type for path {:?}", path.display()),
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
