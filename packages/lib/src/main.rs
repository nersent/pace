#![allow(
    clippy::needless_return,
    clippy::type_complexity,
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::uninlined_format_args,
    clippy::module_inception,
    clippy::upper_case_acronyms,
    unused
)]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod base;
mod content;
mod example_strategy;
mod ml;
mod utils;
mod xd;
use kdam::tqdm;
use polars::prelude::{DataFrame, NamedFrom, PolarsResult};
use polars::series::Series;
use rand::Rng;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use crate::base::asset::asset_data_provider::AssetDataProvider;
use crate::base::asset::in_memory_asset_data_provider::InMemoryAssetDataProvider;
use crate::base::asset::source::{Source, SourceKind};
use crate::base::asset::timeframe::Timeframe;
use crate::base::components::component_context::{ComponentContext, ComponentContextConfig};
use crate::base::components::testing::Fixture;
use crate::base::ta::ema_component::ExponentialMovingAverageComponent;
use crate::base::ta::sma_component::SimpleMovingAverageComponent;
use crate::content::aroon_indicator::{AroonIndicator, AroonIndicatorConfig};
use crate::content::awesome_oscillator_indicator::{
    AwesomeOscillatorIndicator, AwesomeOscillatorIndicatorConfig,
};
use crate::utils::polars::save_df;
use crate::xd::component::Component;
use crate::xd::data_source_component::SourceComponent;
use crate::xd::ma::MaKind;
use crate::xd::ma_component::MaComponent;
use crate::xd::stats::{stdev_from_var, variance};
use crate::xd::xd_indicator::{AoIndicator, AoIndicatorConfig};
use ml::dataset_ml::generate_ml_datasets;

fn benchmark_example_strategy() {
    let mut times: Vec<u128> = vec![];

    let max = 2000;
    for i in 0..max {
        if i % 100 == 0 {
            println!("{:0.2}%", (i as f64) / (max as f64) * 100.0);
        }
        let time = example_strategy::run_example_strategy();
        times.push(time);
    }

    let mean = times.iter().sum::<u128>() as f64 / times.len() as f64;

    println!("Mean: {}us", mean);
}

