#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };

    use polars::prelude::DataFrame;

    use crate::{
        core::{
            context::Context, data_provider::DataProvider,
            in_memory_data_provider::InMemoryDataProvider, incremental::Incremental,
        },
        pinescript::common::PineScriptFloat64,
        polars::series::SeriesCastUtils,
        strategy::{
            strategy::{Strategy, StrategyConfig},
            trade::{StrategySignal, Trade, TradeDirection},
        },
        testing::{
            array_snapshot::ArraySnapshot, fixture::Fixture, pace::format_pace_fixture_path,
        },
        utils::float::Float64Utils,
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/strategy/execution/{}", path))
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct TestTradePayload {
        pub direction: TradeDirection,
        pub is_closed: bool,
        pub entry_tick: Option<usize>,
        pub entry_price: Option<f64>,
        pub exit_tick: Option<usize>,
        pub exit_price: Option<f64>,
    }

    impl TestTradePayload {
        pub fn from_trade(trade: &Trade) -> Self {
            Self {
                direction: trade.direction,
                is_closed: trade.is_closed,
                entry_tick: trade.entry_tick,
                entry_price: trade.entry_price.to_option(),
                exit_tick: trade.exit_tick,
                exit_price: trade.exit_price.to_option(),
            }
        }
    }

    impl ArraySnapshot<Option<Vec<TestTradePayload>>> {
        pub fn assert(&self, expected: &[Option<Vec<TestTradePayload>>]) {
            self.assert_iter(expected, |actual, expected| match (actual, expected) {
                (None, None) => true,
                (Some(actual), Some(expected)) => {
                    if actual.len() != expected.len() {
                        return false;
                    }
                    for (i, actual_trade) in actual.iter().enumerate() {
                        let expected_trade = expected[i];
                        if *actual_trade != expected_trade {
                            return false;
                        }
                    }
                    return true;
                }
                _ => false,
            })
        }
    }

    impl ArraySnapshot<Option<(Option<TestTradePayload>, Vec<TestTradePayload>)>> {
        pub fn assert(
            &self,
            expected: &[Option<(Option<TestTradePayload>, Vec<TestTradePayload>)>],
        ) {
            self.assert_iter(expected, |actual, expected| match (actual, expected) {
                (None, None) => true,
                (Some(actual), Some(expected)) => {
                    if actual.0 != expected.0 {
                        return false;
                    }
                    if actual.1.len() != expected.1.len() {
                        return false;
                    }
                    for (i, actual_trade) in actual.1.iter().enumerate() {
                        let expected_trade = expected.1[i];
                        if *actual_trade != expected_trade {
                            return false;
                        }
                    }
                    return true;
                }
                _ => false,
            })
        }
    }

    fn _test(
        target: &mut Strategy,
        trades: &[Option<TradeDirection>],
        expected: &[Option<(Option<TestTradePayload>, Vec<TestTradePayload>)>],
    ) {
        let mut snapshot =
            ArraySnapshot::<Option<(Option<TestTradePayload>, Vec<TestTradePayload>)>>::new();
        for _ in target.ctx.clone() {
            let tick = target.ctx.bar.index();
            let trade_direction = trades[tick];

            let signal: StrategySignal = match trade_direction {
                Some(TradeDirection::Long) => StrategySignal::Long,
                Some(TradeDirection::Short) => StrategySignal::Short,
                None => StrategySignal::Hold,
            };
            target.next_bar();
            target.next(signal);
            let trades = target
                .trades
                .iter()
                .map(|x| TestTradePayload::from_trade(x))
                .collect::<Vec<_>>();
            let last_trade = trades.last();
            snapshot.push(Some((last_trade.map(|x| *x), trades.clone())))
        }
        snapshot.assert(expected);
    }

    fn _test_trades_history(
        target: &mut Strategy,
        trades: &[Option<StrategySignal>],
        expected: &[Option<Vec<TestTradePayload>>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<Vec<TestTradePayload>>>::new();
        for _ in target.ctx.clone() {
            let tick = target.ctx.bar.index();
            let signal = trades[tick].unwrap_or(StrategySignal::Hold);

            target.next_bar();
            let output = target.next(signal);
            let trades = target
                .trades
                .iter()
                .map(|x| TestTradePayload::from_trade(x))
                .collect::<Vec<_>>();
            snapshot.push(Some(trades))
        }
        snapshot.assert(expected);
    }

    fn _test_equity(
        target: &mut Strategy,
        trades: &[Option<StrategySignal>],
        expected: &[Option<(f64, f64, f64)>],
    ) {
        let expected = expected
            .iter()
            .map(|x| x.as_ref().map(|(a, b, _)| (*a, *b)))
            .collect::<Vec<_>>();

        let mut snapshot = ArraySnapshot::<Option<(f64, f64)>>::new();
        for _ in target.ctx.clone() {
            let tick = target.ctx.bar.index();
            let signal = trades[tick].unwrap_or(StrategySignal::Hold);
            target.next_bar();
            target.next(signal);
            snapshot.push(Some((target.metrics.equity, target.metrics.open_profit)));
        }
        snapshot.assert(&expected);
    }

    #[test]
    fn empty_on_bar_close_continous() {
        let ctx = Context::new(
            InMemoryDataProvider::from_values(Vec::from([1.0, 2.0, 3.0, 4.0, 5.0])).to_arc(),
        );

        _test(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: true,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[None, None, None, None, None],
            &[
                Some((None, vec![])),
                Some((None, vec![])),
                Some((None, vec![])),
                Some((None, vec![])),
                Some((None, vec![])),
            ],
        );
    }

    #[test]
    fn trades_history_on_bar_close_continous() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0,
        ]))));

        _test_trades_history(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: true,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Short),
                // 3; Duplicated
                Some(StrategySignal::Short),
                // 4
                Some(StrategySignal::Long),
                // 5; Duplicated
                Some(StrategySignal::Long),
                // 6; Duplicated
                Some(StrategySignal::Long),
                // 7
                Some(StrategySignal::Short),
                // 8
                Some(StrategySignal::Long),
                // 9
                Some(StrategySignal::Short),
                // 10
                None,
                // 11; Duplicated
                Some(StrategySignal::Short),
                // 12
                None,
                // 13; Duplicated
                Some(StrategySignal::Short),
                // 14
                Some(StrategySignal::Long),
                // 15; Duplicated
                Some(StrategySignal::Long),
                // 16; Duplicated
                Some(StrategySignal::Long),
                // 17; Duplicated
                Some(StrategySignal::Long),
                // 18
                None,
                // 19
                Some(StrategySignal::Short),
                // 20
                None,
                // 21; Duplicated
                Some(StrategySignal::Short),
                // 22
                None,
                // 23; Duplicated
                Some(StrategySignal::Short),
                // 24
                Some(StrategySignal::Long),
                // 25; Duplicated
                Some(StrategySignal::Long),
                // 26
                None,
                // 27
                None,
                // 28; Duplicated
                Some(StrategySignal::Long),
            ],
            &[
                // 0
                Some(vec![]),
                // 1
                Some(vec![]),
                // 2
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 3
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 4
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 5
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 6
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 7
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 8
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 9
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 10
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 11
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 12
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 13
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 14
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 15
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 16
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 17
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 18
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 19
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 20
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 21
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 22
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 23
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 24
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(25.0),
                        entry_tick: Some(24),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 25
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(25.0),
                        entry_tick: Some(24),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 26
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(25.0),
                        entry_tick: Some(24),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 27
                // 24
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),

                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(25.0),
                        entry_tick: Some(24),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 28
                // 24
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(15.0),
                        entry_tick: Some(14),
                        exit_price: Some(20.0),
                        exit_tick: Some(19),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(20.0),
                        entry_tick: Some(19),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(25.0),
                        entry_tick: Some(24),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
            ],
        );
    }

    #[test]
    fn trades_history_next_bar_open_continous() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0,
        ]))));

        _test_trades_history(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: false,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Short),
                // 3; Duplicated
                Some(StrategySignal::Short),
                // 4
                Some(StrategySignal::Long),
                // 5
                None,
                // 6; Duplicated
                Some(StrategySignal::Long),
                // 7
                Some(StrategySignal::Short),
                // 8
                Some(StrategySignal::Long),
                // 9
                Some(StrategySignal::Short),
                // 10
                None,
                // 11; Duplicated
                Some(StrategySignal::Short),
                // 12
                None,
                // 13; Duplicated
                Some(StrategySignal::Short),
                // 14
                Some(StrategySignal::Long),
                // 15; Duplicated
                Some(StrategySignal::Long),
                // 16
                None,
                // 17
                None,
                // 18; Duplicated
                Some(StrategySignal::Long),
                // 19
                Some(StrategySignal::Short),
                // 20; Duplicated
                Some(StrategySignal::Short),
                // 21
                None,
                // 22
                None,
                // 23
                None,
                // 24
                Some(StrategySignal::Long),
                // 25
                None,
                // 26
                None,
                // 27
                None,
                // 28; Duplicated
                Some(StrategySignal::Long),
            ],
            &[
                // 0
                Some(vec![]),
                // 1
                Some(vec![]),
                // 2
                Some(vec![]),
                // 3
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 4
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 5
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 6
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 7
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 8
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 9
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 10
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 11
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 12
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 13
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 14
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 15
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 16
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 17
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 18
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 19
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 20
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(21.0),
                        entry_tick: Some(20),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 21
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(21.0),
                        entry_tick: Some(20),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 22
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(21.0),
                        entry_tick: Some(20),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 23
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(21.0),
                        entry_tick: Some(20),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 24
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(21.0),
                        entry_tick: Some(20),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 25
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(21.0),
                        entry_tick: Some(20),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 26
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(21.0),
                        entry_tick: Some(20),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 27
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(21.0),
                        entry_tick: Some(20),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 28
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(16.0),
                        entry_tick: Some(15),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(21.0),
                        entry_tick: Some(20),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
            ],
        );
    }

    #[test]
    fn trades_history_on_bar_close_intermittent() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0,
        ]))));

        _test_trades_history(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: true,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Short),
                // 3; Duplicated
                Some(StrategySignal::Short),
                // 4
                Some(StrategySignal::ShortExit),
                // 5
                None,
                // 6
                None,
                // 7
                Some(StrategySignal::Short),
                // 8
                Some(StrategySignal::ShortExit),
                // 9
                Some(StrategySignal::Short),
                // 10
                None,
                // 11; Duplicated
                Some(StrategySignal::Short),
                // 12; Duplicated
                Some(StrategySignal::Short),
                // 13; Duplicated
                Some(StrategySignal::Short),
                // 14
                Some(StrategySignal::ShortExit),
                // 15
                None,
                // 16
                None,
                // 17
                Some(StrategySignal::Long),
                // 18
                None,
                // 19; Duplicated
                Some(StrategySignal::Long),
                // 20
                None,
                // 21; Duplicated
                Some(StrategySignal::Long),
                // 22
                None,
                // 23; Duplicated
                Some(StrategySignal::Long),
                // 24
                Some(StrategySignal::LongExit),
                // 25
                None,
                // 26
                None,
                // 27
                Some(StrategySignal::Short),
                // 28
                None,
                // 29; Duplicated
                Some(StrategySignal::Short),
                // 30
                Some(StrategySignal::ShortExit),
                // 31
                None,
                // 32
                None,
                // 33
                Some(StrategySignal::Long),
                // 34
                None,
                // 35; Duplicated
                Some(StrategySignal::Long),
                // 36; Duplicated
                Some(StrategySignal::Long),
                // 37
                Some(StrategySignal::LongExit),
                // 38
                None,
                // 39
                None,
                // 40
                None,
            ],
            &[
                // 0
                Some(vec![]),
                // 1
                Some(vec![]),
                // 2
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 3
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 4
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: Some(5.0),
                    exit_tick: Some(4),
                }]),
                // 5
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: Some(5.0),
                    exit_tick: Some(4),
                }]),
                // 6
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: Some(5.0),
                    exit_tick: Some(4),
                }]),
                // 7
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 8
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                ]),
                // 9
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 10
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 11
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 12
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 13
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 14
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                ]),
                // 15
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                ]),
                // 16
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                ]),
                // 17
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 18
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 19
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 20
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 21
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 22
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 23
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 24
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                ]),
                // 25
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                ]),
                // 26
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                ]),
                // 27
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 28
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 29
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 30
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                ]),
                // 31
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                ]),
                // 32
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                ]),
                // 33
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(34.0),
                        entry_tick: Some(33),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 34
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(34.0),
                        entry_tick: Some(33),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 35
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(34.0),
                        entry_tick: Some(33),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 36
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(34.0),
                        entry_tick: Some(33),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 37
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(34.0),
                        entry_tick: Some(33),
                        exit_price: Some(38.0),
                        exit_tick: Some(37),
                    },
                ]),
                // 38
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(34.0),
                        entry_tick: Some(33),
                        exit_price: Some(38.0),
                        exit_tick: Some(37),
                    },
                ]),
                // 39
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(34.0),
                        entry_tick: Some(33),
                        exit_price: Some(38.0),
                        exit_tick: Some(37),
                    },
                ]),
                // 40
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(10.0),
                        entry_tick: Some(9),
                        exit_price: Some(15.0),
                        exit_tick: Some(14),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(18.0),
                        entry_tick: Some(17),
                        exit_price: Some(25.0),
                        exit_tick: Some(24),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(28.0),
                        entry_tick: Some(27),
                        exit_price: Some(31.0),
                        exit_tick: Some(30),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(34.0),
                        entry_tick: Some(33),
                        exit_price: Some(38.0),
                        exit_tick: Some(37),
                    },
                ]),
            ],
        );
    }

    #[test]
    fn trades_history_next_bar_open_intermittent() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0,
        ]))));

        _test_trades_history(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: false,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Short),
                // 3; Duplicated
                Some(StrategySignal::Short),
                // 4
                Some(StrategySignal::ShortExit),
                // 5
                None,
                // 6
                None,
                // 7
                Some(StrategySignal::Short),
                // 8
                Some(StrategySignal::ShortExit),
                // 9
                Some(StrategySignal::Short),
                // 10
                None,
                // 11; Duplicated
                Some(StrategySignal::Short),
                // 12
                None,
                // 13; Duplicated
                Some(StrategySignal::Short),
                // 14
                Some(StrategySignal::ShortExit),
                // 15
                None,
                // 16
                None,
                // 17
                Some(StrategySignal::Long),
                // 18; Duplicated
                Some(StrategySignal::Long),
                // 19
                None,
                // 20; Duplicated
                Some(StrategySignal::Long),
                // 21; Duplicated
                Some(StrategySignal::Long),
                // 22
                None,
                // 23; Duplicated
                Some(StrategySignal::Long),
                // 24
                Some(StrategySignal::LongExit),
                // 25
                None,
                // 26
                None,
                // 27
                Some(StrategySignal::Short),
                // 28; Duplicated
                Some(StrategySignal::Short),
                // 29; Duplicated
                Some(StrategySignal::Short),
                // 30
                Some(StrategySignal::ShortExit),
                // 31
                None,
                // 32
                None,
                // 33
                Some(StrategySignal::Long),
                // 34
                None,
                // 35; Duplicated
                Some(StrategySignal::Long),
                // 36
                None,
                // 37
                Some(StrategySignal::LongExit),
                // 38
                None,
                // 39
                None,
                // 40
                None,
            ],
            &[
                // 0
                Some(vec![]),
                // 1
                Some(vec![]),
                // 2
                Some(vec![]),
                // 3
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 4
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 5
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: Some(6.0),
                    exit_tick: Some(5),
                }]),
                // 6
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: Some(6.0),
                    exit_tick: Some(5),
                }]),
                // 7
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: Some(6.0),
                    exit_tick: Some(5),
                }]),
                // 8
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 9
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                ]),
                // 10
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 11
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 12
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 13
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 14
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 15
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                ]),
                // 16
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                ]),
                // 17
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                ]),
                // 18
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 19
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 20
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 21
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 22
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 23
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 24
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 25
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                ]),
                // 26
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                ]),
                // 27
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                ]),
                // 28
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 29
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 30
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 31
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                ]),
                // 32
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                ]),
                // 33
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                ]),
                // 34
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(35.0),
                        entry_tick: Some(34),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 35
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(35.0),
                        entry_tick: Some(34),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 36
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(35.0),
                        entry_tick: Some(34),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 37
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(35.0),
                        entry_tick: Some(34),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 38
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(35.0),
                        entry_tick: Some(34),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                ]),
                // 39
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(35.0),
                        entry_tick: Some(34),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                ]),
                // 40
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(29.0),
                        entry_tick: Some(28),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(35.0),
                        entry_tick: Some(34),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                ]),
            ],
        );
    }

    #[test]
    fn trades_history_next_bar_open_mixed() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0,
            45.0, 46.0, 47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0, 57.0, 58.0,
            59.0, 60.0, 61.0, 62.0,
        ]))));

        _test_trades_history(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: false,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Short),
                // 3; Duplicated
                Some(StrategySignal::ShortEntry),
                // 4
                Some(StrategySignal::ShortExit),
                // 5
                None,
                // 6
                None,
                // 7
                Some(StrategySignal::ShortEntry),
                // 8; Duplicated
                Some(StrategySignal::Short),
                // 9
                Some(StrategySignal::ShortExit),
                // 10
                None,
                // 11
                None,
                // 12
                Some(StrategySignal::Long),
                // 13; Duplicated
                Some(StrategySignal::LongEntry),
                // 14
                Some(StrategySignal::LongExit),
                // 15
                None,
                // 16
                None,
                // 17
                Some(StrategySignal::LongEntry),
                // 18; Duplicated
                Some(StrategySignal::Long),
                // 19
                Some(StrategySignal::LongExit),
                // 20
                None,
                // 21
                None,
                // 22
                Some(StrategySignal::Long),
                // 23
                None,
                // 24
                Some(StrategySignal::ShortEntry),
                // 25
                None,
                // 26
                Some(StrategySignal::ShortExit),
                // 27
                None,
                // 28
                Some(StrategySignal::Short),
                // 29
                None,
                // 30
                Some(StrategySignal::LongEntry),
                // 31
                None,
                // 32
                Some(StrategySignal::LongExit),
                // 33
                None,
                // 34
                None,
                // 35
                Some(StrategySignal::LongEntry),
                // 36
                None,
                // 37
                Some(StrategySignal::Short),
                // 38
                None,
                // 39
                Some(StrategySignal::ShortExit),
                // 40
                None,
                // 41
                None,
                // 42
                Some(StrategySignal::ShortEntry),
                // 43
                None,
                // 44
                Some(StrategySignal::Long),
                // 45
                None,
                // 46
                Some(StrategySignal::LongExit),
                // 47
                None,
                // 48
                None,
                // 49
                Some(StrategySignal::LongEntry),
                // 50
                None,
                // 51
                Some(StrategySignal::ShortEntry),
                // 52
                None,
                // 53
                Some(StrategySignal::Long),
                // 54
                None,
                // 55
                None,
                // 56
                Some(StrategySignal::ShortEntry),
                // 57
                None,
                // 58
                Some(StrategySignal::LongEntry),
                // 59
                None,
                // 60
                Some(StrategySignal::Short),
                // 61
                None,
            ],
            &[
                // 0
                Some(vec![]),
                // 1
                Some(vec![]),
                // 2
                Some(vec![]),
                // 3
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 4
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 5
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: Some(6.0),
                    exit_tick: Some(5),
                }]),
                // 6
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: Some(6.0),
                    exit_tick: Some(5),
                }]),
                // 7
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: Some(6.0),
                    exit_tick: Some(5),
                }]),
                // 8
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 9
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 10
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                ]),
                // 11
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                ]),
                // 12
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                ]),
                // 13
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 14
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 15
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                ]),
                // 16
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                ]),
                // 17
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                ]),
                // 18
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 19
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 20
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                ]),
                // 21
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                ]),
                // 22
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                ]),
                // 23
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 24
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 25
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 26
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 27
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                ]),
                // 28
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                ]),
                // 29
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 30
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 31
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 32
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 33
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                ]),
                // 34
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                ]),
                // 35
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                ]),
                // 36
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 37
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 38
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 39
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 40
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                ]),
                // 41
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                ]),
                // 42
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                ]),
                // 43
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 44
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 45
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 46
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 47
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                ]),
                // 48
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                ]),
                // 49
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                ]),
                // 50
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 51
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 52
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 53
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 54
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: Some(55.0),
                        exit_tick: Some(54),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(55.0),
                        entry_tick: Some(54),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 55
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: Some(55.0),
                        exit_tick: Some(54),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(55.0),
                        entry_tick: Some(54),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 56
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: Some(55.0),
                        exit_tick: Some(54),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(55.0),
                        entry_tick: Some(54),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 57
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: Some(55.0),
                        exit_tick: Some(54),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(55.0),
                        entry_tick: Some(54),
                        exit_price: Some(58.0),
                        exit_tick: Some(57),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(58.0),
                        entry_tick: Some(57),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 58
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: Some(55.0),
                        exit_tick: Some(54),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(55.0),
                        entry_tick: Some(54),
                        exit_price: Some(58.0),
                        exit_tick: Some(57),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(58.0),
                        entry_tick: Some(57),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 59
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: Some(55.0),
                        exit_tick: Some(54),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(55.0),
                        entry_tick: Some(54),
                        exit_price: Some(58.0),
                        exit_tick: Some(57),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(58.0),
                        entry_tick: Some(57),
                        exit_price: Some(60.0),
                        exit_tick: Some(59),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(60.0),
                        entry_tick: Some(59),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 60
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: Some(55.0),
                        exit_tick: Some(54),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(55.0),
                        entry_tick: Some(54),
                        exit_price: Some(58.0),
                        exit_tick: Some(57),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(58.0),
                        entry_tick: Some(57),
                        exit_price: Some(60.0),
                        exit_tick: Some(59),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: false,
                        entry_price: Some(60.0),
                        entry_tick: Some(59),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 61
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(14.0),
                        entry_tick: Some(13),
                        exit_price: Some(16.0),
                        exit_tick: Some(15),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(19.0),
                        entry_tick: Some(18),
                        exit_price: Some(21.0),
                        exit_tick: Some(20),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(24.0),
                        entry_tick: Some(23),
                        exit_price: Some(26.0),
                        exit_tick: Some(25),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(26.0),
                        entry_tick: Some(25),
                        exit_price: Some(28.0),
                        exit_tick: Some(27),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(30.0),
                        entry_tick: Some(29),
                        exit_price: Some(32.0),
                        exit_tick: Some(31),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(32.0),
                        entry_tick: Some(31),
                        exit_price: Some(34.0),
                        exit_tick: Some(33),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(37.0),
                        entry_tick: Some(36),
                        exit_price: Some(39.0),
                        exit_tick: Some(38),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(39.0),
                        entry_tick: Some(38),
                        exit_price: Some(41.0),
                        exit_tick: Some(40),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(44.0),
                        entry_tick: Some(43),
                        exit_price: Some(46.0),
                        exit_tick: Some(45),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(46.0),
                        entry_tick: Some(45),
                        exit_price: Some(48.0),
                        exit_tick: Some(47),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(51.0),
                        entry_tick: Some(50),
                        exit_price: Some(53.0),
                        exit_tick: Some(52),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(53.0),
                        entry_tick: Some(52),
                        exit_price: Some(55.0),
                        exit_tick: Some(54),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(55.0),
                        entry_tick: Some(54),
                        exit_price: Some(58.0),
                        exit_tick: Some(57),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(58.0),
                        entry_tick: Some(57),
                        exit_price: Some(60.0),
                        exit_tick: Some(59),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(60.0),
                        entry_tick: Some(59),
                        exit_price: Some(62.0),
                        exit_tick: Some(61),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(62.0),
                        entry_tick: Some(61),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
            ],
        );
    }

    #[test]
    fn trades_history_next_bar_open_edge_cases() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0,
        ]))));

        _test_trades_history(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: false,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Long),
                // 3; Duplicated
                Some(StrategySignal::Long),
                // 4; Duplicated
                Some(StrategySignal::Long),
                // 5
                None,
                // 6
                None,
            ],
            &[
                // 0
                Some(vec![]),
                // 1
                Some(vec![]),
                // 2
                Some(vec![]),
                // 3
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 4
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 5
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 6
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
            ],
        );
    }

    #[test]
    fn sized_trades_history_next_bar_open_edge_cases() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0,
        ]))));

        _test_trades_history(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: false,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Sized(1.0)),
                // 3; Duplicated
                Some(StrategySignal::Sized(1.0)),
                // 4
                None,
                // 5; Flip
                Some(StrategySignal::Sized(-1.0)),
                // 6
                None,
                // 7
                Some(StrategySignal::Sized(0.0)),
                // 8
                None,
                // 9
                None,
            ],
            &[
                // 0
                Some(vec![]),
                // 1
                Some(vec![]),
                // 2
                Some(vec![]),
                // 3
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 4
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 5
                Some(vec![TestTradePayload {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }]),
                // 6
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(7.0),
                        exit_tick: Some(6),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(7.0),
                        entry_tick: Some(6),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 7
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(7.0),
                        exit_tick: Some(6),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: false,
                        entry_price: Some(7.0),
                        entry_tick: Some(6),
                        exit_price: None,
                        exit_tick: None,
                    },
                ]),
                // 8
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(7.0),
                        exit_tick: Some(6),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(7.0),
                        entry_tick: Some(6),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                ]),
                // 9
                Some(vec![
                    TestTradePayload {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(7.0),
                        exit_tick: Some(6),
                    },
                    TestTradePayload {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(7.0),
                        entry_tick: Some(6),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                ]),
            ],
        );
    }

    // #[test]
    // fn dynamic_trades_history_next_bar_open_edge_cases() {
    //     let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
    //         1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0,
    //     ]))));

    //     _test_trades_history(
    //         &mut Strategy::new(
    //             ctx.clone(),
    //             StrategyConfig {
    //                 on_bar_close: false,
    //                 initial_capital: 1000.0,
    //                 buy_with_equity: true,
    //                 ..StrategyConfig::default()
    //             },
    //         ),
    //         &[
    //             // 0
    //             None,
    //             // 1
    //             None,
    //             // 2
    //             Some(StrategySignal::Dynamic(1.0)),
    //             // 3; Duplicated
    //             Some(StrategySignal::Dynamic(1.0)),
    //             // 4
    //             // None,
    //             // // 5; Flip
    //             // Some(StrategySignal::Sized(-1.0)),
    //             // // 6
    //             // None,
    //             // // 7
    //             // Some(StrategySignal::Sized(0.0)),
    //             // // 8
    //             // None,
    //             // // 9
    //             // None,
    //         ],
    //         &[
    //             // 0
    //             Some(vec![]),
    //             // 1
    //             Some(vec![]),
    //             // 2
    //             Some(vec![]),
    //             // 3
    //             Some(vec![TestTradePayload {
    //                 direction: TradeDirection::Long,
    //                 is_closed: false,
    //                 entry_price: Some(4.0),
    //                 entry_tick: Some(3),
    //                 exit_price: None,
    //                 exit_tick: None,
    //             }]),
    //             // 4
    //         ],
    //     );
    // }

    #[test]
    fn equity_empty_on_bar_close_continous() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0,
        ]))));

        _test_equity(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: true,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[None, None, None, None, None],
            &[
                Some((1000.0, 0.0, 0.0)),
                Some((1000.0, 0.0, 0.0)),
                Some((1000.0, 0.0, 0.0)),
                Some((1000.0, 0.0, 0.0)),
                Some((1000.0, 0.0, 0.0)),
            ],
        );
    }

    #[test]
    fn equity_on_bar_close_continous() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            // 0
            1.0,  // 1
            1.0,  // 2
            1.0,  // 3
            2.0,  // 4
            4.0,  // 5
            1.0,  // 6
            0.5,  // 7
            10.0, // 8
            12.0, // 9
            10.0, // 10
            6.0,  // 11
            14.0, // 12
            15.0, // 13
            18.0, // 14
            4.0,  // 15
            18.0, // 16
            20.0, // 17
            18.0, // 18
            6.0,  // 19
            1.0,  // 20
            8.0,  // 21
            9.0,  // 22
            10.0, // 23
            17.0, // 24
            11.0, // 25
            11.0, // 26
            15.0, // 27
            22.0, // 28
            6.0,  // 29
            5.0,  // 30
            7.0,  // 30
            1.0,
        ]))));

        _test_equity(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: true,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Long),
                // 3
                None,
                // 4; Duplicated
                Some(StrategySignal::Long),
                // 5
                None,
                // 6
                None,
                // 7
                None,
                // 8
                Some(StrategySignal::Short),
                // 9
                None,
                // 10; Duplicated
                Some(StrategySignal::Short),
                // 11; Duplicated
                Some(StrategySignal::Short),
                // 12
                None,
                // 13
                None,
                // 14
                None,
                // 15
                Some(StrategySignal::Long),
                // 16
                None,
                // 17
                None,
                // 18
                Some(StrategySignal::Short),
                // 19
                None,
                // 20
                Some(StrategySignal::Long),
                // 21
                Some(StrategySignal::Short),
                // 22
                Some(StrategySignal::Long),
                // 23
                Some(StrategySignal::Short),
                // 24
                Some(StrategySignal::Long),
                // 25
                None,
                // 26
                None,
                // 27; Duplicated
                Some(StrategySignal::Long),
                // 28; Duplicated
                Some(StrategySignal::Long),
                // 29
                Some(StrategySignal::Short),
                // 30; Duplicated
                Some(StrategySignal::Short),
                // 31; Duplicated
                Some(StrategySignal::Short),
            ],
            &[
                // 0
                Some((1000.0, 0.0, 0.0)),
                // 1
                Some((1000.0, 0.0, 0.0)),
                // 2;
                Some((1000.0, 0.0, 0.0)),
                // 3; pnl = (2.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((2000.0, 1000.0, 1.0)),
                // 4; pnl = (4.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((4000.0, 3000.0, 1.0)),
                // 5; pnl = (1.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((1000.0, 0.0, -0.75)),
                // 6; pnl = (0.5 - 1.0) * (1000.0 / 1.0) * 1
                Some((500.0, -500.0, -0.5)),
                // 7; pnl = (10.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((10000.0, 9000.0, 19.0)),
                // 8; pnl = (12.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((12000.0, 0.0, 0.2)),
                // 9; pnl = (10.0 - 12.0) * (12000.0 / 12.0 ~ 1000) * -1
                Some((14000.0, 2000.0, 0.16666)),
                // 10; pnl = (6.0 - 12.0) * (12000.0 / 12.0 ~ 1000) * -1
                Some((18000.0, 6000.0, 0.2857142)),
                // 11; pnl = (14.0 - 12.0) * (12000.0 / 12.0 ~ 1000) * -1
                Some((10000.0, -2000.0, -0.44444)),
                // 12; pnl = (15.0 - 12.0) * (12000.0 / 12.0 ~ 1000) * -1
                Some((9000.0, -3000.0, -0.1)),
                // 13; pnl = (18.0 - 12.0) * (12000.0 / 12.0 ~ 1000) * -1
                Some((6000.0, -6000.0, -0.333333)),
                // 14; pnl = (4.0 - 12.0) * (12000.0 / 12.0 ~ 1000) * -1
                Some((20000.0, 8000.0, 2.3333333)),
                // 15; pnl = (18.0 - 12.0) * (12000.0 / 12.0 ~ 1000) * -1
                Some((6000.0, 0.0, -0.7)),
                // 16; pnl = (20.0 - 18.0) * (6000.0 / 18.0) * 1
                Some((6666.666666, 666.666666, 0.11111111)),
                // 17; pnl = (18.0 - 18.0) * (6000.0 / 18.0) * 1
                Some((6000.0, 0.0, -0.099999)),
                // 18; pnl = (6.0 - 18.0) * (6000.0 / 18.0) * 1
                Some((2000.0, 0.0, -0.6666666)),
                // 19; pnl = (1.0 - 6.0) * (2000.0 / 6.0) * -1
                Some((3666.6666666, 1666.66666, 0.8333333)),
                // 20; pnl = (8.0 - 6.0) * (2000.0 / 6.0) * -1
                Some((1333.33334, 0.0, -0.6363636)),
                // 21; pnl = (9.0 - 8.0) * (1333.33334 / 8.0) * 1
                Some((1500.0, 0.0, 0.12499)),
                // 22; pnl = (10.0 - 9.0) * (1500.0 / 9.0) * -1
                Some((1333.333333, 0.0, -0.1111111)),
                // 23; pnl = (17.0 - 10.0) * (1333.333333 / 10.0) * 1
                Some((2266.66666, 0.0, 0.7)),
                // 24; pnl = (11.0 - 17.0) * (2266.66666 / 17.0) * 1
                Some((3066.66666, 0.0, 0.352941176)),
                // 25; pnl = (11.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((3066.66666, 0.0, 0.0)),
                // 26; pnl = (15.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((4181.8181727, 1115.15151, 0.36363636)),
                // 27; pnl = (22.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((6133.3333333, 3066.666666, 0.466666)),
                // 28; pnl = (6.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((1672.72727, -1393.939390, -0.727272727)),
                // 29; pnl = (5.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((1393.939391, 0.0, -0.16666666)),
                // 30; pnl = (7.0 - 5.0) * (1393.939391 / 5.0) * -1
                Some((836.3636363636364, -557.5757564, -0.4)),
                // 31; pnl = (17.0 - 5.0) * (1393.939391 / 5.0) * -1
                Some((2509.0909038, 1115.1515128, 2.0)),
            ],
        );
    }

    #[test]
    fn equity_on_next_bar_open_continous() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            // 0
            1.0,  // 1
            1.0,  // 2
            1.0,  // 3
            2.0,  // 4
            4.0,  // 5
            1.0,  // 6
            0.5,  // 7
            10.0, // 8
            12.0, // 9
            10.0, // 10
            6.0,  // 11
            14.0, // 12
            15.0, // 13
            18.0, // 14
            4.0,  // 15
            18.0, // 16
            19.0, // 17
            18.0, // 18
            6.0,  // 19
            1.0,  // 20
            0.02, // 21
            0.01, // 22
            10.0, // 23
            17.0, // 24
            11.0, // 25
            11.0, // 26
            15.0, // 27
            22.0, // 28
            6.0,  // 29
            5.0,  // 30
            7.0,  // 31
            1.0,  // 32
            11.0,
        ]))));

        _test_equity(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: false,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Long),
                // 3
                None,
                // 4
                None,
                // 5
                None,
                // 6
                None,
                // 7
                None,
                // 8
                Some(StrategySignal::Short),
                // 9
                None,
                // 10
                None,
                // 11
                None,
                // 12
                None,
                // 13
                None,
                // 14
                None,
                // 15
                Some(StrategySignal::Long),
                // 16
                None,
                // 17
                None,
                // 18
                Some(StrategySignal::Short),
                // 19
                None,
                // 20
                Some(StrategySignal::Long),
                // 21
                Some(StrategySignal::Short),
                // 22
                Some(StrategySignal::Long),
                // 23
                Some(StrategySignal::Short),
                // 24
                Some(StrategySignal::Long),
                // 25
                None,
                // 26
                None,
                // 27; Duplicated
                Some(StrategySignal::Long),
                // 28; Duplicated
                Some(StrategySignal::Long),
                // 29
                Some(StrategySignal::Short),
                // 30; Duplicated
                Some(StrategySignal::Short),
                // 31; Duplicated
                Some(StrategySignal::Short),
                // 32; Duplicated
                Some(StrategySignal::Short),
            ],
            &[
                // 0
                Some((1000.0, 0.0, 0.0)),
                // 1
                Some((1000.0, 0.0, 0.0)),
                // 2;
                Some((1000.0, 0.0, 0.0)),
                // 3;
                Some((1000.0, 0.0, 0.0)),
                // 4; pnl = (4.0 - 2.0) * (1000.0 / 2.0) * 1
                Some((2000.0, 1000.0, 1.0)),
                // 5; pnl = (1.0 - 2.0) * (1000.0 / 2.0) * 1
                Some((500.0, -500.0, -0.75)),
                // 6; pnl = (0.5 - 2.0) * (1000.0 / 2.0) * 1
                Some((250.0, -750.0, -0.50)),
                // 7; pnl = (10.0 - 2.0) * (1000.0 / 2.0) * 1
                Some((5000.0, 4000.0, 19.0)),
                // 8; pnl = (12.0 - 2.0) * (1000.0 / 2.0) * 1
                Some((6000.0, 5000.0, 0.2)),
                // 9; pnl = (10.0 - 2.0) * (1000.0 / 2.0) * 1
                Some((5000.0, 0.0, -0.16666666)),
                // 10; pnl = (6.0 - 10.0) * (5000.0 / 10) * -1
                Some((7000.0, 2000.0, 0.4)),
                // 11; pnl = (14.0 - 10.0) * (5000.0 / 10) * -1
                Some((3000.0, -2000.0, -0.5714285)),
                // 12; pnl = (15.0 - 10.0) * (5000.0 / 10) * -1
                Some((2500.0, -2500.0, -0.16666666666666663)),
                // 13; pnl = (18.0 - 10.0) * (5000.0 / 10) * -1
                Some((1000.0, -4000.0, -0.6)),
                // 14; pnl = (4.0 - 10.0) * (5000.0 / 10) * -1
                Some((8000.0, 3000.0, 7.0)),
                // 15; pnl = (18.0 - 10.0) * (5000.0 / 10) * -1
                Some((1000.0, -4000.0, -0.875)),
                // 16; pnl = (19.0 - 10.0) * (5000.0 / 10) * -1
                Some((500.0, 0.0, -0.5)),
                // 17; pnl = (18.0 - 19.0) * (500.0 / 19) * 1
                Some((473.6842106, -26.3157894, -0.0526315788)),
                // 18; pnl = (6.0 - 19.0) * (500.0 / 19) * 1
                Some((157.8947369, -342.1052631, -0.666666666)),
                // 19; pnl = (1.0 - 19.0) * (500.0 / 19) * 1
                Some((26.31579, 0.0, -0.833333330)),
                // 20; pnl = (0.02 - 1) * (26.31579 / 1) * -1
                Some((52.1052631, 25.78947, 0.98)),
                // 21; pnl = (0.01 - 1) * (26.31579 / 1) * -1
                Some((52.3684210526316, 0.0, 0.005050505050504972)),
                // 22; pnl = (10.0 - 0.01) * (52.3684210526316 / 0.01) * -1
                Some((52368.421052552, 0.0, 999.0)),
                // 23; pnl = (10.0 - 17) * (52368.421052552 / 10) * -1
                Some((15710.52631578948, 0.0, -0.7)),
                // 24; pnl = (11.0 - 17) * (15710.52631578948 / 17) * -1
                Some((10165.634674922605, 0.0, -0.3529411764705882)),
                // 25; pnl = (11.0 - 11) * (10165.634674922605 / 11) * 1
                Some((10165.634674922605, 0.0, 0.0)),
                // 26; pnl = (15.0 - 11) * (10165.634674922605 / 11) * 1
                Some((13862.229102167188, 3696.5944272445836, 0.36363636363636354)),
                // 27; pnl = (22.0 - 11) * (10165.634674922605 / 11) * 1
                Some((20331.26934984521, 10165.634674922605, 0.466666)),
                // 28; pnl = (6.0 - 11) * (10165.634674922605 / 11) * 1
                Some((5544.89164086687, -4620.743034055729545, -0.72727272)),
                // 29; pnl = (5.0 - 11) * (10165.634674922605 / 11) * 1
                Some((4620.7430340, -5544.8916408668, -0.166666666)),
                // 30; pnl = (7.0 - 11) * (10165.634674922605 / 11) * 1
                Some((6469.040247678022, 0.0, 0.40000000000000013)),
                // 31; pnl = (1.0 - 7.0) * (6469.040247678022 / 7) * -1
                Some((12013.93188854, 5544.891640866876, 0.85714285714209)),
                // 32; pnl = (11.0 - 7.0) * (6469.040247678022 / 7) * -1
                Some((2772.445820433438, -3696.594427244584, -0.7692307692306)),
            ],
        );
    }

    #[test]
    fn equity_on_bar_close_intermittent() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            // 0
            1.0,   // 1
            1.0,   // 2
            1.0,   // 3
            2.0,   // 4
            4.0,   // 5
            1.0,   // 6
            0.5,   // 7
            10.0,  // 8
            12.0,  // 9
            10.0,  // 10
            6.0,   // 11
            14.0,  // 12
            15.0,  // 13
            18.0,  // 14
            4.0,   // 15
            18.0,  // 16
            19.0,  // 17
            18.0,  // 18
            6.0,   // 19
            1.0,   // 20
            0.02,  // 21
            0.01,  // 22
            10.0,  // 23
            17.0,  // 24
            11.0,  // 25
            11.0,  // 26
            15.0,  // 27
            34.0,  // 28
            6.0,   // 29
            5.0,   // 30
            7.0,   // 31
            1.0,   // 32
            11.0,  // 33
            500.0, // 34
            -50.0, // 35
            11.0,  // 36
            11.0,  // 37
            11.0,  // 38
            57.0,  // 39
            11.0,  // 40
            5.0,   // 41
            2.0,   // 42
            61.0,  // 43
            57.0,  // 44
            30.0,  // 45
            6.0,   // 46
            8.0,   // 47
            5.0,   // 48
            10.0,  // 49
            8.0,   // 50
            12.0,  // 51
            16.0,
        ]))));

        _test_equity(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: true,
                    buy_with_equity: true,
                    initial_capital: 1000.0,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Long),
                // 3
                None,
                // 4; Duplicated
                Some(StrategySignal::Long),
                // 5
                None,
                // 6
                None,
                // 7; Duplicated
                Some(StrategySignal::Long),
                // 8
                Some(StrategySignal::LongExit),
                // 9
                None,
                // 10
                None,
                // 11
                None,
                // 12
                None,
                // 13
                None,
                // 14
                None,
                // 15
                Some(StrategySignal::Long),
                // 16; Duplicated
                Some(StrategySignal::Long),
                // 17
                None,
                // 18
                Some(StrategySignal::LongExit),
                // 19
                None,
                // 20
                Some(StrategySignal::Long),
                // 21
                Some(StrategySignal::LongExit),
                // 22
                Some(StrategySignal::Long),
                // 23
                Some(StrategySignal::LongExit),
                // 24
                None,
                // 25
                None,
                // 26
                None,
                // 27
                Some(StrategySignal::Short),
                // 28
                None,
                // 29; Duplicated
                Some(StrategySignal::Short),
                // 30
                None,
                // 31; Duplicated
                Some(StrategySignal::Short),
                // 32
                Some(StrategySignal::ShortExit),
                // 33
                None,
                // 34
                None,
                // 35
                None,
                // 36
                None,
                // 37
                None,
                // 38
                Some(StrategySignal::Short),
                // 39; Duplicated
                Some(StrategySignal::Short),
                // 40
                None,
                // 41; Duplicated
                Some(StrategySignal::Short),
                // 42
                None,
                // 43
                None,
                // 44; Duplicated
                Some(StrategySignal::Short),
                // 45
                Some(StrategySignal::ShortExit),
                // 46
                None,
                // 47
                None,
                // 48
                Some(StrategySignal::Short),
                // 49
                Some(StrategySignal::ShortExit),
                // 50
                Some(StrategySignal::Short),
                // 51
                Some(StrategySignal::ShortExit),
            ],
            &[
                // 0
                Some((1000.0, 0.0, 0.0)),
                // 1
                Some((1000.0, 0.0, 0.0)),
                // 2;
                Some((1000.0, 0.0, 0.0)),
                // 3; pnl = (2.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((2000.0, 1000.0, 1.0)),
                // 4; pnl = (4.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((4000.0, 3000.0, 1.0)),
                // 5; pnl = (1.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((1000.0, 0.0, -0.75)),
                // 6; pnl = (0.5 - 1.0) * (1000.0 / 1.0) * 1
                Some((500.0, -500.0, -0.5)),
                // 7; pnl = (10.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((10000.0, 9000.0, 19.0)),
                // 8; pnl = (12.0 - 1.0) * (1000.0 / 1.0) * 1
                Some((12000.0, 0.0, 0.2)),
                // 9; no trade
                Some((12000.0, 0.0, 0.0)),
                // 10; no trade
                Some((12000.0, 0.0, 0.0)),
                // 11; no trade
                Some((12000.0, 0.0, 0.0)),
                // 12; no trade
                Some((12000.0, 0.0, 0.0)),
                // 13; no trade
                Some((12000.0, 0.0, 0.0)),
                // 14; no trade
                Some((12000.0, 0.0, 0.0)),
                // 15
                Some((12000.0, 0.0, 0.0)),
                // 16; pnl = (19.0 - 18.0) * (12000.0 / 18.0) * 1
                Some((12666.66666666, 666.6666666, 0.0555555555)),
                // 17; pnl = (18.0 - 18.0) * (12000.0 / 18.0) * 1
                Some((12000.0, 0.0, -0.0526315789)),
                // 18; pnl = (6.0 - 18.0) * (12000.0 / 18.0) * 1
                Some((4000.0, 0.0, -0.66666666)),
                // 19; no trade
                Some((4000.0, 0.0, 0.0)),
                // 20; no trade
                Some((4000.0, 0.0, 0.0)),
                // 21; pnl = (0.01 - 0.02) * (4000.0 / 0.02) * 1
                Some((2000.0, 0.0, -0.5)),
                // 22; no trade
                Some((2000.0, -0.0, 0.0)),
                // 23; pnl = (17 - 10) * (2000.0 / 10) * 1
                Some((3400.0, 0.0, 0.7)),
                // 24; no trade
                Some((3400.0, 0.0, 0.0)),
                // 25; no trade
                Some((3400.0, 0.0, 0.0)),
                // 26; no trade
                Some((3400.0, 0.0, 0.0)),
                // 27; no trade
                Some((3400.0, 0.0, 0.0)),
                // 28; pnl = (6 - 34) * (3400.0 / 34) * -1
                Some((6200.0, 2800.0, 0.8235294117)),
                // 29; pnl = (5 - 34) * (3400.0 / 34) * -1
                Some((6300.0, 2900.0, 0.0161290322)),
                // 30; pnl = (7 - 34) * (3400.0 / 34) * -1
                Some((6100.0, 2700.0, -0.031746)),
                // 31; pnl = (1 - 34) * (3400.0 / 34) * -1
                Some((6700.0, 3300.0, 0.0983606)),
                // 32; pnl = (11 - 34) * (3400.0 / 34) * -1
                Some((5700.0, 0.0, -0.14925373)),
                // 33; no trade
                Some((5700.0, 0.0, 0.0)),
                // 34; no trade
                Some((5700.0, 0.0, 0.0)),
                // 35; no trade
                Some((5700.0, 0.0, 0.0)),
                // 36; no trade
                Some((5700.0, 0.0, 0.0)),
                // 37; no trade
                Some((5700.0, 0.0, 0.0)),
                // 38; no trade
                Some((5700.0, 0.0, 0.0)),
                // 39; pnl = (11 - 57) * (5700.0 / 57) * -1
                Some((10300.0, 4600.0, 0.807017543)),
                // 40; pnl = (5 - 57) * (5700.0 / 57) * -1
                Some((10900.0, 5200.0, 0.05825242718)),
                // 41; pnl = (2 - 57) * (5700.0 / 57) * -1
                Some((11200.0, 5500.0, 0.02752293577)),
                // 42; pnl = (61 - 57) * (5700.0 / 57) * -1
                Some((5300.0, -400.0, -0.526785714)),
                // 43; pnl = (57 - 57) * (5700.0 / 57) * -1
                Some((5700.0, 0.0, 0.075471698113)),
                // 44; pnl = (30 - 57) * (5700.0 / 57) * -1
                Some((8400.0, 2700.0, 0.4736842105)),
                // 45; pnl = (6 - 57) * (5700.0 / 57) * -1
                Some((10800.0, 0.0, 0.28571428571)),
                // 46; no trade
                Some((10800.0, 0.0, 0.0)),
                // 47; no trade
                Some((10800.0, 0.0, 0.0)),
                // 48; no trade
                Some((10800.0, 0.0, 0.0)),
                // 49; pnl = (8 - 10) * (10800.0 / 10) * -1
                Some((12960.0, 0.0, 0.2)),
                // 50; no trade
                Some((12960.0, 0.0, 0.0)),
                // 51; pnl = (16 - 12) * (12960.0 / 12) * -1
                Some((8640.0, 0.0, -0.33333333333333337)),
            ],
        );
    }

    #[test]
    fn equity_next_bar_open_intermittent() {
        let ctx = Context::new(Arc::from(InMemoryDataProvider::from_values(Vec::from([
            // 0
            1.0,   // 1
            1.0,   // 2
            1.0,   // 3
            2.0,   // 4
            4.0,   // 5
            1.0,   // 6
            0.5,   // 7
            10.0,  // 8
            12.0,  // 9
            10.0,  // 10
            6.0,   // 11
            14.0,  // 12
            15.0,  // 13
            18.0,  // 14
            4.0,   // 15
            18.0,  // 16
            5.0,   // 17
            10.0,  // 18
            6.0,   // 19
            1.0,   // 20
            2.0,   // 21
            5.0,   // 22
            10.0,  // 23
            20.0,  // 24
            10.0,  // 25
            8.0,   // 26
            15.0,  // 27
            34.0,  // 28
            10.0,  // 29
            5.0,   // 30
            7.0,   // 31
            1.0,   // 32
            12.0,  // 33
            4.0,   // 34
            -50.0, // 35
            11.0,  // 36
            11.0,  // 37
            11.0,  // 38
            57.0,  // 39
            8.0,   // 40
            6.0,   // 41
            2.0,   // 42
            12.0,  // 43
            8.0,   // 44
            6.0,   // 45
            4.0,   // 46
            10.0,  // 47
            5.0,   // 48
            10.0,  // 49
            6.0,   // 50
            8.0,   // 51
            2.0,
        ]))));

        _test_equity(
            &mut Strategy::new(
                ctx.clone(),
                StrategyConfig {
                    on_bar_close: false,
                    initial_capital: 1000.0,
                    buy_with_equity: true,
                    ..StrategyConfig::default()
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(StrategySignal::Long),
                // 3
                None,
                // 4; Duplicated
                Some(StrategySignal::Long),
                // 5
                None,
                // 6
                None,
                // 7; Duplicated
                Some(StrategySignal::Long),
                // 8
                Some(StrategySignal::LongExit),
                // 9
                None,
                // 10
                None,
                // 11
                None,
                // 12
                None,
                // 13
                None,
                // 14
                None,
                // 15
                Some(StrategySignal::Long),
                // 16; Duplicated
                Some(StrategySignal::Long),
                // 17
                None,
                // 18
                Some(StrategySignal::LongExit),
                // 19
                None,
                // 20
                Some(StrategySignal::Long),
                // 21; long entry
                Some(StrategySignal::LongExit),
                // 22; long exit
                Some(StrategySignal::Long),
                // 23; long entry
                Some(StrategySignal::LongExit),
                // 24; long exit
                None,
                // 25
                None,
                // 26
                None,
                // 27
                Some(StrategySignal::Short),
                // 28
                None,
                // 29; Duplicated
                Some(StrategySignal::Short),
                // 30
                None,
                // 31; Duplicated
                Some(StrategySignal::Short),
                // 32
                Some(StrategySignal::ShortExit),
                // 33
                None,
                // 34
                None,
                // 35
                None,
                // 36
                None,
                // 37
                None,
                // 38
                Some(StrategySignal::Short),
                // 39; Duplicated
                Some(StrategySignal::Short),
                // 40
                None,
                // 41; Duplicated
                Some(StrategySignal::Short),
                // 42
                None,
                // 43
                None,
                // 44; Duplicated
                Some(StrategySignal::Short),
                // 45
                Some(StrategySignal::ShortExit),
                // 46
                None,
                // 47
                None,
                // 48
                Some(StrategySignal::Short),
                // 49; short entry
                Some(StrategySignal::ShortExit),
                // 50; short exit
                Some(StrategySignal::Short),
                // 51; short entry; no trades
                Some(StrategySignal::ShortExit),
            ],
            &[
                // 0
                Some((1000.0, 0.0, 0.0)),
                // 1
                Some((1000.0, 0.0, 0.0)),
                // 2;
                Some((1000.0, 0.0, 0.0)),
                // 3;
                Some((1000.0, 0.0, 0.0)),
                // 4; pnl = (4.0 - 2.0) * (1000.0 / 2) * 1
                Some((2000.0, 1000.0, 1.0)),
                // 5; pnl = (1.0 - 2.0) * (1000.0 / 2) * 1
                Some((500.0, -500.0, -0.75)),
                // 6; pnl = (0.5 - 2.0) * (1000.0 / 2) * 1
                Some((250.0, -750.0, -0.5)),
                // 7; pnl = (10 - 2.0) * (1000.0 / 2) * 1
                Some((5000.0, 4000.0, 19.0)),
                // 8; pnl = (12 - 2.0) * (1000.0 / 2) * 1
                Some((6000.0, 5000.0, 0.2)),
                // 9; pnl = (10 - 2.0) * (1000.0 / 2) * 1
                Some((5000.0, 0.0, -0.16666666666666663)),
                // 10; no trade
                Some((5000.0, 0.0, 0.0)),
                // 11; no trade
                Some((5000.0, 0.0, 0.0)),
                // 12; no trade
                Some((5000.0, 0.0, 0.0)),
                // 13; no trade
                Some((5000.0, 0.0, 0.0)),
                // 14; no trade
                Some((5000.0, 0.0, 0.0)),
                // 15; no trade
                Some((5000.0, 0.0, 0.0)),
                // 16; no trade
                Some((5000.0, 0.0, 0.0)),
                // 17; pnl = (10.0 - 5.0) * (5000.0 / 5) * 1
                Some((10000.0, 5000.0, 1.0)),
                // 18; pnl = (6.0 - 5.0) * (5000.0 / 5) * 1
                Some((6000.0, 1000.0, -0.4)),
                // 19; pnl = (1.0 - 5.0) * (5000.0 / 5) * 1
                Some((1000.0, 0.0, -0.83333333333)),
                // 20; no trade
                Some((1000.0, 0.0, 0.0)),
                // 21; no trade
                Some((1000.0, 0.0, 0.0)),
                // 22; pnl = (10.0 - 5) * (1000.0 / 5) * 1
                Some((2000.0, 0.0, 1.0)),
                // 23; no trade
                Some((2000.0, 0.0, 0.0)),
                // 24; pnl = (10.0 - 20) * (2000.0 / 20) * 1
                Some((1000.0, 0.0, -0.5)),
                // 25; no trades
                Some((1000.0, 0.0, 0.0)),
                // 26; no trades
                Some((1000.0, 0.0, 0.0)),
                // 27; no trades
                Some((1000.0, 0.0, 0.0)),
                // 28; no trades
                Some((1000.0, 0.0, 0.0)),
                // 29; pnl = (5.0 - 10) * (1000.0 / 10) * -1
                Some((1500.0, 500.0, 0.5)),
                // 30; pnl = (7.0 - 10) * (1000.0 / 10) * -1
                Some((1300.0, 300.0, -0.1333333333)),
                // 31; pnl = (1.0 - 10) * (1000.0 / 10) * -1
                Some((1900.0, 900.0, 0.46153846)),
                // 32; pnl = (12.0 - 10) * (1000.0 / 10) * -1
                Some((800.0, -200.0, -0.578947368)),
                // 33; pnl = (4.0 - 10) * (1000.0 / 10) * -1
                Some((1600.0, 0.0, 1.0)),
                // 34; no trades
                Some((1600.0, 0.0, 0.0)),
                // 35; no trades
                Some((1600.0, 0.0, 0.0)),
                // 36; no trades
                Some((1600.0, 0.0, 0.0)),
                // 37; no trades
                Some((1600.0, 0.0, 0.0)),
                // 38; no trades
                Some((1600.0, 0.0, 0.0)),
                // 39; no trades
                Some((1600.0, 0.0, 0.0)),
                // 40; pnl = (6.0 - 8) * (1600.0 / 8) * -1
                Some((2000.0, 400.0, 0.25)),
                // 41; pnl = (2.0 - 8) * (1600.0 / 8) * -1
                Some((2800.0, 1200.0, 0.4)),
                // 42; pnl = (12 - 8) * (1600.0 / 8) * -1
                Some((800.0, -800.0, -0.714285714)),
                // 43; pnl = (8 - 8) * (1600.0 / 8) * -1
                Some((1600.0, 0.0, 1.0)),
                // 44; pnl = (6 - 8) * (1600.0 / 8) * -1
                Some((2000.0, 400.0, 0.25)),
                // 45; pnl = (4 - 8) * (1600.0 / 8) * -1
                Some((2400.0, 800.0, 0.2)),
                // 46; pnl = (10 - 8) * (1600.0 / 8) * -1
                Some((1200.0, 0.0, -0.5)),
                // 47; no trades
                Some((1200.0, 0.0, 0.0)),
                // 48; no trades
                Some((1200.0, 0.0, 0.0)),
                // 49; no trades
                Some((1200.0, 0.0, 0.0)),
                // 50; pnl = (8 - 6) * (1200.0 / 6) * -1
                Some((800.0, 0.0, -0.33333333)),
                // 51; no trades
                Some((800.0, 0.0, 0.0)),
            ],
        );
    }

    #[derive(Clone, Debug)]
    struct TestExecutionPayload {
        pub open_profit: f64,
        pub net_profit: f64,
        pub position_size: f64,
    }

    impl ArraySnapshot<TestExecutionPayload> {
        pub fn assert(&self, expected: &[TestExecutionPayload]) {
            self.assert_iter(expected, |actual, expected| {
                return actual.open_profit.compare(expected.open_profit)
                    && actual.net_profit.compare(expected.net_profit)
                    && actual.position_size.compare(expected.position_size);
            });
        }
    }

    struct TestExecutionTarget {
        pub ctx: Context,
        pub long_entries: Vec<usize>,
        pub long_exits: Vec<usize>,
        pub short_entries: Vec<usize>,
        pub short_exits: Vec<usize>,
        strategy: Strategy,
    }

    impl TestExecutionTarget {
        pub fn new(
            ctx: Context,
            strategy_config: StrategyConfig,
            long_entries: Vec<usize>,
            long_exits: Vec<usize>,
            short_entries: Vec<usize>,
            short_exits: Vec<usize>,
        ) -> Self {
            let strategy = Strategy::new(ctx.clone(), strategy_config);
            return Self {
                ctx: ctx.clone(),
                strategy,
                long_entries,
                short_entries,
                long_exits,
                short_exits,
            };
        }

        pub fn next(&mut self) -> TestExecutionPayload {
            let tick = self.ctx.bar.index();

            let mut trade_direction: Option<TradeDirection> = None;

            if self.long_entries.contains(&tick) || self.short_exits.contains(&tick) {
                trade_direction = Some(TradeDirection::Long);
            } else if self.short_entries.contains(&tick) || self.long_exits.contains(&tick) {
                trade_direction = Some(TradeDirection::Short);
            }

            let signal: StrategySignal = match trade_direction {
                Some(TradeDirection::Long) => StrategySignal::Long,
                Some(TradeDirection::Short) => StrategySignal::Short,
                None => StrategySignal::Hold,
            };

            let initial_capital = self.strategy.config.initial_capital;
            self.strategy.next_bar();

            let res = TestExecutionPayload {
                open_profit: self.strategy.metrics.open_profit,
                net_profit: self.strategy.metrics.net_profit,
                position_size: self.strategy.metrics.position_size,
            };
            self.strategy.next(signal);

            return res;
        }
    }

    fn _test_execution(target: &mut TestExecutionTarget, expected: &[TestExecutionPayload]) {
        let mut snapshot = ArraySnapshot::<TestExecutionPayload>::new();
        for _ in target.ctx.clone() {
            let payload = target.next();
            snapshot.push(payload);
        }
        snapshot.assert(expected);
    }

    fn _load_execution_metrics(df: &DataFrame) -> Vec<TestExecutionPayload> {
        let mut list: Vec<TestExecutionPayload> = vec![];

        let open_profit = df.column("_target_open_profit_").unwrap().to_f64();
        let net_profit = df.column("_target_net_profit_").unwrap().to_f64();
        let position_size = df.column("_target_position_size_").unwrap().to_f64();

        for i in 0..open_profit.len() {
            let p = TestExecutionPayload {
                open_profit: open_profit[i].ps_nz(),
                net_profit: net_profit[i].ps_nz(),
                position_size: position_size[i].ps_nz(),
            };
            list.push(p);
        }

        return list;
    }

    // #[test]
    // fn on_next_bar_open_intermittent_extensive() {
    //     let (df, ctx) = Fixture::load(&format_path("default.csv"));
    //     let expected = _load_execution_metrics(&df);

    //     let mut long_entries: Vec<usize> = vec![];
    //     let mut long_exits: Vec<usize> = vec![];
    //     let mut short_entries: Vec<usize> = vec![];
    //     let mut short_exits: Vec<usize> = vec![];

    //     let signal_column = df.column("_target_signal_").unwrap().to_f64();

    //     for i in 0..expected.len() {
    //         let signal = signal_column[i].ps_nz();
    //         if signal.compare(1.0) {
    //             long_entries.push(i);
    //         } else if signal.compare(-1.0) {
    //             short_entries.push(i);
    //         } else if signal.compare(2.0) {
    //             long_exits.push(i);
    //         } else if signal.compare(-2.0) {
    //             short_exits.push(i);
    //         }
    //     }

    //     _test_execution(
    //         &mut TestExecutionTarget::new(
    //             ctx.clone(),
    //             StrategyConfig {
    //
    //                 on_bar_close: false,
    //                 initial_capital: 1000.0,
    //                 buy_with_equity: false,
    //                 ..StrategyConfig::default()
    //             },
    //             long_entries,
    //             long_exits,
    //             short_entries,
    //             short_exits,
    //         ),
    //         &expected,
    //     );
    // }
}
