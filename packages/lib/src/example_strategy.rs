use colored::Colorize;
use std::time::Instant;

use crate::base::{
    components::testing::Fixture,
    strategy::{
        metrics::omega_ratio_metric::OmegaRatioMetric,
        strategy_context::{StrategyContext, StrategyContextConfig},
        strategy_runner::{StrategyRunner, StrategyRunnerConfig, StrategyRunnerMetricsConfig},
        trade::TradeDirection,
    },
};

pub fn run_example_strategy() -> u128 {
    return 0;
    // let (df, ctx) = Fixture::raw("ml/fixtures/btc_1d.csv");

    // let mut strategy_ctx = StrategyExecutionContext::new(
    //     ctx.clone(),
    //     StrategyExecutionContextConfig {
    //         on_bar_close: false,
    //         continous: false,
    //     },
    // );

    // let mut equity_metric = EquityMetric::new(
    //     ctx.clone(),
    //     EquityMetricConfig {
    //         initial_capital: 1000.0,
    //     },
    // );
    // let mut sharpe_ratio_metric = SharpeRatioMetric::new(
    //     ctx.clone(),
    //     SharpeRatioMetricConfig {
    //         risk_free_rate: 0.0,
    //     },
    // );
    // let mut omega_ratio_metric = OmegaRatioMetric::new(
    //     ctx.clone(),
    //     OmegaRatioMetricConfig {
    //         risk_free_rate: 0.0,
    //     },
    // );
    // let mut total_closed_trades_metric = TotalClosedTradesMetric::new(ctx.clone());

    // let rsi_indicator = &mut RelativeStrengthIndexIndicator::new(
    //     ctx.clone(),
    //     RelativeStrengthIndexIndicatorConfig {
    //         length: 140,
    //         src: Source::from_kind(ctx.clone(), SourceKind::OHLC4),
    //     }, // RelativeStrengthIndexIndicatorConfig::default(ctx.clone()),
    // );

    // let rsi_strategy = &mut RelativeStrengthIndexStrategy::new(
    //     ctx.clone(),
    //     RelativeStrengthIndexStrategyConfig {
    //         threshold_oversold: 41.0825080871582,
    //         threshold_overbought: 56.878456115722656,
    //     }, // RelativeStrengthIndexStrategyConfig::default(ctx.clone()),
    // );

    // let mut runner = StrategyRunner::new(
    //     ctx.clone(),
    //     StrategyExecutionContext::new(
    //         ctx.clone(),
    //         StrategyExecutionContextConfig {
    //             on_bar_close: false,
    //             continous: true,
    //         },
    //     ),
    //     // StrategyRunnerConfig::default(ctx.clone()),
    //     StrategyRunnerConfig {
    //         print: true,
    //         start_tick: Some(2327),
    //         end_tick: Some(3337),
    //         metrics: StrategyRunnerMetricsConfig {
    //             equity: Some(equity_metric),
    //             omega_ratio: Some(omega_ratio_metric),
    //             sharpe_ratio: Some(sharpe_ratio_metric),
    //             track: false,
    //         },
    //     },
    // );

    // let start_time = Instant::now();

    // let result = runner.run(|| {
    //     let ctx = ctx.get();
    //     let tick = ctx.current_tick;

    //     let mut trade: Option<TradeDirection> = None;

    //     let rsi = rsi_indicator.next();
    //     let rsi_trade = rsi_strategy.next(rsi);

    //     trade = rsi_trade;

    //     // if false {
    //     //     let rsi = rsi_indicator.next();
    //     //     let rsi_trade = rsi_strategy.next(rsi);

    //     //     trade = rsi_trade;
    //     // } else {
    //     //     let long_ticks = [2, 20];
    //     //     let short_ticks = [10, 15];

    //     //     if long_ticks.contains(&tick) {
    //     //         trade = Some(TradeDirection::Long);
    //     //     } else if short_ticks.contains(&tick) {
    //     //         trade = Some(TradeDirection::Short);
    //     //     }
    //     // }

    //     return trade;
    // });

    // let end_time = Instant::now();
    // let elapsed_time = end_time - start_time;
    // let elapsed_time = elapsed_time.as_micros();

    // return elapsed_time;
}

pub fn run_example_strategy_refactor() {
    let (df, ctx) = Fixture::raw("ml/fixtures/btc_1d.csv");

    let mut strategy_ctx = StrategyContext::new(
        ctx.clone(),
        StrategyContextConfig {
            on_bar_close: true,
            continous: false,
            initial_capital: 1000.0,
            buy_with_equity: false,
        },
    );

    let mut runner = StrategyRunner::new(
        ctx.clone(),
        StrategyContext::new(
            ctx.clone(),
            StrategyContextConfig {
                on_bar_close: false,
                continous: true,
                buy_with_equity: false,
                initial_capital: 1000.0,
            },
        ),
        // StrategyRunnerConfig::default(ctx.clone()),
        StrategyRunnerConfig {
            print: true,
            start_tick: None,
            end_tick: Some(10),
            metrics: StrategyRunnerMetricsConfig {
                omega_ratio: None,
                sharpe_ratio: None,
                track: false,
            },
        },
    );

    let res = runner.run(|| {
        let ctx = ctx.get();
        let tick = ctx.current_tick;

        let long_ticks = [4];
        let short_ticks = [2, 3, 7];

        let mut trade: Option<TradeDirection> = None;

        if long_ticks.contains(&tick) {
            trade = Some(TradeDirection::Long);
        } else if short_ticks.contains(&tick) {
            trade = Some(TradeDirection::Short);
        }

        return trade;
    });

    // for cctx in ctx {
    //     let ctx = cctx.get();
    //     let tick = ctx.current_tick;

    //     let long_ticks = [4];
    //     let short_ticks = [2, 3, 7];

    //     let mut trade: Option<TradeDirection> = None;

    //     if long_ticks.contains(&tick) {
    //         trade = Some(TradeDirection::Long);
    //     } else if short_ticks.contains(&tick) {
    //         trade = Some(TradeDirection::Short);
    //     }

    //     strategy_ctx.next(trade);

    //     if tick > 30 {
    //         break;
    //     }
    // }
}
