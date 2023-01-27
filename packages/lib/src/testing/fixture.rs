use std::path::Path;

use polars::prelude::DataFrame;

use crate::{
    asset::timeframe::Timeframe,
    components::component_context::ComponentContext,
    data::{csv::read_csv, polars::SeriesCastUtils},
    strategy::{action::StrategyActionKind, polars::SeriesCastUtilsForStrategy},
};

pub struct Fixture {}

impl Fixture {
    pub fn raw(path: &str) -> (DataFrame, ComponentContext) {
        let mut path = Path::new(file!())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(path);

        let test_mode = std::env::var("NEXTEST").is_ok();

        if (test_mode) {
            path = Path::new("../..").join(path);
        }

        let df = read_csv(&path);
        let ctx = ComponentContext::build_from_df(&df, "TEST", Timeframe::OneDay);
        return (df, ctx);
    }

    pub fn load(path: &str) -> (DataFrame, ComponentContext, Vec<Option<f64>>) {
        let (df, ctx) = Self::raw(path);
        let values = df.column("_target_").unwrap().to_f64();
        return (df, ctx, values);
    }

    pub fn strategy(path: &str) -> (DataFrame, ComponentContext, Vec<Option<StrategyActionKind>>) {
        let (df, ctx) = Self::raw(path);
        let values = df.column("_target_").unwrap().to_strategy_action();
        return (df, ctx, values);
    }
}
