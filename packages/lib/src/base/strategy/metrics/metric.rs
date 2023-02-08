use super::{
    equity_metric::{Equity, EquityMetric},
    omega_ratio_metric::OmegaRatioMetric,
    sharpe_ratio_metric::SharpeRatioMetric,
    total_closed_trades_metric::TotalClosedTradesMetric,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MetricKind {
    Equity,
    OmegaRatio,
    SharpeRatio,
    TotalClosedTrades,
}

pub enum MetricComponentUnion {
    Equity(EquityMetric),
    OmegaRatio(OmegaRatioMetric),
    SharpeRatio(SharpeRatioMetric),
    TotalClosedTrades(TotalClosedTradesMetric),
}

pub enum MetricComponentResultUnion {
    Equity(Equity),
    OmegaRatio(f64),
    SharpeRatio(f64),
    TotalClosedTrades(usize),
}
