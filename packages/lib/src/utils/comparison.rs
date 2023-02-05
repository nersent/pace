pub trait FloatComparison {
    fn compare_with_precision(&self, target: f64, precision: f64) -> bool;
    fn compare(&self, target: f64) -> bool;
}

impl FloatComparison for f64 {
    fn compare_with_precision(&self, target: f64, precision: f64) -> bool {
        return (self - target).abs() < precision;
    }

    fn compare(&self, target: f64) -> bool {
        return self.compare_with_precision(target, 0.00001);
    }
}
