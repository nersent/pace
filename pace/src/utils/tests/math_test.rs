#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{
            average_true_range::Atr, bars_since::BarsSince, change::Change, dev::Dev,
            exponential_moving_average::Ema,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
        utils::{
            float::{Float64Utils, OptionFloatUtils},
            math::round_to_min_tick,
        },
    };

    #[test]
    fn _round_to_min_tick() {
        // https://www.tradingcode.net/tradingview/round-to-tick/#round-up-and-down-to-nearest-tick
        assert!(round_to_min_tick(2.69, 0.25).compare(2.75));
        assert!(round_to_min_tick(132.923, 0.25).compare(133.00));
        assert!(round_to_min_tick(-38.32, 0.25).compare(-38.25));
        assert!(round_to_min_tick(-9.8953, 0.25).compare(-10.00));
        assert!(round_to_min_tick(f64::NAN, 0.25).compare(0.0));
        assert!(round_to_min_tick(-4.75, 0.25).compare(-4.75));
        assert!(round_to_min_tick(5.92991, 0.25).compare(6.00));
    }
}
