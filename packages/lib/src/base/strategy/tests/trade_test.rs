#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::base::strategy::trade::{
        compute_fill_size, compute_pnl, compute_return, compute_trade_pnl, Trade, TradeDirection,
    };

    #[test]
    fn trade_get_opposite_from_short() {
        assert_eq!(
            TradeDirection::Short.get_opposite(),
            TradeDirection::Long,
            "Should be Long"
        );
    }

    #[test]
    fn trade_get_opposite_from_long() {
        assert_eq!(
            TradeDirection::Long.get_opposite(),
            TradeDirection::Short,
            "Should be Short"
        );
    }

    #[test]
    fn compute_fill_size_equity_greater_than_price() {
        assert_eq!(compute_fill_size(100.0, 20.0), 5.0);
    }

    #[test]
    fn compute_fill_size_equity_lesser_than_price() {
        assert_eq!(compute_fill_size(10.0, 50.0), 0.2);
    }

    #[test]
    fn compute_fill_size_equity_0() {
        assert_eq!(compute_fill_size(0.0, 50.0), 0.0);
    }

    #[test]
    fn compute_fill_size_price_0() {
        assert_eq!(compute_fill_size(50.0, 0.0), 0.0);
    }

    #[test]
    fn compute_pnl_profit() {
        assert_eq!(compute_pnl(100.0, 50.0), 50.0);
    }

    #[test]
    fn compute_pnl_loss() {
        assert_eq!(compute_pnl(50.0, 100.0), -50.0);
    }

    #[test]
    fn compute_return_profit() {
        assert_eq!(compute_return(100.0, 50.0), 1.0);
    }

    #[test]
    fn compute_return_loss() {
        assert_eq!(compute_return(-50.0, 100.0), -1.5);
    }

    #[test]
    fn compute_return_prev_equity_0() {
        assert_eq!(compute_return(-50.0, 0.0), 0.0);
    }

    #[test]
    fn compute_trade_pnl_long_profit() {
        assert_eq!(compute_trade_pnl(1.5, 10.0, 20.0, true), 15.0);
    }

    #[test]
    fn compute_trade_pnl_long_loss() {
        assert_eq!(compute_trade_pnl(1.5, 20.0, 10.0, true), -15.0);
    }

    #[test]
    fn compute_trade_pnl_short_profit() {
        assert_eq!(compute_trade_pnl(1.5, 20.0, 10.0, false), 15.0);
    }

    #[test]
    fn compute_trade_pnl_short_loss() {
        assert_eq!(compute_trade_pnl(1.5, 10.0, 20.0, false), -15.0);
    }

    #[test]
    fn trade_pnl_no_entry_price() {
        let mut trade = Trade::new(TradeDirection::Long);
        trade.entry_price = None;
        assert_eq!(trade.pnl(1.5, 20.0), None);
    }

    #[test]
    fn trade_pnl_long_profit() {
        let mut trade = Trade::new(TradeDirection::Long);
        trade.entry_price = Some(10.0);
        assert_eq!(trade.pnl(1.5, 20.0), Some(15.0));
    }

    #[test]
    fn trade_pnl_long_loss() {
        let mut trade = Trade::new(TradeDirection::Long);
        trade.entry_price = Some(20.0);
        assert_eq!(trade.pnl(1.5, 10.0), Some(-15.0));
    }

    #[test]
    fn trade_pnl_short_profit() {
        let mut trade = Trade::new(TradeDirection::Short);
        trade.entry_price = Some(20.0);
        assert_eq!(trade.pnl(1.5, 10.0), Some(15.0));
    }

    #[test]
    fn trade_pnl_short_loss() {
        let mut trade = Trade::new(TradeDirection::Short);
        trade.entry_price = Some(10.0);
        assert_eq!(trade.pnl(1.5, 20.0), Some(-15.0));
    }
}
