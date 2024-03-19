use crate::utils::float::Float64Utils;

pub trait PineScriptFloat64 {
    fn ps_is_zero(self) -> bool;
    fn ps_is_not_zero(self) -> bool;
    /// Same as `sign` in PineScript.
    fn ps_sign(self) -> f64;
    /// Same as `na` in PineScript.
    fn ps_na(self) -> bool;
    /// Same as `nz` in PineScript.
    fn ps_nz(self) -> f64;
    /// Same as `nz(value, replacement)` in PineScript.
    fn ps_nz_with(self, replacement: f64) -> f64;
    /// Returns the maximum of two values. Same as `math.max` in PineScript.
    fn ps_max(self, other: f64) -> f64;
    /// Returns the minimum of two values. Same as `math.min` in PineScript.
    fn ps_min(self, other: f64) -> f64;
    fn ps_to_bool(self) -> bool;
    fn ps_normalize(self) -> f64;
    fn ps_log(self) -> f64;
    fn ps_exp(self) -> f64;
}

/// Returns the absolute value of a number. Same as `math.abs` in PineScript.
// fn ps_abs(value: f64) -> f64;

impl PineScriptFloat64 for f64 {
    fn ps_is_zero(self) -> bool {
        return !self.is_nan() && self == 0.0;
    }

    fn ps_is_not_zero(self) -> bool {
        return !self.is_nan() && self != 0.0;
    }

    fn ps_sign(self) -> f64 {
        if self.is_nan() {
            return f64::NAN;
        }
        if self > 0.0 {
            return 1.0;
        }
        if self < 0.0 {
            return -1.0;
        }
        return 0.0;
    }

    fn ps_na(self) -> bool {
        return self.is_nan();
    }

    fn ps_nz(self) -> f64 {
        if self.is_nan() {
            return 0.0;
        }
        return self;
    }

    fn ps_nz_with(self, replacement: f64) -> f64 {
        if self.is_nan() {
            return replacement;
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

    fn ps_to_bool(self) -> bool {
        return !self.is_zero();
    }

    fn ps_normalize(self) -> f64 {
        if self.is_infinite() {
            return f64::NAN;
        }
        return self;
    }

    fn ps_log(self) -> f64 {
        return self.ln().ps_normalize();
    }

    fn ps_exp(self) -> f64 {
        return self.exp().ps_normalize();
    }

    // fn ps_abs(value: f64) -> f64 {
    //     if value.is_nan() {
    //         return f64::NAN;
    //     }
    //     return f64::abs(value);
    // }
}
