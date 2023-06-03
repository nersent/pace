use crate::utils::float::Float64Utils;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Trend {
    Bearish = -1,
    Neutral = 0,
    Bullish = 1,
    Strong = 2,
}

impl From<f64> for Trend {
    fn from(value: f64) -> Self {
        if value.compare(-1.0) {
            return Trend::Bearish;
        }
        if value.compare(1.0) {
            return Trend::Bullish;
        }
        return Trend::Neutral;
    }
}

impl Into<i32> for Trend {
    fn into(self) -> i32 {
        match self {
            Trend::Bullish => 1,
            Trend::Neutral => 0,
            Trend::Bearish => -1,
            Trend::Strong => 2,
        }
    }
}

impl Into<f64> for Trend {
    fn into(self) -> f64 {
        let i: i32 = self.into();
        return i as f64;
    }
}
