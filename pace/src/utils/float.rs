pub trait Float64Utils {
    const PRICE_PRECISION: f64;
    fn to_option(self) -> Option<f64>;
    fn normalize(self) -> f64;
    fn is_zero(self) -> bool;
    fn compare_with_precision(&self, target: f64, precision: f64) -> bool;
    fn compare(&self, target: f64) -> bool;
    fn is_non_zero(self) -> bool;
}

impl Float64Utils for f64 {
    const PRICE_PRECISION: f64 = 0.01;

    fn to_option(self) -> Option<f64> {
        if self.is_nan() {
            return None;
        }
        return Some(self);
    }

    fn normalize(self) -> f64 {
        if self.is_normal() {
            return self;
        }
        return f64::NAN;
    }

    fn is_zero(self) -> bool {
        return !self.is_nan() && self.compare(0.0);
    }

    fn is_non_zero(self) -> bool {
        return !self.is_nan() && !self.compare(0.0);
    }

    fn compare_with_precision(&self, target: f64, precision: f64) -> bool {
        if self.is_nan() {
            return target.is_nan();
        }
        return (self - target).abs() < precision;
    }

    fn compare(&self, target: f64) -> bool {
        return self.compare_with_precision(target, 0.00001);
    }
}

pub trait OptionFloatUtils {
    fn unwrap_nan(self) -> f64;
}

impl OptionFloatUtils for Option<f64> {
    fn unwrap_nan(self) -> f64 {
        return self.unwrap_or(f64::NAN);
    }
}

impl OptionFloatUtils for Option<i32> {
    fn unwrap_nan(self) -> f64 {
        return self.map_or(f64::NAN, |v| v as f64);
    }
}

impl OptionFloatUtils for Option<usize> {
    fn unwrap_nan(self) -> f64 {
        return self.map_or(f64::NAN, |v| v as f64);
    }
}
