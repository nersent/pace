use std::path::{Path, PathBuf};

use colored::Colorize;
use polars::prelude::DataFrame;

use crate::{
    asset::timeframe::Timeframe,
    data::{csv::read_csv, polars::SeriesCastUtils},
    math::comparison::FloatComparison,
    strategy::{action::StrategyActionKind, polars::SeriesCastUtilsForStrategy},
};

use super::component_context::ComponentContext;

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

impl ComponentTestSnapshot<StrategyActionKind> {
    pub fn assert(&self, expected: &[Option<StrategyActionKind>]) {
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
