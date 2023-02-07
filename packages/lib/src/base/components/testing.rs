use std::path::{Path, PathBuf};

use colored::Colorize;
use polars::prelude::DataFrame;

use crate::{
    base::{
        asset::timeframe::Timeframe,
        strategy::{
            orderbook_i::Order,
            polars::SeriesCastUtilsForStrategy,
            trade::{Trade, TradeDirection},
        },
    },
    utils::{
        comparison::FloatComparison,
        polars::{read_df, SeriesCastUtils},
    },
};

use super::component_context::ComponentContext;

pub struct Fixture {}

impl Fixture {
    pub fn raw(path: &str) -> (DataFrame, ComponentContext) {
        let mut path = Path::new(file!())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(path);

        let test_mode = std::env::var("NEXTEST").is_ok();

        if (test_mode) {
            path = Path::new("../../").join(path);
        }

        let df = read_df(&path);
        let ctx = ComponentContext::build_from_df(&df, "TEST", Timeframe::OneDay);
        return (df, ctx);
    }

    pub fn load_with_target(
        path: &str,
        target: &str,
    ) -> (DataFrame, ComponentContext, Vec<Option<f64>>) {
        let (df, ctx) = Self::raw(path);
        let values = df.column(target).unwrap().to_f64();
        return (df, ctx, values);
    }

    pub fn load(path: &str) -> (DataFrame, ComponentContext, Vec<Option<f64>>) {
        return Self::load_with_target(path, "_target_");
    }

    pub fn strategy(path: &str) -> (DataFrame, ComponentContext, Vec<Option<TradeDirection>>) {
        let (df, ctx) = Self::raw(path);
        let values = df.column("_target_").unwrap().to_trade();
        return (df, ctx, values);
    }
}

pub struct ComponentTestSnapshot<T> {
    pub debug_mode: bool,
    pub print_max_index: Option<usize>,
    pub actual: Vec<Option<T>>,
}

impl<T: std::fmt::Debug> ComponentTestSnapshot<T> {
    pub fn new() -> Self {
        return ComponentTestSnapshot::<T> {
            actual: Vec::new(),
            debug_mode: false,
            print_max_index: None,
        };
    }

    pub fn debug_mode(&mut self) {
        self.debug_mode = true;
    }

    pub fn debug_mode_max(&mut self, max_index: usize) {
        self.print_max_index = Some(max_index);
        self.debug_mode();
    }

    pub fn push(&mut self, value: Option<T>) {
        self.actual.push(value);
    }

    pub fn assert_iter(&self, expected: &[Option<T>], compare: fn(&T, &T) -> bool) {
        assert_eq!(
            self.actual.len(),
            expected.len(),
            "Got different sizes | Actual: {} | Expected: {}",
            format!("{}", self.actual.len()).red(),
            format!("{}", expected.len()).green(),
        );
        for i in 0..self.actual.len() {
            let actual = &self.actual[i];
            let expected = &expected[i];
            let is_equal = match (actual, expected) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => compare(_actual, _expected),
                _ => false,
            };
            if !is_equal {
                println!(
                    "{}: {} | {}\n",
                    format!("[{:?}]", i).red().bold(),
                    format!("{:?}", actual).black().on_bright_red().bold(),
                    format!("{:?}", expected).black().on_green().bold(),
                );
                if !self.debug_mode {
                    panic!("Component assertion failed at index {}", i);
                } else {
                    break;
                }
            }
            if self.debug_mode
                && (self.print_max_index.is_none() || self.print_max_index.unwrap() > i)
            {
                println!(
                    "{}: {}",
                    format!("[{:?}]", i).bright_cyan().bold(),
                    format!("{:?}", actual).white(),
                );
            }
        }
    }
}

impl ComponentTestSnapshot<bool> {
    pub fn assert(&self, expected: &[Option<bool>]) {
        self.assert_iter(expected, |actual, expected| {
            return actual == expected;
        })
    }
}

impl ComponentTestSnapshot<i32> {
    pub fn assert(&self, expected: &[Option<i32>]) {
        self.assert_iter(expected, |actual, expected| {
            return actual == expected;
        })
    }
}

