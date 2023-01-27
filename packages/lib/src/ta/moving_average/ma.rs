pub struct MovingAverage;

impl MovingAverage {
    pub fn ohlc4(open: f64, high: f64, low: f64, close: f64) -> f64 {
        return (open + high + low + close) / 4.0;
    }

    pub fn hlc3(high: f64, low: f64, close: f64) -> f64 {
        return (high + low + close) / 3.0;
    }

    pub fn hl2(high: f64, low: f64) -> f64 {
        return (high + low) / 2.0;
    }
}
