use super::{
    ema_component::ExponentialMovingAverageComponent, rma_component::RunningMovingAverageComponent,
    sma_component::SimpleMovingAverageComponent,
};

pub fn compute_ohlc4(open: f64, high: f64, low: f64, close: f64) -> f64 {
    return (open + high + low + close) / 4.0;
}

pub fn compute_hlc3(high: f64, low: f64, close: f64) -> f64 {
    return (high + low + close) / 3.0;
}

pub fn compute_hl2(high: f64, low: f64) -> f64 {
    return (high + low) / 2.0;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MovingAverageKind {
    SMA,
    EMA,
    RMA,
}

pub enum MovingAverageComponentUnion {
    SMA(SimpleMovingAverageComponent),
    EMA(ExponentialMovingAverageComponent),
    RMA(RunningMovingAverageComponent),
}
