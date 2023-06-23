use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
};

/// Similar implementation of Pinescript `float` type.
/// Previous values are accessible via `[]` operator, but are stored only after `next()` call. Assigning a value does't add it immediately to the series.
pub struct Float {
    pub ctx: Context,
    series: FloatSeries,
    _value: f64,
}

impl Float {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            series: FloatSeries::new(ctx),
            _value: f64::NAN,
        };
    }

    pub fn with_initial_value(mut self, value: f64) -> Self {
        self._value = value;
        return self;
    }

    /// PineScript `:=` operator.
    pub fn set(&mut self, value: f64) {
        self._value = value;
    }

    pub fn value(&self) -> f64 {
        return self._value;
    }

    pub fn get(&self, index: usize) -> f64 {
        if index == 0 {
            return self._value;
        }
        return self.series.get(index - 1);
    }
}

impl Incremental<(), ()> for Float {
    /// Finalizes the value by adding it to the series, so it is accessible via `[]` operator (max bars back)
    /// Should be called after all calculations are done, at the end of the bar.
    fn next(&mut self, _: ()) {
        self.series.next(self._value);
    }
}

impl Into<f64> for Float {
    fn into(self) -> f64 {
        return self.value();
    }
}
