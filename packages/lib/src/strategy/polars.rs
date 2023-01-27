use std::{path::Path, time::Duration};

use polars::{
    prelude::{CsvReader, DataFrame, DataType, IsFloat, ParquetReader, SerReader, TimeUnit},
    series::Series,
};

use crate::data::polars::SeriesCastUtils;

use super::action::StrategyActionKind;

pub trait SeriesCastUtilsForStrategy {
    fn to_strategy_action(&self) -> Vec<Option<StrategyActionKind>>;
}

impl SeriesCastUtilsForStrategy for Series {
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
}
