#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };

    use polars::prelude::DataFrame;

    use crate::{
        core::{context::Context, incremental::Incremental},
        polars::series::SeriesCastUtils,
        strategy::{
            metrics::{
                cobra_metrics::{CobraMetrics, CobraMetricsConfig},
                common::{
                    avg_win_loss_ratio, gross_loss_percent, gross_profit_percent,
                    long_net_profit_percent, max_drawdown_percent, max_run_up_percent,
                    net_profit_percent, short_net_profit_percent,
                },
                equity_metrics::EquityMetrics,
                tradingview_metrics::{TradingViewMetrics, TradingViewMetricsConfig},
            },
            strategy::{Strategy, StrategyConfig},
            trade::TradeDirection,
        },
        testing::{
            array_snapshot::ArraySnapshot, comparison::FloatComparison, fixture::Fixture,
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/strategy/metrics/{}", path))
    }

    #[derive(Debug)]
    struct TestBaseMetrics {
        pub equity: f64,
        pub net_profit: f64,
        pub open_profit: f64,
        pub gross_profit: f64,
        pub gross_loss: f64,
        pub closed_trades: usize,
        pub winning_trades: usize,
        pub losing_trades: usize,
    }

    #[derive(Debug)]
    struct TestAdditionalBaseMetrics {
        pub net_equity: f64,
        pub long_net_profit: f64,
        pub short_net_profit: f64,
        pub max_drawdown: f64,
        pub max_run_up: f64,
        pub avg_winning_trade: f64,
        pub avg_losing_trade: f64,
        pub avg_trade: f64,
        pub avg_winning_losing_trade_ratio: f64,
        pub profit_factor: f64,
        pub profitable: f64,
    }

    #[derive(Debug)]
    struct TestCobraMetrics {
        pub equity_curve_max_dd: f64,
        pub intra_trade_max_dd: f64,
        pub profit_factor: f64,
        pub profitable: f64,
        pub trades: usize,
        pub net_profit_l_s_ratio: f64,
    }

    #[derive(Debug)]
    struct TestUtilityMetrics {
        pub net_profit_percent: f64,
        pub gross_profit_percent: f64,
        pub gross_loss_percent: f64,
        pub long_net_profit_percent: f64,
        pub short_net_profit_percent: f64,
        pub max_drawdown_percent: f64,
        pub net_equity_min: f64,
        pub net_equity_max: f64,
        pub max_run_up_percent: f64,
    }

    #[derive(Debug)]
    struct TestMetricsPayload {
        pub base: TestBaseMetrics,
        pub base_additional: TestAdditionalBaseMetrics,
        pub cobra_metrics: TestCobraMetrics,
        pub utility: TestUtilityMetrics,
    }

    impl ArraySnapshot<Option<TestMetricsPayload>> {
        pub fn assert(&self, expected: &[Option<TestMetricsPayload>]) {
            self.assert_iter(expected, |actual, expected| match (actual, expected) {
                (None, None) => true,
                (Some(actual), Some(expected)) => {
                    let are_base_valid = true
                        && actual.base.equity.compare(expected.base.equity)
                        && actual.base.net_profit.compare(expected.base.net_profit)
                        && actual.base.open_profit.compare(expected.base.open_profit)
                        && actual.base.gross_profit.compare(expected.base.gross_profit)
                        && actual.base.gross_loss.compare(expected.base.gross_loss)
                        && actual.base.closed_trades == expected.base.closed_trades
                        && actual.base.winning_trades == expected.base.winning_trades
                        && actual.base.losing_trades == expected.base.losing_trades;

                    let are_additional_base_valid = true
                        && actual
                            .base_additional
                            .net_equity
                            .compare(expected.base_additional.net_equity)
                        && actual
                            .base_additional
                            .long_net_profit
                            .compare(expected.base_additional.long_net_profit)
                        && actual
                            .base_additional
                            .short_net_profit
                            .compare(expected.base_additional.short_net_profit)
                        && actual
                            .base_additional
                            .max_drawdown
                            .compare(expected.base_additional.max_drawdown)
                        && actual
                            .base_additional
                            .max_run_up
                            .compare(expected.base_additional.max_run_up)
                        && actual
                            .base_additional
                            .avg_winning_trade
                            .compare(expected.base_additional.avg_winning_trade)
                        && actual
                            .base_additional
                            .avg_losing_trade
                            .compare(expected.base_additional.avg_losing_trade)
                        && actual
                            .base_additional
                            .avg_trade
                            .compare(expected.base_additional.avg_trade)
                        && actual
                            .base_additional
                            .avg_winning_losing_trade_ratio
                            .compare(expected.base_additional.avg_winning_losing_trade_ratio)
                        && actual
                            .base_additional
                            .profit_factor
                            .compare(expected.base_additional.profit_factor)
                        && actual
                            .base_additional
                            .profitable
                            .compare(expected.base_additional.profitable);

                    let are_cobra_valid = true
                        && actual
                            .cobra_metrics
                            .equity_curve_max_dd
                            .compare(expected.cobra_metrics.equity_curve_max_dd)
                        && actual
                            .cobra_metrics
                            .intra_trade_max_dd
                            .compare(expected.cobra_metrics.intra_trade_max_dd)
                        && actual
                            .cobra_metrics
                            .profit_factor
                            .compare(expected.cobra_metrics.profit_factor)
                        && actual
                            .cobra_metrics
                            .profitable
                            .compare(expected.cobra_metrics.profitable)
                        && actual.cobra_metrics.trades == expected.cobra_metrics.trades
                        && actual
                            .cobra_metrics
                            .net_profit_l_s_ratio
                            .compare(expected.cobra_metrics.net_profit_l_s_ratio);

                    let are_utility_valid = true
                        && actual
                            .utility
                            .net_profit_percent
                            .compare(expected.utility.net_profit_percent)
                        && actual
                            .utility
                            .gross_profit_percent
                            .compare(expected.utility.gross_profit_percent)
                        && actual
                            .utility
                            .gross_loss_percent
                            .compare(expected.utility.gross_loss_percent)
                        && actual
                            .utility
                            .long_net_profit_percent
                            .compare(expected.utility.long_net_profit_percent)
                        && actual
                            .utility
                            .short_net_profit_percent
                            .compare(expected.utility.short_net_profit_percent)
                        && actual
                            .utility
                            .net_equity_min
                            .compare(expected.utility.net_equity_min)
                        && actual
                            .utility
                            .net_equity_max
                            .compare(expected.utility.net_equity_max)
                        && actual
                            .utility
                            .max_drawdown_percent
                            .compare(expected.utility.max_drawdown_percent);

                    // return false;

                    return are_base_valid
                        && are_additional_base_valid
                        && are_cobra_valid
                        && are_utility_valid;
                }
                _ => false,
            })
        }
    }

    struct TestMetricsTarget {
        pub ctx: Context,
        pub long_entries: Vec<usize>,
        pub short_entries: Vec<usize>,
        pub equity_metrics: EquityMetrics,
        pub tradingview_metrics: TradingViewMetrics,
        pub cobra_metrics: CobraMetrics,
        strategy: Strategy,
        max_run_up_percent: f64,
    }

    impl TestMetricsTarget {
        pub fn new(
            ctx: Context,
            strategy_config: StrategyConfig,
            long_entries: Vec<usize>,
            short_entries: Vec<usize>,
        ) -> Self {
            let strategy = Strategy::new(ctx.clone(), strategy_config);
            return Self {
                ctx: ctx.clone(),
                equity_metrics: EquityMetrics::new(ctx.clone(), &strategy),
                tradingview_metrics: TradingViewMetrics::new(
                    ctx.clone(),
                    &strategy,
                    TradingViewMetricsConfig {
                        risk_free_rate: 0.0,
                    },
                ),
                cobra_metrics: CobraMetrics::new(
                    ctx.clone(),
                    &strategy,
                    CobraMetricsConfig {
                        estimated: false,
                        returns_start_year: Some(2018), // returns_start_year: Some(2018),
                    },
                ),
                strategy,
                long_entries,
                short_entries,
                max_run_up_percent: 0.0,
            };
        }

        pub fn next(&mut self) -> TestMetricsPayload {
            let tick = self.ctx.bar.index();

            let mut trade_direction: Option<TradeDirection> = None;

            if self.long_entries.contains(&tick) {
                trade_direction = Some(TradeDirection::Long);
            } else if self.short_entries.contains(&tick) {
                trade_direction = Some(TradeDirection::Short);
            }

            let initial_capital = self.strategy.config.initial_capital;

            self.strategy.next(trade_direction);

            let prev_max_run_up = self.tradingview_metrics.data.max_run_up;

            self.equity_metrics.next(&self.strategy);
            self.tradingview_metrics.next(&self.strategy);
            self.cobra_metrics.next(&self.strategy);

            let metrics = &self.strategy.metrics;
            let equity_metrics = &self.equity_metrics.data;
            let tradingview_metrics = &self.tradingview_metrics.data;
            let cobra_metrics = &self.cobra_metrics.data;

            if tradingview_metrics.max_run_up > prev_max_run_up {
                self.max_run_up_percent = max_run_up_percent(
                    tradingview_metrics.max_run_up,
                    equity_metrics.bar_equity_max,
                )
            }

            // println!("[{}]: {:?}", bar_index, cobra_metrics);
            // if bar_index < 1605 {}

            return TestMetricsPayload {
                base: TestBaseMetrics {
                    equity: equity_metrics.equity,
                    net_profit: tradingview_metrics.net_profit,
                    open_profit: tradingview_metrics.open_pl,
                    gross_profit: tradingview_metrics.gross_profit,
                    gross_loss: tradingview_metrics.gross_loss,
                    closed_trades: tradingview_metrics.total_closed_trades,
                    winning_trades: tradingview_metrics.number_winning_trades,
                    losing_trades: tradingview_metrics.number_losing_trades,
                },
                base_additional: TestAdditionalBaseMetrics {
                    net_equity: equity_metrics.net_equity,
                    long_net_profit: metrics.long_net_profit,
                    short_net_profit: metrics.short_net_profit,
                    max_drawdown: tradingview_metrics.max_drawdown,
                    max_run_up: tradingview_metrics.max_run_up,
                    avg_winning_trade: tradingview_metrics.avg_winning_trade,
                    avg_losing_trade: tradingview_metrics.avg_losing_trade,
                    avg_trade: tradingview_metrics.avg_trade,
                    avg_winning_losing_trade_ratio: avg_win_loss_ratio(
                        tradingview_metrics.avg_winning_trade,
                        tradingview_metrics.avg_losing_trade,
                    )
                    .unwrap_or(0.0),
                    profit_factor: tradingview_metrics.profit_factor,
                    profitable: tradingview_metrics.percent_profitable,
                },
                cobra_metrics: TestCobraMetrics {
                    equity_curve_max_dd: cobra_metrics.equity_curve_max_dd,
                    intra_trade_max_dd: cobra_metrics.intra_trade_max_dd,
                    profit_factor: cobra_metrics.profit_factor,
                    profitable: cobra_metrics.profitable,
                    trades: cobra_metrics.trades,
                    net_profit_l_s_ratio: cobra_metrics.net_profit_l_s_ratio,
                },
                utility: TestUtilityMetrics {
                    gross_loss_percent: gross_loss_percent(
                        tradingview_metrics.gross_loss,
                        initial_capital,
                    ),
                    gross_profit_percent: gross_profit_percent(
                        tradingview_metrics.gross_profit,
                        initial_capital,
                    ),
                    long_net_profit_percent: long_net_profit_percent(
                        metrics.long_net_profit,
                        initial_capital,
                    ),
                    net_profit_percent: net_profit_percent(metrics.net_profit, initial_capital),
                    short_net_profit_percent: short_net_profit_percent(
                        metrics.short_net_profit,
                        initial_capital,
                    ),
                    max_drawdown_percent: max_drawdown_percent(
                        tradingview_metrics.max_drawdown,
                        equity_metrics.net_equity_max,
                    ),
                    net_equity_min: equity_metrics.net_equity_min,
                    net_equity_max: equity_metrics.net_equity_max,
                    max_run_up_percent: self.max_run_up_percent,
                },
            };
        }
    }

    fn _test_metrics(target: &mut TestMetricsTarget, expected: &[Option<TestMetricsPayload>]) {
        let mut snapshot = ArraySnapshot::<Option<TestMetricsPayload>>::new();
        for _ in target.ctx.clone() {
            let payload = target.next();
            snapshot.push(Some(payload));
        }
        snapshot.assert(expected);
    }

    fn _load_metrics(df: &DataFrame) -> Vec<Option<TestMetricsPayload>> {
        // BASE METRICS
        let equity = df.column("_target_equity_").unwrap().to_f64();
        let net_profit = df.column("_target_net_profit_").unwrap().to_f64();
        let open_profit = df.column("_target_open_profit_").unwrap().to_f64();
        let gross_profit = df.column("_target_gross_profit_").unwrap().to_f64();
        let gross_loss = df.column("_target_gross_loss_").unwrap().to_f64();
        let closed_trades = df.column("_target_closed_trades_").unwrap().to_usize();
        let losing_trades = df.column("_target_losing_trades_").unwrap().to_usize();
        let winning_trades = df.column("_target_winning_trades_").unwrap().to_usize();

        // ADDITIONAL BASE METRICS
        let net_equity = df.column("_target_net_equity_").unwrap().to_f64();
        let long_net_profit = df.column("_target_long_net_profit_").unwrap().to_f64();
        let short_net_profit = df.column("_target_short_net_profit_").unwrap().to_f64();
        let max_drawdown = df.column("_target_max_drawdown_").unwrap().to_f64();
        let max_run_up = df.column("_target_max_run_up_").unwrap().to_f64();
        let avg_trade = df.column("_target_avg_trade_").unwrap().to_f64();
        let avg_winning_trade = df.column("_target_avg_winning_trade_").unwrap().to_f64();
        let avg_losing_trade = df.column("_target_avg_losing_trade_").unwrap().to_f64();
        let avg_win_loss_trade_ratio = df.column("_target_avg_win_loss_ratio_").unwrap().to_f64();

        // COBRA METRICS
        let equity_max_drawdown_percent = df
            .column("_target_equity_max_drawdown_percent_")
            .unwrap()
            .to_f64();
        let profit_factor = df.column("_target_profit_factor_").unwrap().to_f64();
        let percent_profitable = df.column("_target_percent_profitable_").unwrap().to_f64();
        let long_short_net_profit_ratio = df
            .column("_target_long_short_net_profit_ratio_")
            .unwrap()
            .to_f64();
        let intra_trade_max_drawdown_percent = df
            .column("_target_intra_trade_max_drawdown_percent_")
            .unwrap()
            .to_f64();

        // UTILITY METRICS
        let net_profit_percent = df.column("_target_net_profit_percent_").unwrap().to_f64();
        let gross_profit_percent = df.column("_target_gross_profit_percent_").unwrap().to_f64();
        let gross_loss_percent = df.column("_target_gross_loss_percent_").unwrap().to_f64();
        let long_net_profit_percent = df
            .column("_target_long_net_profit_percent_")
            .unwrap()
            .to_f64();
        let short_net_profit_percent = df
            .column("_target_short_net_profit_percent_")
            .unwrap()
            .to_f64();
        // let net_equity_max_drawdown_percent = df
        //     .column("_target_net_equity_max_drawdown_percent_")
        //     .unwrap()
        //     .to_f64();
        let net_equity_max_drawdown_percent =
            df.column("_target_max_drawdown_percent_").unwrap().to_f64();
        let net_equity_min = df.column("_target_net_equity_min_").unwrap().to_f64();
        let net_equity_max = df.column("_target_net_equity_max_").unwrap().to_f64();

        let mut metrics: Vec<Option<TestMetricsPayload>> = Vec::new();

        for i in 0..equity.len() {
            let m = TestMetricsPayload {
                base: TestBaseMetrics {
                    equity: equity[i].unwrap(),
                    net_profit: net_profit[i].unwrap(),
                    open_profit: open_profit[i].unwrap(),
                    gross_profit: gross_profit[i].unwrap(),
                    gross_loss: gross_loss[i].unwrap(),
                    closed_trades: closed_trades[i].unwrap(),
                    winning_trades: winning_trades[i].unwrap(),
                    losing_trades: losing_trades[i].unwrap(),
                },
                base_additional: TestAdditionalBaseMetrics {
                    net_equity: net_equity[i].unwrap(),
                    long_net_profit: long_net_profit[i].unwrap(),
                    short_net_profit: short_net_profit[i].unwrap(),
                    max_drawdown: max_drawdown[i].unwrap(),
                    max_run_up: max_run_up[i].unwrap(),
                    avg_winning_trade: avg_winning_trade[i].unwrap_or(0.0),
                    avg_losing_trade: avg_losing_trade[i].unwrap_or(0.0),
                    avg_trade: avg_trade[i].unwrap_or(0.0),
                    avg_winning_losing_trade_ratio: avg_win_loss_trade_ratio[i].unwrap_or(0.0),
                    profit_factor: profit_factor[i].unwrap_or(0.0),
                    profitable: percent_profitable[i].unwrap_or(0.0),
                },
                cobra_metrics: TestCobraMetrics {
                    equity_curve_max_dd: equity_max_drawdown_percent[i].unwrap(),
                    intra_trade_max_dd: intra_trade_max_drawdown_percent[i].unwrap_or(0.0),
                    profit_factor: profit_factor[i].unwrap_or(0.0),
                    profitable: percent_profitable[i].unwrap_or(0.0),
                    trades: closed_trades[i].unwrap(),
                    net_profit_l_s_ratio: long_short_net_profit_ratio[i].unwrap_or(0.0),
                },
                utility: TestUtilityMetrics {
                    gross_loss_percent: gross_loss_percent[i].unwrap_or(0.0),
                    gross_profit_percent: gross_profit_percent[i].unwrap_or(0.0),
                    long_net_profit_percent: long_net_profit_percent[i].unwrap_or(0.0),
                    net_profit_percent: net_profit_percent[i].unwrap_or(0.0),
                    short_net_profit_percent: short_net_profit_percent[i].unwrap_or(0.0),
                    max_drawdown_percent: net_equity_max_drawdown_percent[i].unwrap_or(0.0),
                    net_equity_min: net_equity_min[i].unwrap_or(0.0),
                    net_equity_max: net_equity_max[i].unwrap_or(0.0),
                    max_run_up_percent: 0.0,
                },
            };
            metrics.push(Some(m));
        }

        return metrics;
    }

    #[test]
    fn on_next_bar_open_continous_extensive() {
        let (df, ctx) = Fixture::load_ctx(&format_path("next_bar_continous.csv"));
        let expected = _load_metrics(&df);

        _test_metrics(
            &mut TestMetricsTarget::new(
                ctx.clone(),
                StrategyConfig {
                    continous: true,
                    on_bar_close: false,
                    initial_capital: 1000.0,
                    buy_with_equity: false,
                },
                // Long entries
                vec![2, 18, 44, 60, 120, 180, 400, 700, 1000, 1600],
                // Short entries
                vec![10, 24, 48, 64, 155, 190, 420, 900, 1250],
            ),
            &expected,
        );
    }
}