fn aha() {
    benchmark_example_strategy();
    return ();
    let mut iterations = String::new();
    println!("How many iterations?");
    std::io::stdin().read_line(&mut iterations).unwrap();
    let iterations: u32 = iterations.trim().parse().unwrap();

    let mut page = String::new();
    println!("Which page?");
    std::io::stdin().read_line(&mut page).unwrap();
    page = page.trim().to_string();

    let df = Fixture::raw_df("ml/fixtures/btc_1d.csv");
    let asset_data_provider: Arc<dyn AssetDataProvider + Send + Sync> = Arc::new(
        InMemoryAssetDataProvider::from_df(&df, "btc_usd", Timeframe::OneDay),
    );

    let mut save_step = String::new();
    println!("Save step?");
    std::io::stdin().read_line(&mut save_step).unwrap();
    let save_step: u32 = save_step.trim().parse().unwrap();

    let mut largest_equity: f64 = 1000.0;

    let mut series_id: Vec<f64> = Vec::new();
    let mut series_config_continous: Vec<f64> = Vec::new();
    let mut series_config_rsi_length: Vec<f64> = Vec::new();
    let mut series_config_rsi_overbought: Vec<f64> = Vec::new();
    let mut series_config_rsi_oversold: Vec<f64> = Vec::new();
    let mut series_config_rsi_source_kind: Vec<f64> = Vec::new();

    let mut series_net_profit: Vec<f64> = Vec::new();
    let mut series_net_profit_percent: Vec<f64> = Vec::new();
    /// Current unrealized profit or loss for all open positions. In TradingView `strategy.openprofit`
    let mut series_open_profit: Vec<f64> = Vec::new();
    /// Total value of all completed winning trades. In TradingView `strategy.grossprofit`
    let mut series_gross_profit: Vec<f64> = Vec::new();
    let mut series_gross_profit_percent: Vec<f64> = Vec::new();
    /// Total value of all completed losing trades. In TradingView `strategy.grossloss`
    let mut series_gross_loss: Vec<f64> = Vec::new();
    let mut series_gross_loss_percent: Vec<f64> = Vec::new();
    /// Current equity (initial capital + net profit + open profit). In TradingView `strategy.equity`
    let mut series_equity: Vec<f64> = Vec::new();
    /// Net current equity (initial capital + net profit)
    let mut series_net_equity: Vec<f64> = Vec::new();
    /// Total number of closed tradesIn TradingView `strategy.closedtrades`
    let mut series_closed_trades: Vec<f64> = Vec::new();
    /// Total number of winning tradesIn TradingView `strategy.wintrades`
    let mut series_winning_trades: Vec<f64> = Vec::new();
    /// Total number of losing tradesIn TradingView `strategy.losstrades`
    let mut series_losing_trades: Vec<f64> = Vec::new();
    /// Maximum equity drawdown value for the whole trading interval. In TradingView `strategy.max_drawdown`
    let mut series_max_drawdown: Vec<f64> = Vec::new();
    let mut series_max_drawdown_percent: Vec<f64> = Vec::new();
    /// Maximum equity run-up value for the whole trading interval. In TradingView `strategy.max_runup`
    let mut series_max_run_up: Vec<f64> = Vec::new();
    let mut series_max_run_up_percent: Vec<f64> = Vec::new();
    /// The amount of money made for every unit of money it lost.
    let mut series_profit_factor: Vec<f64> = Vec::new();
    /// The percentage of winning trades generated by a strategy.
    let mut series_percent_profitable: Vec<f64> = Vec::new();
    /// The gross profit divided by the number of winning trades.
    let mut series_avg_winning_trade: Vec<f64> = Vec::new();
    /// The gross loss divided by the number of losing trades.
    let mut series_avg_losing_trade: Vec<f64> = Vec::new();
    /// The sum of money gained or lost by the average trade.
    let mut series_avg_trade: Vec<f64> = Vec::new();
    /// The average value of how many currency units you win for every unit you lose.
    let mut series_avg_winning_losing_trade_ratio: Vec<f64> = Vec::new();
    /// The overall profit or loss generated by long trades.
    let mut series_long_net_profit: Vec<f64> = Vec::new();
    let mut series_long_net_profit_percent: Vec<f64> = Vec::new();
    /// The overall profit or loss generated by short trades.
    let mut series_short_net_profit: Vec<f64> = Vec::new();
    let mut series_short_net_profit_percent: Vec<f64> = Vec::new();
    /// Long to short net profit ratio
    let mut series_long_short_net_profit_ratio: Vec<f64> = Vec::new();
    /// Maximum equity drawdown value for the equity curve. Uses `strategy.equity`
    let mut series_equity_max_drawdown: Vec<f64> = Vec::new();
    let mut series_equity_max_drawdown_percent: Vec<f64> = Vec::new();
    /// Maximum drawdown that occured during trades.
    let mut series_intra_trade_max_drawdown: Vec<f64> = Vec::new();
    let mut series_intra_trade_max_drawdown_percent: Vec<f64> = Vec::new();
    /// Maximum drawdown that occured on net equity (realized profits)
    let mut series_net_equity_max_drawdown_percent: Vec<f64> = Vec::new();

    ///////////////////////
    let mut series_equity_returns: Vec<f64> = Vec::new();
    let mut series_equity_returns_mean: Vec<f64> = Vec::new();
    let mut series_equity_returns_std: Vec<f64> = Vec::new();
    let mut series_equity_returns_sum: Vec<f64> = Vec::new();

    let mut series_equity_positive_returns: Vec<f64> = Vec::new();
    let mut series_equity_positive_returns_mean: Vec<f64> = Vec::new();
    let mut series_equity_positive_returns_std: Vec<f64> = Vec::new();
    let mut series_equity_positive_returns_sum: Vec<f64> = Vec::new();

    let mut series_equity_negative_returns: Vec<f64> = Vec::new();
    let mut series_equity_negative_returns_mean: Vec<f64> = Vec::new();
    let mut series_equity_negative_returns_std: Vec<f64> = Vec::new();
    let mut series_equity_negative_returns_sum: Vec<f64> = Vec::new();

    let mut series_equity_omega_ratio: Vec<f64> = Vec::new();
    let mut series_equity_sharpe_ratio: Vec<f64> = Vec::new();
    let mut series_equity_sortino_ratio: Vec<f64> = Vec::new();

    ///
    let mut series_net_equity_returns: Vec<f64> = Vec::new();
    let mut series_net_equity_returns_mean: Vec<f64> = Vec::new();
    let mut series_net_equity_returns_std: Vec<f64> = Vec::new();
    let mut series_net_equity_returns_sum: Vec<f64> = Vec::new();

    let mut series_net_equity_positive_returns: Vec<f64> = Vec::new();
    let mut series_net_equity_positive_returns_mean: Vec<f64> = Vec::new();
    let mut series_net_equity_positive_returns_std: Vec<f64> = Vec::new();
    let mut series_net_equity_positive_returns_sum: Vec<f64> = Vec::new();

    let mut series_net_equity_negative_returns: Vec<f64> = Vec::new();
    let mut series_net_equity_negative_returns_mean: Vec<f64> = Vec::new();
    let mut series_net_equity_negative_returns_std: Vec<f64> = Vec::new();
    let mut series_net_equity_negative_returns_sum: Vec<f64> = Vec::new();

    ///
    let mut series_equity_returns_history: Vec<f64> = Vec::new();
    let mut series_equity_returns_history_id: Vec<f64> = Vec::new();
    let mut series_equity_returns_history_index: Vec<f64> = Vec::new();

    let mut series_net_equity_history: Vec<f64> = Vec::new();
    let mut series_net_equity_history_id: Vec<f64> = Vec::new();
    let mut series_net_equity_history_index: Vec<f64> = Vec::new();

    let mut series_net_equity_omega_ratio: Vec<f64> = Vec::new();
    let mut series_net_equity_sharpe_ratio: Vec<f64> = Vec::new();
    let mut series_net_equity_sortino_ratio: Vec<f64> = Vec::new();

    for id in tqdm!(0..iterations) {
        let (res, config) =
            example_strategy::run_example_strategy_refactor(Arc::clone(&asset_data_provider));
        let metrics = res.metrics;
        let strategy_metrics = metrics.metrics;

        if strategy_metrics.closed_trades != 0
            && metrics.equity_metrics.is_some()
            && metrics.net_equity_metrics.is_some()
        {
            let equity_metrics = metrics.equity_metrics.unwrap();
            let net_equity_metrics = metrics.net_equity_metrics.unwrap();

            if metrics.metrics.net_equity > largest_equity {
                println!("New largest equity: {}", metrics.metrics.net_equity);
                largest_equity = metrics.metrics.net_equity;
            }

            let (continous, rsi_length, rsi_oversold, rsi_overbought, rsi_source_kind) = config;

            series_id.push(id as f64);
            series_config_continous.push(continous as i32 as f64);
            series_config_rsi_length.push(rsi_length as f64);
            series_config_rsi_overbought.push(rsi_overbought as f64);
            series_config_rsi_oversold.push(rsi_oversold as f64);
            series_config_rsi_source_kind.push(rsi_source_kind as f64);
            // --------------------

            series_net_profit.push(strategy_metrics.net_profit);
            series_net_profit_percent.push(strategy_metrics.net_profit_percent);
            /// Current unrealized profit or loss for all open positions. In TradingView `strategy.openprofit`
            series_open_profit.push(strategy_metrics.open_profit);
            /// Total value of all completed winning trades. In TradingView `strategy.grossprofit`
            series_gross_profit.push(strategy_metrics.gross_profit);
            series_gross_profit_percent.push(strategy_metrics.gross_profit_percent);
            /// Total value of all completed losing trades. In TradingView `strategy.grossloss`
            series_gross_loss.push(strategy_metrics.gross_loss);
            series_gross_loss_percent.push(strategy_metrics.gross_loss_percent);
            /// Current equity (initial capital + net profit + open profit). In TradingView `strategy.equity`
            series_equity.push(strategy_metrics.equity);
            /// Net current equity (initial capital + net profit)
            series_net_equity.push(strategy_metrics.net_equity);
            /// Total number of closed tradesIn TradingView `strategy.closedtrades`
            series_closed_trades.push(strategy_metrics.closed_trades as f64);
            /// Total number of winning tradesIn TradingView `strategy.wintrades`
            series_winning_trades.push(strategy_metrics.winning_trades as f64);
            /// Total number of losing tradesIn TradingView `strategy.losstrades`
            series_losing_trades.push(strategy_metrics.losing_trades as f64);
            /// Maximum equity drawdown value for the whole trading interval. In TradingView `strategy.max_drawdown`
            series_max_drawdown.push(strategy_metrics.max_drawdown);
            series_max_drawdown_percent.push(strategy_metrics.max_drawdown_percent);
            /// Maximum equity run-up value for the whole trading interval. In TradingView `strategy.max_runup`
            series_max_run_up.push(strategy_metrics.max_run_up);
            series_max_run_up_percent.push(strategy_metrics.max_run_up_percent);
            /// The amount of money made for every unit of money it lost.);
            series_profit_factor.push(strategy_metrics.profit_factor);
            /// The percentage of winning trades generated by a strategy.);
            series_percent_profitable.push(strategy_metrics.percent_profitable);
            /// The gross profit divided by the number of winning trades.);
            series_avg_winning_trade.push(strategy_metrics.avg_winning_trade);
            /// The gross loss divided by the number of losing trades.);
            series_avg_losing_trade.push(strategy_metrics.avg_losing_trade);
            /// The sum of money gained or lost by the average trade.);
            series_avg_trade.push(strategy_metrics.avg_trade);
            /// The average value of how many currency units you win for every unit you lose.);
            series_avg_winning_losing_trade_ratio
                .push(strategy_metrics.avg_winning_losing_trade_ratio);
            /// The overall profit or loss generated by long trades.);
            series_long_net_profit.push(strategy_metrics.long_net_profit);
            series_long_net_profit_percent.push(strategy_metrics.long_net_profit_percent);
            /// The overall profit or loss generated by short trades.);
            series_short_net_profit.push(strategy_metrics.short_net_profit);
            series_short_net_profit_percent.push(strategy_metrics.short_net_profit_percent);
            /// Long to short net profit ratio
            series_long_short_net_profit_ratio.push(strategy_metrics.long_short_net_profit_ratio);
            /// Maximum equity drawdown value for the equity curve. Uses `strategy.equity`
            series_equity_max_drawdown.push(strategy_metrics.equity_max_drawdown);
            series_equity_max_drawdown_percent.push(strategy_metrics.equity_max_drawdown_percent);
            /// Maximum drawdown that occured during trades.);
            series_intra_trade_max_drawdown.push(strategy_metrics.intra_trade_max_drawdown);
            series_intra_trade_max_drawdown_percent
                .push(strategy_metrics.intra_trade_max_drawdown_percent);
            /// Maximum drawdown that occured on net equity (realized profits)
            series_net_equity_max_drawdown_percent
                .push(strategy_metrics.net_equity_max_drawdown_percent);

            ///
            series_equity_returns.push(equity_metrics.returns.returns);
            series_equity_returns_mean.push(equity_metrics.returns.returns_mean);
            series_equity_returns_std.push(equity_metrics.returns.returns_stdev);
            series_equity_returns_sum.push(equity_metrics.returns.returns_sum);

            series_equity_positive_returns.push(equity_metrics.positive_returns.returns);
            series_equity_positive_returns_mean.push(equity_metrics.positive_returns.returns_mean);
            series_equity_positive_returns_std.push(equity_metrics.positive_returns.returns_stdev);
            series_equity_positive_returns_sum.push(equity_metrics.positive_returns.returns_sum);

            series_equity_negative_returns.push(equity_metrics.negative_returns.returns);
            series_equity_negative_returns_mean.push(equity_metrics.negative_returns.returns_mean);
            series_equity_negative_returns_std.push(equity_metrics.negative_returns.returns_stdev);
            series_equity_negative_returns_sum.push(equity_metrics.negative_returns.returns_sum);

            series_equity_omega_ratio.push(equity_metrics.omega_ratio);
            series_equity_sortino_ratio.push(equity_metrics.sortino_ratio);
            series_equity_sharpe_ratio.push(equity_metrics.sharpe_ratio);

            ///
            series_net_equity_returns.push(net_equity_metrics.returns.returns);
            series_net_equity_returns_mean.push(net_equity_metrics.returns.returns_mean);
            series_net_equity_returns_std.push(net_equity_metrics.returns.returns_stdev);
            series_net_equity_returns_sum.push(net_equity_metrics.returns.returns_sum);

            series_net_equity_positive_returns.push(net_equity_metrics.positive_returns.returns);
            series_net_equity_positive_returns_mean
                .push(net_equity_metrics.positive_returns.returns_mean);
            series_net_equity_positive_returns_std
                .push(net_equity_metrics.positive_returns.returns_stdev);
            series_net_equity_positive_returns_sum
                .push(net_equity_metrics.positive_returns.returns_sum);

            series_net_equity_negative_returns.push(net_equity_metrics.negative_returns.returns);
            series_net_equity_negative_returns_mean
                .push(net_equity_metrics.negative_returns.returns_mean);
            series_net_equity_negative_returns_std
                .push(net_equity_metrics.negative_returns.returns_stdev);
            series_net_equity_negative_returns_sum
                .push(net_equity_metrics.negative_returns.returns_sum);

            series_net_equity_omega_ratio.push(net_equity_metrics.omega_ratio);
            series_net_equity_sortino_ratio.push(net_equity_metrics.sortino_ratio);
            series_net_equity_sharpe_ratio.push(net_equity_metrics.sharpe_ratio);

            ///
            let _id = id as f64;
            series_equity_returns_history_id
                .extend(vec![_id; metrics.equity_returns_history.len()]);
            series_equity_returns_history.extend(metrics.equity_returns_history);

            series_net_equity_history_id.extend(vec![_id; metrics.net_equity_history.len()]);
            series_net_equity_history.extend(metrics.net_equity_history);

            // println!(
            //     "{:?}\n{:?}",
            //     series_net_equity_history_id, series_net_equity_history
            // );
            // break;
        }

        if id % save_step == 0 {
            println!("Saving");
            {
                let series_id = Series::new("id", series_id.clone());
                let series_config_continous =
                    Series::new("config_continous", series_config_continous.clone());
                let series_config_rsi_length =
                    Series::new("config_rsi_length", series_config_rsi_length.clone());
                let series_config_rsi_overbought = Series::new(
                    "config_rsi_overbought",
                    series_config_rsi_overbought.clone(),
                );
                let series_config_rsi_oversold =
                    Series::new("config_rsi_oversold", series_config_rsi_oversold.clone());
                let series_config_rsi_source_kind = Series::new(
                    "config_rsi_source_kind",
                    series_config_rsi_source_kind.clone(),
                );

                let mut series_net_profit = Series::new("net_profit", series_net_profit.clone());
                let mut series_net_profit_percent =
                    Series::new("net_profit_percent", series_net_profit_percent.clone());
                /// Current unrealized profit or loss for all open positions. In TradingView `strategy.openprofit`
                let mut series_open_profit = Series::new("open_profit", series_open_profit.clone());
                /// Total value of all completed winning trades. In TradingView `strategy.grossprofit`
                let mut series_gross_profit =
                    Series::new("gross_profit", series_gross_profit.clone());
                let mut series_gross_profit_percent =
                    Series::new("gross_profit_percent", series_gross_profit_percent.clone());
                /// Total value of all completed losing trades. In TradingView `strategy.grossloss`
                let mut series_gross_loss = Series::new("gross_loss", series_gross_loss.clone());
                let mut series_gross_loss_percent =
                    Series::new("gross_loss_percent", series_gross_loss_percent.clone());
                /// Current equity (initial capital + net profit + open profit). In TradingView `strategy.equity`
                let mut series_equity = Series::new("equity", series_equity.clone());
                /// Net current equity (initial capital + net profit)
                let mut series_net_equity = Series::new("net_equity", series_net_equity.clone());
                /// Total number of closed tradesIn TradingView `strategy.closedtrades`
                let mut series_closed_trades =
                    Series::new("closed_trades", series_closed_trades.clone());
                /// Total number of winning tradesIn TradingView `strategy.wintrades`
                let mut series_winning_trades =
                    Series::new("winning_trades", series_winning_trades.clone());
                /// Total number of losing tradesIn TradingView `strategy.losstrades`
                let mut series_losing_trades =
                    Series::new("losing_trades", series_losing_trades.clone());
                /// Maximum equity drawdown value for the whole trading interval. In TradingView `strategy.max_drawdown`
                let mut series_max_drawdown =
                    Series::new("max_drawdown", series_max_drawdown.clone());
                let mut series_max_drawdown_percent =
                    Series::new("max_drawdown_percent", series_max_drawdown_percent.clone());
                /// Maximum equity run-up value for the whole trading interval. In TradingView `strategy.max_runup`
                let mut series_max_run_up = Series::new("max_run_up", series_max_run_up.clone());
                let mut series_max_run_up_percent =
                    Series::new("max_run_up_percent", series_max_run_up_percent.clone());
                /// The amount of money made for every unit of money it lost.
                let mut series_profit_factor =
                    Series::new("profit_factor", series_profit_factor.clone());
                /// The percentage of winning trades generated by a strategy.
                let mut series_percent_profitable =
                    Series::new("percent_profitable", series_percent_profitable.clone());
                /// The gross profit divided by the number of winning trades.
                let mut series_avg_winning_trade =
                    Series::new("avg_winning_trade", series_avg_winning_trade.clone());
                /// The gross loss divided by the number of losing trades.
                let mut series_avg_losing_trade =
                    Series::new("avg_losing_trade", series_avg_losing_trade.clone());
                /// The sum of money gained or lost by the average trade.
                let mut series_avg_trade = Series::new("avg_trade", series_avg_trade.clone());
                /// The average value of how many currency units you win for every unit you lose.
                let mut series_avg_winning_losing_trade_ratio = Series::new(
                    "avg_winning_losing_trade_ratio",
                    series_avg_winning_losing_trade_ratio.clone(),
                );
                /// The overall profit or loss generated by long trades.
                let mut series_long_net_profit =
                    Series::new("long_net_profit", series_long_net_profit.clone());
                let mut series_long_net_profit_percent = Series::new(
                    "long_net_profit_percent",
                    series_long_net_profit_percent.clone(),
                );
                /// The overall profit or loss generated by short trades.
                let mut series_short_net_profit =
                    Series::new("short_net_profit", series_short_net_profit.clone());
                let mut series_short_net_profit_percent = Series::new(
                    "short_net_profit_percent",
                    series_short_net_profit_percent.clone(),
                );
                /// Long to short net profit ratio
                let mut series_long_short_net_profit_ratio = Series::new(
                    "long_short_net_profit_ratio",
                    series_long_short_net_profit_ratio.clone(),
                );
                /// Maximum equity drawdown value for the equity curve. Uses `strategy.equity`
                let mut series_equity_max_drawdown =
                    Series::new("equity_max_drawdown", series_equity_max_drawdown.clone());
                let mut series_equity_max_drawdown_percent = Series::new(
                    "equity_max_drawdown_percent",
                    series_equity_max_drawdown_percent.clone(),
                );
                /// Maximum drawdown that occured during trades.
                let mut series_intra_trade_max_drawdown = Series::new(
                    "intra_trade_max_drawdown",
                    series_intra_trade_max_drawdown.clone(),
                );
                let mut series_intra_trade_max_drawdown_percent = Series::new(
                    "intra_trade_max_drawdown_percent",
                    series_intra_trade_max_drawdown_percent.clone(),
                );
                /// Maximum drawdown that occured on net equity (realized profits)
                let mut series_net_equity_max_drawdown_percent = Series::new(
                    "net_equity_max_drawdown_percent",
                    series_net_equity_max_drawdown_percent.clone(),
                );

                ///
                let mut series_equity_returns =
                    Series::new("equity_returns", series_equity_returns.clone());
                let mut series_equity_returns_mean =
                    Series::new("equity_returns_mean", series_equity_returns_mean.clone());
                let mut series_equity_returns_std =
                    Series::new("equity_returns_stdev", series_equity_returns_std.clone());
                let mut series_equity_returns_sum =
                    Series::new("equity_returns_sum", series_equity_returns_sum.clone());

                let mut series_equity_positive_returns = Series::new(
                    "equity_positive_returns",
                    series_equity_positive_returns.clone(),
                );
                let mut series_equity_positive_returns_mean = Series::new(
                    "equity_positive_returns_mean",
                    series_equity_positive_returns_mean.clone(),
                );
                let mut series_equity_positive_returns_std = Series::new(
                    "equity_positive_returns_stdev",
                    series_equity_positive_returns_std.clone(),
                );
                let mut series_equity_positive_returns_sum = Series::new(
                    "equity_positive_returns_sum",
                    series_equity_positive_returns_sum.clone(),
                );

                let mut series_equity_negative_returns = Series::new(
                    "equity_negative_returns",
                    series_equity_negative_returns.clone(),
                );
                let mut series_equity_negative_returns_mean = Series::new(
                    "equity_negative_returns_mean",
                    series_equity_negative_returns_mean.clone(),
                );
                let mut series_equity_negative_returns_std = Series::new(
                    "equity_negative_returns_stdev",
                    series_equity_negative_returns_std.clone(),
                );
                let mut series_equity_negative_returns_sum = Series::new(
                    "equity_negative_returns_sum",
                    series_equity_negative_returns_sum.clone(),
                );

                let mut series_equity_omega_ratio =
                    Series::new("equity_omega_ratio", series_equity_omega_ratio.clone());
                let mut series_equity_sharpe_ratio =
                    Series::new("equity_sharpe_ratio", series_equity_sharpe_ratio.clone());
                let mut series_equity_sortino_ratio =
                    Series::new("equity_sortino_ratio", series_equity_sortino_ratio.clone());
                ///
                let mut series_net_equity_returns =
                    Series::new("net_equity_returns", series_net_equity_returns.clone());
                let mut series_net_equity_returns_mean = Series::new(
                    "net_equity_returns_mean",
                    series_net_equity_returns_mean.clone(),
                );
                let mut series_net_equity_returns_std = Series::new(
                    "net_equity_returns_stdev",
                    series_net_equity_returns_std.clone(),
                );
                let mut series_net_equity_returns_sum = Series::new(
                    "net_equity_returns_sum",
                    series_net_equity_returns_sum.clone(),
                );

                let mut series_net_equity_positive_returns = Series::new(
                    "net_equity_positive_returns",
                    series_net_equity_positive_returns.clone(),
                );
                let mut series_net_equity_positive_returns_mean = Series::new(
                    "net_equity_positive_returns_mean",
                    series_net_equity_positive_returns_mean.clone(),
                );
                let mut series_net_equity_positive_returns_std = Series::new(
                    "net_equity_positive_returns_stdev",
                    series_net_equity_positive_returns_std.clone(),
                );
                let mut series_net_equity_positive_returns_sum = Series::new(
                    "net_equity_positive_returns_sum",
                    series_net_equity_positive_returns_sum.clone(),
                );

                let mut series_net_equity_negative_returns = Series::new(
                    "net_equity_negative_returns",
                    series_net_equity_negative_returns.clone(),
                );
                let mut series_net_equity_negative_returns_mean = Series::new(
                    "net_equity_negative_returns_mean",
                    series_net_equity_negative_returns_mean.clone(),
                );
                let mut series_net_equity_negative_returns_std = Series::new(
                    "net_equity_negative_returns_stdev",
                    series_net_equity_negative_returns_std.clone(),
                );
                let mut series_net_equity_negative_returns_sum = Series::new(
                    "net_equity_negative_returns_sum",
                    series_net_equity_negative_returns_sum.clone(),
                );
                let mut series_net_equity_omega_ratio = Series::new(
                    "net_equity_omega_ratio",
                    series_net_equity_omega_ratio.clone(),
                );
                let mut series_net_equity_sharpe_ratio = Series::new(
                    "net_equity_sharpe_ratio",
                    series_net_equity_sharpe_ratio.clone(),
                );
                let mut series_net_equity_sortino_ratio = Series::new(
                    "net_equity_sortino_ratio",
                    series_net_equity_sortino_ratio.clone(),
                );

                let columns = vec![
                    series_id,
                    series_config_continous,
                    series_config_rsi_length,
                    series_config_rsi_overbought,
                    series_config_rsi_oversold,
                    series_config_rsi_source_kind,
                    //
                    series_net_profit,
                    series_net_profit_percent,
                    /// Current unrealized profit or loss for all open positions. In TradingView `strategy.openprofit`
                    series_open_profit,
                    /// Total value of all completed winning trades. In TradingView `strategy.grossprofit`
                    series_gross_profit,
                    series_gross_profit_percent,
                    /// Total value of all completed losing trades. In TradingView `strategy.grossloss`
                    series_gross_loss,
                    series_gross_loss_percent,
                    /// Current equity (initial capital + net profit + open profit). In TradingView `strategy.equity`
                    series_equity,
                    /// Net current equity (initial capital + net profit)
                    series_net_equity,
                    /// Total number of closed tradesIn TradingView `strategy.closedtrades`
                    series_closed_trades,
                    /// Total number of winning tradesIn TradingView `strategy.wintrades`
                    series_winning_trades,
                    /// Total number of losing tradesIn TradingView `strategy.losstrades`
                    series_losing_trades,
                    /// Maximum equity drawdown value for the whole trading interval. In TradingView `strategy.max_drawdown`
                    series_max_drawdown,
                    series_max_drawdown_percent,
                    /// Maximum equity run-up value for the whole trading interval. In TradingView `strategy.max_runup`
                    series_max_run_up,
                    series_max_run_up_percent,
                    /// The amount of money made for every unit of money it lost.
                    series_profit_factor,
                    /// The percentage of winning trades generated by a strategy.
                    series_percent_profitable,
                    /// The gross profit divided by the number of winning trades.
                    series_avg_winning_trade,
                    /// The gross loss divided by the number of losing trades.
                    series_avg_losing_trade,
                    /// The sum of money gained or lost by the average trade.
                    series_avg_trade,
                    /// The average value of how many currency units you win for every unit you lose.
                    series_avg_winning_losing_trade_ratio,
                    /// The overall profit or loss generated by long trades.
                    series_long_net_profit,
                    series_long_net_profit_percent,
                    /// The overall profit or loss generated by short trades.
                    series_short_net_profit,
                    series_short_net_profit_percent,
                    /// Long to short net profit ratio
                    series_long_short_net_profit_ratio,
                    /// Maximum equity drawdown value for the equity curve. Uses `strategy.equity`
                    series_equity_max_drawdown,
                    series_equity_max_drawdown_percent,
                    /// Maximum drawdown that occured during trades.
                    series_intra_trade_max_drawdown,
                    series_intra_trade_max_drawdown_percent,
                    /// Maximum drawdown that occured on net equity (realized profits)
                    series_net_equity_max_drawdown_percent,
                    ///
                    series_equity_returns,
                    series_equity_returns_mean,
                    series_equity_returns_std,
                    series_equity_returns_sum,
                    series_equity_positive_returns,
                    series_equity_positive_returns_mean,
                    series_equity_positive_returns_std,
                    series_equity_positive_returns_sum,
                    series_equity_negative_returns,
                    series_equity_negative_returns_mean,
                    series_equity_negative_returns_std,
                    series_equity_negative_returns_sum,
                    series_equity_omega_ratio,
                    series_equity_sharpe_ratio,
                    series_equity_sortino_ratio,
                    ///
                    series_net_equity_returns,
                    series_net_equity_returns_mean,
                    series_net_equity_returns_std,
                    series_net_equity_returns_sum,
                    series_net_equity_positive_returns,
                    series_net_equity_positive_returns_mean,
                    series_net_equity_positive_returns_std,
                    series_net_equity_positive_returns_sum,
                    series_net_equity_negative_returns,
                    series_net_equity_negative_returns_mean,
                    series_net_equity_negative_returns_std,
                    series_net_equity_negative_returns_sum,
                    series_net_equity_omega_ratio,
                    series_net_equity_sharpe_ratio,
                    series_net_equity_sortino_ratio,
                ];

                let df: PolarsResult<DataFrame> = DataFrame::new(columns);
                let mut df = df.unwrap();

                save_df(
                    &mut df,
                    Path::new(&format!(".out/criterions/{page}/{id}/metrics.parquet")),
                );
            }
            // {
            //     // let mut series_equity_returns_history: Vec<f64> = Vec::new();
            //     // let mut series_equity_returns_history_id: Vec<f64> = Vec::new();

            //     // let mut series_net_equity_history: Vec<f64> = Vec::new();
            //     // let mut series_net_equity_history_id: Vec<f64> = Vec::new();
            //     let mut series_equity_returns_history_id =
            //         Series::new("id", series_equity_returns_history_id.clone());
            //     // let mut series_equity_returns_history_index =
            //     //     Series::new("_index", series_equity_returns_history_index);
            //     let mut series_equity_returns_history =
            //         Series::new("return", series_equity_returns_history.clone());

            //     let columns = vec![
            //         series_equity_returns_history_id,
            //         // series_equity_returns_history_index,
            //         series_equity_returns_history,
            //     ];

            //     let df: PolarsResult<DataFrame> = DataFrame::new(columns);
            //     let mut df = df.unwrap();

            //     save_df(
            //         &mut df,
            //         Path::new(&format!(
            //             ".out/criterions/{page}/{id}/equity_returns.parquet"
            //         )),
            //     );
            // }
            {
                // let mut series_equity_returns_history: Vec<f64> = Vec::new();
                // let mut series_equity_returns_history_id: Vec<f64> = Vec::new();

                // let mut series_net_equity_history: Vec<f64> = Vec::new();
                // let mut series_net_equity_history_id: Vec<f64> = Vec::new();
                let mut series_net_equity_history_id =
                    Series::new("id", series_net_equity_history_id.clone());
                // let mut series_equity_returns_history_index =
                //     Series::new("_index", series_equity_returns_history_index);
                let mut series_net_equity_history =
                    Series::new("net_equity", series_net_equity_history.clone());

                let columns = vec![
                    series_net_equity_history_id,
                    // series_equity_returns_history_index,
                    series_net_equity_history,
                ];

                let df: PolarsResult<DataFrame> = DataFrame::new(columns);
                let mut df = df.unwrap();

                save_df(
                    &mut df,
                    Path::new(&format!(".out/criterions/{page}/{id}/net_equity.parquet")),
                );
            }
        }
    }

    // benchmark_example_strategy();
    // example_strategy::run_example_strategy();
    // generate_ml_datasets();
}

