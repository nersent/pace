use chrono::Duration;

use crate::core::{context::Context, incremental::Incremental};

pub struct TimeframeAdapterConfig<T, R> {
    pub inner: Box<dyn Incremental<T, R>>,
    pub should_update_delegate: Box<dyn Fn(&TimeframeAdapter<T, Option<R>>) -> bool>,
    // pub interval_delegate:
}

pub struct TimeframeAdapter<T, R> {
    pub ctx: Context,
    pub config: TimeframeAdapterConfig<T, R>,
    pub prev_value: Option<R>,
    pub prev_timestamp: Option<Duration>,
}

impl<T, R> TimeframeAdapter<T, R> {
    pub fn new(
        ctx: Context,
        config: TimeframeAdapterConfig<T, R>,
        initial_value: Option<R>,
        initial_timestamp: Option<Duration>,
    ) -> Self {
        Self {
            ctx,
            config,
            prev_value: initial_value,
            prev_timestamp: initial_timestamp,
        }
    }
}

impl<T, R> Incremental<T, Option<R>> for TimeframeAdapter<T, R> {
    fn next(&mut self, input: T) -> Option<R> {
        let should_update = (self.config.should_update_delegate)(&self);

        if update {
            let bar = &self.ctx.bar;
            let current_timestamp = bar.time();

            self.prev_timestamp = Some(current_timestamp);
            self.prev_value = self.config.inner.next(input);
        }

        return self.prev_value;
    }
}
