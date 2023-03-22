use crate::{
    core::{context::Context, incremental::Incremental},
    statistics::{mean::Mean, stdev::Stdev},
};

pub struct ReturnsData {
    pub delta: f64,
    pub stdev: f64,
    pub mean: f64,
}

impl Default for ReturnsData {
    fn default() -> Self {
        return Self {
            delta: 0.0,
            stdev: 0.0,
            mean: 0.0,
        };
    }
}

pub struct Returns {
    pub ctx: Context,
    pub data: ReturnsData,
    prev_value: f64,
    stdev: Stdev,
    mean: Mean,
}

impl Returns {
    pub fn new(ctx: Context, initial_value: f64) -> Self {
        return Self::build(ctx, initial_value, false);
    }

    pub fn build(ctx: Context, initial_value: f64, fast: bool) -> Self {
        return Self {
            ctx: ctx.clone(),
            data: ReturnsData::default(),
            prev_value: initial_value,
            stdev: Stdev::build(ctx.clone(), fast),
            mean: Mean::new(ctx.clone()),
        };
    }
}

impl Incremental<f64, ()> for Returns {
    fn next(&mut self, value: f64) {
        self.data.delta = value - self.prev_value;
        self.data.stdev = self.stdev.next(self.data.delta);
        self.data.mean = self.mean.next(self.data.delta);
    }
}
