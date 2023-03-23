use colored::Colorize;

use crate::strategy::trade::TradeDirection;

use super::comparison::FloatComparison;

pub struct ArraySnapshot<T> {
    pub debug_mode: bool,
    pub print_max_index: Option<usize>,
    pub actual: Vec<T>,
}

pub trait Compare<T> {
    fn compare(&self, other: &T) -> bool;
}

impl<T: std::fmt::Debug> ArraySnapshot<T> {
    pub fn new() -> Self {
        return ArraySnapshot::<T> {
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

    pub fn actual(&mut self, value: Vec<T>) {
        self.actual = value;
    }

    pub fn push(&mut self, value: T) {
        self.actual.push(value);
    }

    pub fn assert_iter(&self, expected: &[T], compare_delegate: fn(&T, &T) -> bool) {
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
            let is_equal = compare_delegate(actual, expected);
            if !is_equal {
                println!(
                    "{}: {} | {}\n",
                    format!("[{:?}]", i).red().bold(),
                    format!("{:?}", actual).black().on_bright_red().bold(),
                    format!("{:?}", expected).black().on_green().bold(),
                );
                if !self.debug_mode {
                    panic!("Array snapshot assertion failed at index {}", i);
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

impl ArraySnapshot<Option<i32>> {
    pub fn assert(&self, expected: &[Option<i32>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (Some(actual), Some(expected)) => actual == expected,
            (None, None) => true,
            _ => false,
        });
    }
}

impl ArraySnapshot<Option<f64>> {
    pub fn assert(&self, expected: &[Option<f64>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (Some(actual), Some(expected)) => (*actual).compare(*expected),
            (None, None) => true,
            _ => false,
        });
    }
}

impl ArraySnapshot<Option<bool>> {
    pub fn assert(&self, expected: &[Option<bool>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (Some(actual), Some(expected)) => actual == expected,
            (None, None) => true,
            _ => false,
        });
    }
}

impl ArraySnapshot<(Option<f64>, Option<f64>, bool)> {
    pub fn assert(&self, expected: &[(Option<f64>, Option<f64>, bool)]) {
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
        });
    }
}

impl ArraySnapshot<Option<(f64, f64)>> {
    pub fn assert(&self, expected: &[Option<(f64, f64)>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (None, None) => true,
            (Some(actual), Some(expected)) => {
                actual.0.compare(expected.0) && actual.1.compare(expected.1)
            }
            _ => false,
        })
    }
}

impl ArraySnapshot<Option<(Option<f64>, Option<f64>)>> {
    pub fn assert(&self, expected: &[Option<(Option<f64>, Option<f64>)>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (None, None) => true,
            (Some(actual), Some(expected)) => match (actual.0, expected.0) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            },
            _ => false,
        })
    }
}

impl ArraySnapshot<Option<(Option<f64>, Option<f64>, Option<f64>)>> {
    pub fn assert(&self, expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (None, None) => true,
            (Some(actual), Some(expected)) => {
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
            }
            _ => false,
        })
    }
}

impl ArraySnapshot<Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>> {
    pub fn assert(
        &self,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (None, None) => true,
            (Some(actual), Some(expected)) => {
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
            }
            _ => false,
        })
    }
}

impl ArraySnapshot<Option<TradeDirection>> {
    pub fn assert(&self, expected: &[Option<TradeDirection>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (Some(actual), Some(expected)) => actual == expected,
            (None, None) => true,
            _ => false,
        });
    }
}
