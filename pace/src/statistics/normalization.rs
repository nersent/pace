use crate::{
    core::{context::Context, incremental::Incremental},
    utils::float::Float64Utils,
};

use super::{mean::Mean, stdev::Stdev};

pub fn clip_value(value: f64, min: f64, max: f64) -> f64 {
    return f64::min(f64::max(value, min), max);
}

pub fn normalize_value(value: f64, min: f64, max: f64) -> f64 {
    return (value - min) / (max - min);
}

pub fn scale_value_up(value: f64, threshold: f64, max: f64) -> f64 {
    if value <= threshold {
        return 0.0;
    }

    let delta = value - threshold;
    let mean = max - threshold;

    return clip_value(delta / mean, 0.0, 1.0);
}

pub fn scale_value_down(value: f64, threshold: f64, min: f64) -> f64 {
    if value >= threshold {
        return 0.0;
    }

    let delta = value - threshold;
    let mean = min - threshold;
    return clip_value(delta / mean, 0.0, 1.0);
}

pub fn scale_value_centered(value: f64, mean: f64, min: f64, max: f64) -> f64 {
    let distance = (max - min) / 2.0;
    let abs_diff = (value - mean).abs();
    return clip_value(1.0 - abs_diff / distance, 0.0, 1.0);
}

pub fn scale_value_around_mean(value: f64, mean: f64) -> f64 {
    return (2.0 * value - mean) / mean;
}

pub fn scale_value_min_max(value: f64, min: f64, max: f64) -> f64 {
    let mut value = value;
    let mut min = min;
    let mut max = max;
    if min < 0.0 {
        let offset = min.abs();
        min = 0.0;
        max += offset;
        value += offset;
    }
    let mean = max - min;
    return scale_value_around_mean(value, mean);
}

pub fn rescale(src: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    if old_min.compare(old_max) || new_min.compare(new_max) {
        panic!(
            "Invalid range: {} - {} -> {} - {}",
            old_min, old_max, new_min, new_max
        );
    }
    return new_min
        + (new_max - new_min) * (src - old_min) / f64::max(old_max - old_min, f64::EPSILON);
}

pub fn zscore(value: f64, mean: f64, std: f64) -> f64 {
    return (value - mean) / f64::max(std, f64::EPSILON);
}

pub struct MinMaxScalerConfig {
    pub min: f64,
    pub max: f64,
}

impl Default for MinMaxScalerConfig {
    fn default() -> Self {
        return Self {
            min: -1.0,
            max: 1.0,
        };
    }
}

/// Transforms the value to the given range based on past min and max values.
pub struct MinMaxScaler {
    pub ctx: Context,
    pub config: MinMaxScalerConfig,
    pub data_min: f64,
    pub data_max: f64,
}

impl MinMaxScaler {
    pub fn new(ctx: Context, config: MinMaxScalerConfig) -> Self {
        return Self {
            ctx,
            config,
            data_min: f64::MAX,
            data_max: f64::MIN,
        };
    }
}

impl Incremental<f64, f64> for MinMaxScaler {
    fn next(&mut self, value: f64) -> f64 {
        self.data_min = f64::min(self.data_min, value);
        self.data_max = f64::max(self.data_max, value);

        return rescale(
            value,
            self.data_min,
            self.data_max,
            self.config.min,
            self.config.max,
        );
    }
}

pub struct FixedScalerConfig {
    pub data_min: f64,
    pub data_max: f64,
    pub min: f64,
    pub max: f64,
}

/// Transforms the value to the fixed given range.
pub struct FixedScaler {
    pub ctx: Context,
    pub config: FixedScalerConfig,
}

impl FixedScaler {
    pub fn new(ctx: Context, config: FixedScalerConfig) -> Self {
        return Self { ctx, config };
    }
}

impl Incremental<f64, f64> for FixedScaler {
    fn next(&mut self, value: f64) -> f64 {
        return rescale(
            value,
            self.config.data_min,
            self.config.data_max,
            self.config.min,
            self.config.max,
        );
    }
}

pub struct StandardScalerConfig {
    pub estimate: bool,
}

impl Default for StandardScalerConfig {
    fn default() -> Self {
        return Self { estimate: true };
    }
}

/// Transforms the value to using z-score based on past min and max values.
pub struct StandardScaler {
    pub ctx: Context,
    pub config: StandardScalerConfig,
    stdev: Stdev,
    mean: Mean,
}

impl StandardScaler {
    pub fn new(ctx: Context, config: StandardScalerConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            stdev: Stdev::build(ctx.clone(), config.estimate),
            mean: Mean::new(ctx.clone()),
            config,
        };
    }
}

impl Incremental<f64, f64> for StandardScaler {
    fn next(&mut self, value: f64) -> f64 {
        return zscore(value, self.mean.next(value), self.stdev.next(value));
    }
}
