#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::Arc,
    };

    use polars::prelude::DataFrame;

    use crate::{
        core::{
            context::Context,
            data_provider::DataProvider,
            in_memory_data_provider::InMemoryDataProvider,
            incremental::{Incremental, IncrementalDefault},
        },
        pinescript::common::PineScriptFloat64,
        polars::series::SeriesCastUtils,
        strategy_refactor::{
            common::{Qty, Signal, SignalOptions},
            strategy::{Strategy, StrategyConfig},
            trade::{Trade, TradeDirection},
        },
        testing::{
            array_snapshot::{ArraySnapshot, Compare},
            fixture::Fixture,
            pace::format_pace_fixture_path,
        },
        utils::float::{Float64Utils, OptionFloatUtils},
    };
    use serde::{Deserialize, Serialize};

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/strategy/backtest/{}", path))
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestFixture {
        pub name: String,
        pub config: BacktestFixtureConfig,
        pub signals: Vec<BacktestSignal>,
        pub trades: Vec<BacktestTrade>,
        pub timeline: Option<BacktestTimeline>,
    }

    impl BacktestFixture {
        pub fn load(path: &Path) -> Self {
            let data = fs::read_to_string(path).unwrap();
            let json: serde_json::Value = serde_json::from_str(&data).unwrap();

            return serde_json::from_value(json).unwrap();
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestTimeline {
        pub equity: Option<Vec<f64>>,
        pub position_size: Option<Vec<f64>>,
        pub open_profit: Option<Vec<f64>>,
        pub net_profit: Option<Vec<f64>>,
        pub open_trades: Option<Vec<f64>>,
        pub closed_trades: Option<Vec<f64>>,
        pub gross_loss: Option<Vec<f64>>,
        pub gross_profit: Option<Vec<f64>>,
        pub winning_trades: Option<Vec<f64>>,
        pub losing_trades: Option<Vec<f64>>,
    }

    impl BacktestTimeline {
        pub fn default() -> Self {
            Self {
                equity: None,
                position_size: None,
                open_profit: None,
                net_profit: None,
                open_trades: None,
                closed_trades: None,
                gross_loss: None,
                gross_profit: None,
                winning_trades: None,
                losing_trades: None,
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestFixtureConfig {
        pub initial_capital: f64,
        pub process_orders_on_close: bool,
        pub price: Option<Vec<f64>>,
        pub fixture: Option<String>,
        pub price_precision: Option<usize>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestSignal {
        pub __disabled: Option<bool>,
        pub bar_index: usize,
        pub kind: String,
        pub id: String,
        pub direction: String,
        pub qty: Option<f64>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestTrade {
        pub __disabled: Option<bool>,
        pub entry_bar_index: Option<usize>,
        pub entry_price: Option<f64>,
        pub entry_id: Option<String>,
        pub exit_bar_index: Option<usize>,
        pub exit_price: Option<f64>,
        pub exit_id: Option<String>,
        pub profit: Option<f64>,
        pub size: Option<f64>,
        pub direction: Option<String>,
        pub closed: bool,
    }

    impl BacktestTrade {
        pub fn from_trade(trade: &Trade) -> Self {
            return Self {
                __disabled: Some(false),
                entry_bar_index: trade.entry_bar_index,
                entry_price: Some(trade.entry_price),
                entry_id: trade.entry_id.clone(),
                exit_bar_index: trade.exit_bar_index,
                exit_price: Some(trade.exit_price),
                exit_id: trade.exit_id.clone(),
                profit: Some(trade.profit),
                size: Some(trade.size),
                direction: Some(match trade.direction {
                    TradeDirection::Long => "long".to_string(),
                    TradeDirection::Short => "short".to_string(),
                }),
                closed: trade.closed,
            };
        }

        pub fn assert_eq(&self, other: &BacktestTrade) {
            assert_eq!(
                self.entry_bar_index, other.entry_bar_index,
                "[entry_bar_index] {:?} != {:?} \nSrc: {:?}\nTarget: {:?}",
                self.entry_bar_index, other.entry_bar_index, self, other
            );

            let price_precision = 1.0;

            assert!(
                other.entry_price.is_none()
                    || self
                        .entry_price
                        .unwrap_nan()
                        .compare(other.entry_price.unwrap_nan()),
                "[entry_price mismatch] {:?} != {:?}\nSrc: {:?}\nTarget: {:?}",
                self.entry_price,
                other.entry_price,
                self,
                other
            );

            assert_eq!(
                self.entry_id, other.entry_id,
                "[entry_id] {:?} != {:?}\nSrc: {:?}\nTarget: {:?}",
                self.entry_id, other.entry_id, self, other
            );

            assert_eq!(
                self.exit_bar_index, other.exit_bar_index,
                "[exit_bar_index] {:?} != {:?}\nSrc: {:?}\nTarget: {:?}",
                self.exit_bar_index, other.exit_bar_index, self, other
            );

            assert!(
                other.exit_price.is_none()
                    || self
                        .exit_price
                        .unwrap_nan()
                        .compare(other.exit_price.unwrap_nan()),
                "[exit_price] {:?} != {:?}\nSrc: {:?}\nTarget: {:?}",
                self.exit_price,
                other.exit_price,
                self,
                other
            );

            assert_eq!(
                self.exit_id, other.exit_id,
                "[exit_id] {:?} != {:?}\nSrc: {:?}\nTarget: {:?}",
                self.exit_id, other.exit_id, self, other
            );

            assert!(
                other.profit.is_none()
                    || self
                        .profit
                        .unwrap_nan()
                        .compare_with_precision(other.profit.unwrap_nan(), price_precision),
                "[profit] {:?} != {:?}\nSrc: {:?}\nTarget: {:?}",
                self.profit,
                other.profit,
                self,
                other
            );

            assert!(
                other.size.is_none() || self.size.unwrap_nan().compare(other.size.unwrap_nan()),
                "[size] {:?} != {:?}\nSrc: {:?}\nTarget: {:?}",
                self.size,
                other.size,
                self,
                other
            );

            assert_eq!(
                self.direction, other.direction,
                "[direction] {:?} != {:?}\nSrc: {:?}\nTarget: {:?}",
                self.direction, other.direction, self, other
            );

            assert_eq!(
                self.closed, other.closed,
                "[closed] {:?} != {:?}\nSrc: {:?}\nTarget: {:?}",
                self.closed, other.closed, self, other
            );
        }
    }

    pub struct Backtest {
        pub fixture: BacktestFixture,
        pub ctx: Context,
        pub strategy: Strategy,
    }

    impl Backtest {
        pub fn new(mut backtest_fixture: BacktestFixture) -> Self {
            let mut ctx: Option<Context> = None;

            let mut df: Option<DataFrame> = None;

            if let Some(fixture_path) = &backtest_fixture.config.fixture {
                let (_df, _ctx) = Fixture::load(&format_path(fixture_path));

                backtest_fixture.timeline = Some(BacktestTimeline {
                    equity: Some(_df.column("equity").unwrap().to_f64()),
                    position_size: Some(_df.column("position_size").unwrap().to_f64()),
                    open_profit: Some(_df.column("open_profit").unwrap().to_f64()),
                    net_profit: Some(_df.column("net_profit").unwrap().to_f64()),
                    open_trades: Some(_df.column("open_trades").unwrap().to_f64()),
                    closed_trades: Some(_df.column("closed_trades").unwrap().to_f64()),
                    gross_loss: Some(_df.column("gross_loss").unwrap().to_f64()),
                    gross_profit: Some(_df.column("gross_profit").unwrap().to_f64()),
                    winning_trades: Some(_df.column("winning_trades").unwrap().to_f64()),
                    losing_trades: Some(_df.column("losing_trades").unwrap().to_f64()),
                    ..BacktestTimeline::default()
                });

                df = Some(_df);
                ctx = Some(_ctx);
            } else {
                let price = backtest_fixture.config.price.clone().unwrap();
                ctx = Some(Context::new(
                    InMemoryDataProvider::from_values(price).to_arc(),
                ));
            }

            let ctx = ctx.unwrap();

            let strategy_config = StrategyConfig {
                initial_capital: backtest_fixture.config.initial_capital,
                process_orders_on_close: backtest_fixture.config.process_orders_on_close,
                ..StrategyConfig::default(ctx.clone())
            };

            let strategy = Strategy::new(ctx.clone(), strategy_config);

            Self {
                fixture: backtest_fixture,
                ctx,
                strategy,
            }
        }

        pub fn run(&mut self) {
            for i in self.ctx.first_bar_index..=self.ctx.last_bar_index {
                self.ctx.bar.index.set(i);

                let bar = &self.ctx.bar;
                let bar_index = bar.index();

                self.strategy.next_bar();

                let signal_fixture = self
                    .fixture
                    .signals
                    .iter()
                    .find(|s| s.bar_index == bar_index && !s.__disabled.unwrap_or(false));

                if let Some(signal_fixture) = signal_fixture {
                    let direction = match signal_fixture.direction.as_str() {
                        "long" => TradeDirection::Long,
                        "short" => TradeDirection::Short,
                        _ => panic!("Unknown direction {}", signal_fixture.direction),
                    };

                    let config = SignalOptions::new(direction)
                        .with_qty(Qty::EquityPct(1.0))
                        .with_id(signal_fixture.id.clone());

                    let signal: Signal = match signal_fixture.kind.as_str() {
                        "entry" => Signal::Entry(config),
                        "exit" => Signal::Exit(config),
                        _ => panic!("Unknown signal kind {}", signal_fixture.kind),
                    };

                    self.strategy.signal(signal);
                }

                self.strategy.next(());

                let price_precision = 1.0;

                if true {
                    if let Some(timeline) = &self.fixture.timeline {
                        if let Some(equity_series) = &timeline.equity {
                            let target_equity = equity_series[bar_index];
                            assert!(
                                self.strategy
                                    .equity
                                    .compare_with_precision(target_equity, price_precision),
                                "Equity mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                self.strategy.equity,
                                target_equity,
                                self.strategy.equity - target_equity
                            );
                        }

                        if let Some(position_size_series) = &timeline.position_size {
                            let target_position_size = position_size_series[bar_index];
                            assert!(
                                self.strategy.position_size.compare(target_position_size),
                                "Position size mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                self.strategy.position_size,
                                target_position_size,
                                self.strategy.position_size - target_position_size
                            );
                        }

                        if let Some(open_profit_series) = &timeline.open_profit {
                            let target_open_profit = open_profit_series[bar_index];
                            assert!(
                                self.strategy
                                    .open_profit
                                    .compare_with_precision(target_open_profit, price_precision),
                                "Open profit mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                self.strategy.open_profit,
                                target_open_profit,
                                self.strategy.open_profit - target_open_profit
                            );
                        }

                        if let Some(net_profit_series) = &timeline.net_profit {
                            let target_net_profit = net_profit_series[bar_index];
                            assert!(
                                self.strategy
                                    .net_profit
                                    .compare_with_precision(target_net_profit, price_precision),
                                "Net profit mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                self.strategy.net_profit,
                                target_net_profit,
                                self.strategy.net_profit - target_net_profit
                            );
                        }

                        if let Some(open_trades_series) = &timeline.open_trades {
                            let src_open_trades = self.strategy.open_trades.len() as f64;
                            let target_open_trades = open_trades_series[bar_index];
                            assert!(
                                src_open_trades.compare(target_open_trades),
                                "Open trades mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                src_open_trades,
                                src_open_trades,
                                src_open_trades - target_open_trades
                            );
                        }

                        if let Some(closed_trades_series) = &timeline.closed_trades {
                            let src_closed_trades = self.strategy.closed_trades.len() as f64;
                            let target_closed_trades = closed_trades_series[bar_index];
                            assert!(
                                src_closed_trades.compare(target_closed_trades),
                                "Closed trades mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                src_closed_trades,
                                target_closed_trades,
                                src_closed_trades - target_closed_trades
                            );
                        }

                        if let Some(gross_profit) = &timeline.gross_profit {
                            let src_gross_profit = self.strategy.gross_profit;
                            let target_gross_profit = gross_profit[bar_index];
                            assert!(
                                src_gross_profit
                                    .compare_with_precision(target_gross_profit, price_precision),
                                "Gross profit mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                src_gross_profit,
                                target_gross_profit,
                                src_gross_profit - target_gross_profit
                            );
                        }

                        if let Some(gross_loss) = &timeline.gross_loss {
                            let src_gross_loss = self.strategy.gross_loss;
                            let target_gross_loss = gross_loss[bar_index];
                            assert!(
                                src_gross_loss
                                    .compare_with_precision(target_gross_loss, price_precision),
                                "Gross loss mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                src_gross_loss,
                                target_gross_loss,
                                src_gross_loss - target_gross_loss
                            );
                        }

                        if let Some(winning_trades) = &timeline.winning_trades {
                            let src_winning_trades = self.strategy.winning_trades as f64;
                            let target_winning_trades = winning_trades[bar_index];
                            assert!(
                                src_winning_trades.compare(target_winning_trades),
                                "Winning trades mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                src_winning_trades,
                                target_winning_trades,
                                src_winning_trades - target_winning_trades
                            );
                        }

                        if let Some(losing_trades) = &timeline.losing_trades {
                            let src_losing_trades = self.strategy.losing_trades as f64;
                            let target_losing_trades = losing_trades[bar_index];
                            assert!(
                                src_losing_trades.compare(target_losing_trades),
                                "Losing trades mismatch at bar index {}. {} != {} | Diff: {}",
                                bar_index,
                                src_losing_trades,
                                target_losing_trades,
                                src_losing_trades - target_losing_trades
                            );
                        }
                    }
                }
            }

            let trades_fixture = self
                .fixture
                .trades
                .iter()
                .filter(|t| !t.__disabled.unwrap_or(false))
                .collect::<Vec<_>>();

            let closed_trades_fixture = trades_fixture
                .iter()
                .filter(|t| t.closed)
                .map(|t| t.clone())
                .collect::<Vec<_>>();
            let open_trades_fixture = trades_fixture
                .iter()
                .filter(|t| !t.closed)
                .map(|t| t.clone())
                .collect::<Vec<_>>();

            let closed_trades = self.strategy.closed_trades.clone();
            let open_trades = self.strategy.open_trades.clone();
            let trades = closed_trades
                .iter()
                .chain(open_trades.iter())
                .collect::<Vec<_>>();

            for trade_fixture in &self.fixture.trades {
                if trade_fixture.__disabled.unwrap_or(false) {
                    continue;
                }

                let trade = trades.iter().find(|t| {
                    t.entry_id == trade_fixture.entry_id && t.exit_id == trade_fixture.exit_id
                });

                assert!(
                    trade.is_some(),
                    "Trade not found. Entry ID: {:?}, Exit ID: {:?}, Direction: {:?}\nTrades: {:?}",
                    trade_fixture.entry_id,
                    trade_fixture.exit_id,
                    trade_fixture.direction,
                    trades
                );
                let trade = *trade.unwrap();
                BacktestTrade::from_trade(trade).assert_eq(trade_fixture);
            }

            assert_eq!(
                closed_trades_fixture.len(),
                closed_trades.len(),
                "Closed trades count mismatch. Expected: {:?}, Actual: {:?}",
                closed_trades_fixture,
                closed_trades
            );
            assert_eq!(
                open_trades_fixture.len(),
                open_trades.len(),
                "Open trades count mismatch. Expected: {:?}, Actual: {:?}",
                open_trades_fixture,
                open_trades
            )
        }
    }

    // #[test]
    // pub fn simple() {
    //     Backtest::new(BacktestFixture::load(&format_path("simple.json"))).run();
    // }

    #[test]
    pub fn tv_entry_only_duplicates() {
        Backtest::new(BacktestFixture::load(&format_path(
            "tv_entry_only_duplicates.json",
        )))
        .run();
    }
}
