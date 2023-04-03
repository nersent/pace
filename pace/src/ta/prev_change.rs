use crate::{
    common::window_cache::WindowCache,
    core::{context::Context, incremental::Incremental},
    pinescript::common::ps_diff,
};

pub struct PrevChange {
    pub ctx: Context,
    prev_value: Option<f64>,
}

impl PrevChange {
    pub fn new(ctx: Context) -> Self {
        return Self::with_initial(ctx, None);
    }

    pub fn with_initial(ctx: Context, initial_value: Option<f64>) -> Self {
        return Self {
            ctx: ctx.clone(),
            prev_value: initial_value,
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for PrevChange {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        let diff = ps_diff(value, self.prev_value);
        self.prev_value = value;
        return diff;
    }
}
