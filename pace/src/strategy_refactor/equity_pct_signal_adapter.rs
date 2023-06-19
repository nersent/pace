use crate::{core::context::Context, utils::float::Float64Utils};

use super::{
    common::{OrderConfig, Qty, Signal},
    strategy::Strategy,
    trade::TradeDirection,
    utils::order_size,
};

pub struct EquityPctSignalAdapter {
    pub ctx: Context,
    pub prev_equity_pct: f64,
}

pub fn compute_order_size_for_equity_pct(
    equity_pct: f64,
    equity: f64,
    current_position: f64,
    instrument_price: f64,
    point_value: f64,
    exchange_rate: f64,
) -> f64 {
    let equity_order_size = order_size(
        equity_pct.abs(),
        equity,
        exchange_rate,
        instrument_price,
        point_value,
    );

    let sign = equity_pct.signum();

    let mut order_size = equity_order_size * sign - current_position;

    return order_size;
}

impl EquityPctSignalAdapter {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx,
            prev_equity_pct: 0.0,
        };
    }

    pub fn next(&mut self, equity_pct: f64, strategy: &Strategy) -> Signal {
        let mut signal: Signal = Signal::Hold;

        if strategy.equity > 0.0 {
            if !equity_pct.compare(self.prev_equity_pct) {
                self.prev_equity_pct = equity_pct;

                let order_size = compute_order_size_for_equity_pct(
                    equity_pct,
                    strategy.equity,
                    strategy.position_size,
                    strategy.get_instrument_price(),
                    1.0,
                    1.0,
                );

                let order_size = self.ctx.round_contracts(order_size);

                if order_size == 0.0 {
                    return signal;
                }

                let direction = if order_size > 0.0 {
                    TradeDirection::Long
                } else {
                    TradeDirection::Short
                };

                let order_size = order_size.abs();
                // assert!(
                //     order_size > 0.0,
                //     "\nOrder size must be greater than 0.\nOrder Size: {} | Current Position: {} | Prev Equity %: {} | Equity %: {} | Trade Direction: {:?} | Equity: {}",
                //     order_size,
                //     strategy.position_size,
                //     self.prev_equity_pct,
                //     equity_pct,
                //     direction,
                //     strategy.equity
                // );

                // let mut order_size_diff = order_size + strategy.position_size;

                let order = OrderConfig {
                    id: self.ctx.bar.index().to_string(),
                    direction,
                    qty: Qty::Contracts(order_size),
                };

                signal = Signal::Order(order);

                self.prev_equity_pct = equity_pct;
            }
        }

        return signal;
    }
}
