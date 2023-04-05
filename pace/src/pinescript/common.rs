pub trait PineScriptFloat64 {
    /// Same as `nz` in PineScript.
    fn ps_nz(self) -> f64;
    /// Returns the maximum of two values. Same as `math.max` in PineScript.
    fn ps_max(self, other: f64) -> f64;
    /// Returns the minimum of two values. Same as `math.min` in PineScript.
    fn ps_min(self, other: f64) -> f64;
}

/// Returns the absolute value of a number. Same as `math.abs` in PineScript.
// fn ps_abs(value: f64) -> f64;

impl PineScriptFloat64 for f64 {
    fn ps_nz(self) -> f64 {
        if self.is_nan() {
            return 0.0;
        }
        return self;
    }

    fn ps_max(self, other: f64) -> f64 {
        if self.is_nan() || other.is_nan() {
            return f64::NAN;
        }
        return f64::max(self, other);
    }

    fn ps_min(self, other: f64) -> f64 {
        if self.is_nan() || other.is_nan() {
            return f64::NAN;
        }
        return f64::min(self, other);
    }

    // fn ps_abs(value: f64) -> f64 {
    //     if value.is_nan() {
    //         return f64::NAN;
    //     }
    //     return f64::abs(value);
    // }
}