fn main() {
    let df = Fixture::raw_df("btc_1d.csv");
    let asset_data_provider: Arc<dyn AssetDataProvider + Send + Sync> = Arc::new(
        InMemoryAssetDataProvider::from_df(&df, "btc_usd", Timeframe::OneDay),
    );

    println!("Loaded");

    let mut iterations = String::new();
    println!("How many iterations?");
    std::io::stdin().read_line(&mut iterations).unwrap();
    let iterations: u32 = iterations.trim().parse().unwrap();
    // let iterations: u32 = 1;

    let mut time_list: Vec<u128> = Vec::new();
    let mut time_list_s: Vec<f64> = Vec::new();

    for id in tqdm!(0..iterations) {
        let ctx = ComponentContext::from_asset_data_provider(Arc::clone(&asset_data_provider));
        let mut sma = SimpleMovingAverageComponent::new(ctx.clone(), 100);
        let mut ema = ExponentialMovingAverageComponent::new(ctx.clone(), 229);
        let mut ao = AwesomeOscillatorIndicator::new(
            ctx.clone(),
            AwesomeOscillatorIndicatorConfig {
                long_length: 30,
                long_ma_type: base::ta::ma::MovingAverageKind::SMA,
                short_length: 14,
                short_ma_type: base::ta::ma::MovingAverageKind::RMA,
                long_source: Source::from_kind(ctx.clone(), SourceKind::Close),
                short_source: Source::from_kind(ctx.clone(), SourceKind::OHLC4),
            },
        );

        // let mut ao_xd = AoIndicator::new(
        //     ctx.clone(),
        //     AoIndicatorConfig {
        //         long_ma: Box::new(MaComponent::new(ctx.clone(), MaKind::SMA, 30)),
        //         long_src: Box::new(SourceComponent::new(
        //             ctx.clone(),
        //             xd::source_kind::SourceKind::Close,
        //         )),
        //         short_ma: Box::new(MaComponent::new(ctx.clone(), MaKind::RMA, 14)),
        //         short_src: Box::new(SourceComponent::new(
        //             ctx.clone(),
        //             xd::source_kind::SourceKind::OHLC4,
        //         )),
        //     },
        // );

        // let mut aroon = AroonIndicator::new(ctx.clone(), AroonIndicatorConfig { length: 30 });
        // let mut aroon_xd = XdIndicator::new(ctx.clone(), XdIndicatorConfig {});

        let start_time = Instant::now();
        let mut last_value: Option<f64> = None;

        for cctx in ctx {
            let ctx = cctx.get();
            let close = ctx.close();
            let output = ao.next();
            // let output = ao_xd.next(());
            last_value = output;
        }

        let end_time = Instant::now();
        let time = end_time - start_time;
        time_list.push(time.as_micros());
        time_list_s.push(time.as_secs_f64());
        println!("{:?}", last_value);
    }

    let time_mean = time_list_s.iter().sum::<f64>() / time_list_s.len() as f64;
    let time_var = variance(&time_list_s, time_mean);
    let time_stdev = stdev_from_var(time_var);

    println!("Time list: {:?}", time_list);

    println!(WRAR
        "Mean time: {}ms | Stdev: {}ms",
        time_mean / 1000.0,
        time_stdev * 1000.0
    );
}
