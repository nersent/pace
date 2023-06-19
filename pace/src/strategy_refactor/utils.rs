// the calculated order size, rounded down to the smallest trade quantity. For stocks, futures, CFDs, and forex that minimum quantity is 1. For Bitcoin (BTCUSD) it’s 0.000001 and Ethereum (ETHUSD) uses 0.0001.
//
// https://www.tradingcode.net/tradingview/equity-percent-default-order/#order-size-formula
pub fn order_size(
    // the percentage of current strategy equity to invest in each order. This percentage is from the default_qty_value setting or the manual ‘Order size’ option in the strategy’s settings window.
    equity_pct: f64,
    // the strategy’s current equity, which is the sum of its initial capital, closed net profit, and open position profit.
    // Since TradingView includes open profit/loss in the equity, part of the calculated order size is based on a position result we haven’t realised yet.
    // When that position closes with a much worse result than earlier calculated with, the order size is a bigger percentage of equity than we want. And if that position closes with much more profit, the computed order size is too small.
    equity: f64,
    // the price of the currency pair TradingView uses to perform a currency conversion, if applicable. There are two possibilities with this variable:
    // If the strategy currency and instrument currency are the same, no currency conversion happens and we multiply with 1.
    // If the strategy currency differs from the instrument currency, TradingView does a currency conversion with the appropriate currency pair.
    exchange_rate: f64,
    // the last available price from the bar that the order generates on. The code equivalent of that price is the close variable.
    // That instrument price is the bar’s close price, unless we use the ‘Recalculate After Order Filled’ and/or ‘Recalculate On Every Tick’ options. With those options the instrument price can also be a price from inside the bar.
    // This instrument price is from the bar on which the order generates. That’s often a different bar than the one on which the order fills. For instance, when the strategy generates a market order on a closed bar, that order fills one bar later at the open.
    instrument_price: f64,
    // the instrument’s point value, which the currency amount for one full point of price movement. For stocks, that’s 1. For the E-mini Nasdaq 100 future, that’s 20.
    point_value: f64,
) -> f64 {
    assert!(equity >= 0.0, "Equity must be greater than 0");
    assert!(
        equity_pct >= 0.0,
        "Equity percentage must be greater than 0"
    );
    return (equity_pct * equity * exchange_rate) / (instrument_price * point_value);
}

pub fn trade_profit(size: f64, entry_price: f64, current_price: f64, is_long: bool) -> f64 {
    let multiplier = if is_long { 1.0 } else { -1.0 };
    return (current_price - entry_price) * size * multiplier;
}
