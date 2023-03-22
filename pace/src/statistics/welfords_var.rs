use crate::core::{context::Context, incremental::Incremental};

/// Calculates variance using Welford's online algorithm. Has O(1) complexity.
pub struct WelfordsVar {
    pub ctx: Context,
    n: usize,
    mean: f64,
    deviation: f64,
}

impl WelfordsVar {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            mean: 0.0,
            deviation: 0.0,
            n: 0,
        };
    }
}

impl Incremental<f64, f64> for WelfordsVar {
    fn next(&mut self, value: f64) -> f64 {
        self.n += 1;

        let delta = value - self.mean;

        self.mean += delta / (self.n as f64);
        self.deviation += delta * (value - self.mean);

        if self.n <= 1 {
            return 0.0;
        }

        return self.deviation / (self.n as f64 - 1.0);
    }
}