impl ComponentTestSnapshot<f64> {
    pub fn assert(&self, expected: &[Option<f64>]) {
        self.assert_iter(expected, |actual, expected| {
            return actual.compare(*expected);
        })
    }
}

impl ComponentTestSnapshot<TradeDirection> {
    pub fn assert(&self, expected: &[Option<TradeDirection>]) {
        self.assert_iter(expected, |actual, expected| {
            return actual == expected;
        })
    }
}

impl ComponentTestSnapshot<(Option<f64>, Option<f64>, bool)> {
    pub fn assert(&self, expected: &[Option<(Option<f64>, Option<f64>, bool)>]) {
        self.assert_iter(expected, |actual, expected| {
            let is_first_valid = match (actual.0, expected.0) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            let is_second_valid = match (actual.1, expected.1) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            return is_first_valid && is_second_valid && actual.2 == expected.2;
        })
    }
}

impl ComponentTestSnapshot<(Option<f64>, Option<f64>)> {
    pub fn assert(&self, expected: &[Option<(Option<f64>, Option<f64>)>]) {
        self.assert_iter(expected, |actual, expected| match (actual.0, expected.0) {
            (None, None) => true,
            (Some(_actual), Some(_expected)) => _actual.compare(_expected),
            _ => false,
        })
    }
}

impl ComponentTestSnapshot<(Option<f64>, Option<f64>, Option<f64>)> {
    pub fn assert(&self, expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>]) {
        self.assert_iter(expected, |actual, expected| {
            let is_first_valid = match (actual.0, expected.0) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            let is_second_valid = match (actual.1, expected.1) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            let is_third_valid = match (actual.2, expected.2) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            return is_first_valid && is_second_valid && is_third_valid;
        })
    }
}

impl ComponentTestSnapshot<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)> {
    pub fn assert(
        &self,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        self.assert_iter(expected, |actual, expected| {
            let is_first_valid = match (actual.0, expected.0) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            let is_second_valid = match (actual.1, expected.1) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            let is_third_valid = match (actual.2, expected.2) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            let is_fourth_valid = match (actual.3, expected.3) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            return is_first_valid && is_second_valid && is_third_valid && is_fourth_valid;
        })
    }
}

impl ComponentTestSnapshot<(Vec<Order>, Vec<Order>)> {
    pub fn assert(&self, expected: &[Option<(Vec<Order>, Vec<Order>)>]) {
        self.assert_iter(expected, |actual, expected| {
            if actual.0.len() != expected.0.len() || actual.1.len() != expected.1.len() {
                return false;
            }
            for (i, actual_order) in actual.0.iter().enumerate() {
                let expected_order = expected.0[i];
                if *actual_order != expected_order {
                    return false;
                }
            }
            for (i, actual_order) in actual.1.iter().enumerate() {
                let expected_order = expected.1[i];
                if *actual_order != expected_order {
                    return false;
                }
            }
            return true;
        })
    }
}

impl ComponentTestSnapshot<Trade> {
    pub fn assert(&self, expected: &[Option<Trade>]) {
        self.assert_iter(expected, |actual, expected| {
            return actual == expected;
        })
    }
}

impl ComponentTestSnapshot<(Option<Trade>, Vec<Trade>)> {
    pub fn assert(&self, expected: &[Option<(Option<Trade>, Vec<Trade>)>]) {
        self.assert_iter(expected, |actual, expected| {
            if actual.0 != expected.0 {
                return false;
            }
            if actual.1.len() != expected.1.len() {
                return false;
            }
            for (i, actual_trade) in actual.1.iter().enumerate() {
                let expected_trade = expected.1[i];
                if *actual_trade != expected_trade {
                    return false;
                }
            }
            return true;
        })
    }
}

impl ComponentTestSnapshot<Vec<Trade>> {
    pub fn assert(&self, expected: &[Option<Vec<Trade>>]) {
        self.assert_iter(expected, |actual, expected| {
            if actual.len() != expected.len() {
                return false;
            }
            for (i, actual_trade) in actual.iter().enumerate() {
                let expected_trade = expected[i];
                if *actual_trade != expected_trade {
                    return false;
                }
            }
            return true;
        })
    }
}
