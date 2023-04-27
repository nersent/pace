use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell, UnsafeCell},
    path::Path,
    rc::Rc,
    sync::Arc,
};

use nersent_pace::{
    content::relative_strength_index::{
        RelativeStrengthIndex, RelativeStrengthIndexConfig, RelativeStrengthIndexStrategy,
        RelativeStrengthIndexStrategyConfig,
    },
    core::{
        asset::Asset,
        context::Context,
        data_provider::DataProvider,
        in_memory_data_provider::InMemoryDataProvider,
        incremental::{Chained, ForcedInput, Incremental, IncrementalDefault},
        timeframe::Timeframe,
    },
    pinescript::pinescript_exporter::{PineScriptExportStrategyConfig, PineScriptExporter},
    polars::io::read_df,
    statistics::normalization::{
        FixedScaler, FixedScalerConfig, MinMaxScaler, MinMaxScalerConfig, StandardScaler,
        StandardScalerConfig,
    },
    strategy::{
        metrics::tradingview_metrics::{
            TradingViewMetrics, TradingViewMetricsConfig, TradingViewMetricsData,
            TradingViewMetricsProvider,
        },
        optimization::{force_curve_fit, ForceCurveFitConfig},
        strategy::{Strategy, StrategyConfig},
        strategy_runner::{
            StrategyRunner, StrategyRunnerTarget, StrategyRunnerTargetMetricsProvider,
            StrategyRunnerTargetOptions,
        },
        trade::{SignalFixture, StrategySignal},
    },
};

// use nersent_pace::{
//     content::relative_strength_index::{
//         RelativeStrengthIndex, RelativeStrengthIndexConfig, RelativeStrengthIndexStrategy,
//         RelativeStrengthIndexStrategyConfig,
//     },
//     core::{
//         context::Context,
//         data_provider::DataProvider,
//         in_memory_data_provider::InMemoryDataProvider,
//         incremental::{Incremental, IncrementalDefault},
//     },
//     pinescript::pinescript_exporter::{PineScriptExportStrategyConfig, PineScriptExporter},
//     polars::io::read_df,
//     strategy::{
//         metrics::{
//             cobra_metrics::{CobraMetrics, CobraMetricsConfig},
//             tradingview_metrics::{TradingViewMetrics, TradingViewMetricsConfig},
//         },
//         optimization::{fit_trades, FitTradesConfig},
//         strategy::{Strategy, StrategyConfig},
//     },
//     ta::relative_strength_index::Rsi,
// };

// pub trait IncrementalNew<T, R> {
//     fn next(&self, data: T) -> R;
//     // fn chain<K>(self, next: Box<dyn IncrementalNew<R, K>>) -> f64;
// }

// pub trait IncrementalXd<T, R>
// where
//     Self: Sized,
// {
//     fn chain<K>(self, next: Box<dyn IncrementalNew<R, K>>) -> f64 {
//         return 0.0;
//     }
// }

// pub trait IncrementChain<T: Sized, R: Sized, K: IncrementalNew<T, R>>:
//     IncrementalNew<T, R> + Sized
// where
//     Self: Sized,
// {
//     fn chain(self, next: K) -> f64;
// }

// type Xd = Box<dyn IncrementalNew<(), f64>  ;

// struct AhaSrc {};

// impl AhaSrc {
//     fn new() -> Self {
//         return Self {};
//     }
// }

// impl IncrementalNew<(), f64> for AhaSrc {
//     fn next(&self, _: ()) -> f64 {
//         return 0.0;
//     }
// }

// impl IncrementalXd<(), f64> for AhaSrc {

// }capture.html
// }

// impl Aha {
//     fn new(xd: Xd) -> Self {
//         return Self { xd };
//     }
// }

// impl IncrementalNew<f64, f64> for Aha {
//     fn next(&self, xd: f64) -> f64 {
//         return 6.9;
//     }
// }

pub struct Xd {}

impl Incremental<&Strategy, StrategySignal> for Xd {
    fn next(&mut self, input: &Strategy) -> StrategySignal {
        return StrategySignal::Neutral;
    }
}

fn main() {
    let data_path = Path::new("example/fixtures/btc_1d.csv");
    let df = read_df(&data_path);

    let ctx = Context::new(InMemoryDataProvider::from_df(&df).to_arc());

    let mut strategy = Strategy::new(
        ctx.clone(),
        StrategyConfig {
            on_bar_close: false,
            ..Default::default()
        },
    );

    ctx.bar.index.set(0);
    strategy.next(StrategySignal::Neutral);
    strategy.next(StrategySignal::Long);
    println!(
        "[0]: {:?} | {:?}",
        strategy.current_dir, strategy.metrics.position_size
    );

    ctx.bar.index.set(1);
    strategy.next(StrategySignal::Neutral);
    strategy.next(StrategySignal::Neutral);
    println!(
        "[1]: {:?} | {:?}",
        strategy.current_dir, strategy.metrics.position_size
    );

    // let chained = /*ForcedInput::new(
    //     ctx.clone(),*/
    //     <StrippedInput<f64> as Incremental<&Strategy, f64>>::to_box(StrippedInput::new(
    //         ctx.clone(),
    //         RelativeStrengthIndex::new(
    //             ctx.clone(),
    //             RelativeStrengthIndexConfig::default(ctx.clone()),
    //         )
    //         .to_box(),
    //         // Chained::new(
    //                    //     ctx.clone(),
    //                    //     RelativeStrengthIndex::new(
    //                    //         ctx.clone(),
    //                    //         RelativeStrengthIndexConfig::default(ctx.clone()),
    //                    //     )
    //                    //     .to_box(),
    //                    //     RelativeStrengthIndexStrategy::new(
    //                    //         ctx.clone(),
    //                    //         RelativeStrengthIndexStrategyConfig::default(),
    //                    //     )
    //                    //     .to_box(),
    //                    // ),
    //     ));
    // //);

    // let target = StrategyRunnerTarget::<TradingViewMetricsData> {
    //     assets: vec![Asset {
    //         hash: "test".to_string(),
    //         symbol: "test".to_string(),
    //         timeframe: Timeframe::OneDay,
    //     }],
    //     id: "test".to_string(),
    //     options: StrategyRunnerTargetOptions {
    //         data_provider: Box::new(|asset| {
    //             let data_path = Path::new("example/fixtures/btc_1d.csv");
    //             let df = read_df(&data_path);
    //             return InMemoryDataProvider::from_df(&df).to_arc();
    //         }),
    //         ctx: Box::new(|data_provider, asset| {
    //             return Context::new(data_provider);
    //         }),
    //         strategy: Box::new(|ctx, data_provider, asset| {
    //             return Strategy::new(ctx, StrategyConfig::default());
    //         }),
    //         target: Box::new(move |ctx, data_provider, strategy, asset| {
    //             return <ForcedInput<StrategySignal> as Incremental<&Strategy, StrategySignal>>::to_box(
    //             ForcedInput::new(
    //                 ctx.clone(),
    //                 Chained::new(
    //                     ctx.clone(),
    //                     RelativeStrengthIndex::new(
    //                         ctx.clone(),
    //                         RelativeStrengthIndexConfig::default(ctx.clone()),
    //                     )
    //                     .to_box(),
    //                     RelativeStrengthIndexStrategy::new(
    //                         ctx.clone(),
    //                         RelativeStrengthIndexStrategyConfig::default(),
    //                     )
    //                     .to_box(),
    //                 )
    //                 .to_box(),
    //             ),
    //         );
    //         }),
    //         metrics_provider: Some(Box::new(|ctx, data_provider, strategy, asset| {
    //             return TradingViewMetrics::new(
    //                 ctx.clone(),
    //                 strategy,
    //                 TradingViewMetricsConfig::default(),
    //             )
    //             .to_box();
    //         })),
    //         periods: Box::new(|ctx, data_provider, asset| {
    //             return vec![(data_provider.get_start_tick(), data_provider.get_end_tick())];
    //         }),
    //     },
    // };

    // let mut strategy_runner = StrategyRunner::new();

    // let res = strategy_runner.run(vec![target]);

    // println!(
    //     "{:?}",
    //     res[0]
    //         .metrics
    //         .as_ref()
    //         .unwrap()
    //         .get_metrics()
    //         .net_equity_history
    // );

    // let data_path = Path::new("example/fixtures/btc_1d.csv");
    // let df = read_df(&data_path);

    // let ctx = Context::new(InMemoryDataProvider::from_df(&df).to_arc());
    // let mut strategy = Strategy::new(ctx.clone(), StrategyConfig::default());

    // // let signals = force_curve_fit(
    // //     Arc::clone(&ctx.data),
    // //     ForceCurveFitConfig {
    // //         start_index: ctx.data.get_end_tick() - 30,
    // //         end_index: ctx.data.get_end_tick(),
    // //     },
    // // );

    // let mut min_max_scaler = MinMaxScaler::new(ctx.clone(), MinMaxScalerConfig::default());
    // let mut fixed_scaler = FixedScaler::new(
    //     ctx.clone(),
    //     FixedScalerConfig {
    //         data_min: -10.0,
    //         data_max: 10.0,
    //         min: -1.0,
    //         max: 1.0,
    //     },
    // );
    // let mut z_score = StandardScaler::new(ctx.clone(), StandardScalerConfig::default());

    // for i in ctx.clone() {
    //     ctx.bar.index.set(i);

    //     let close = ctx.bar.close();

    //     if i < 50 {
    //         println!(
    //             "[{}]: {} -> {}; {} | ({}; {})",
    //             i,
    //             close,
    //             min_max_scaler.next(close),
    //             z_score.next(close),
    //             min_max_scaler.data_min,
    //             min_max_scaler.data_max
    //         );
    //     }

    //     // strategy.next(signals.get(i));
    //     // metrics.next(&strategy);

    //     // let rsi = rsi_indicator.next(());
    //     // let rsi_signal = rsi_strategy.next(rsi);

    //     // strategy.next(rsi_signal);
    //     // metrics.next(&strategy);
    // }

    // let ps_exporter = PineScriptExporter::new();
    // let ps = ps_exporter.strategy(&strategy, PineScriptExportStrategyConfig::default());

    // println!("{}", ps);

    // let signals = SignalFixture {
    //     long_entries: vec![2],
    //     long_exits: vec![],
    //     short_entries: vec![5],
    //     short_exits: vec![],
    //     // long_entries: vec![2, 8],
    //     // long_exits: vec![14],
    //     // short_entries: vec![5, 17],
    //     // short_exits: vec![20],
    // };

    // for i in ctx.clone() {
    //     ctx.bar.index.set(i);
    //     strategy.next(signals.get(i));
    // }

    // println!("sum: {}", 2.0 + f64::NAN);
    // println!("mult: {}", 2.0 * f64::NAN);
    // println!("pow: {}", f64::powf(2.0, f64::NAN));
    // println!("pow: {}", f64::powf(f64::NAN, 2.0));
    // println!("max: {}", f64::max(f64::NAN, 2.0));
    // println!("diff: {}", 1.0 - f64::NAN);

    return;

    // let aha_src = AhaSrc::new().;

    // AhaSrc::ch

    // let data_path = Path::new("example/fixtures/btc_1d.csv");
    // let df = read_df(&data_path);

    // let ctx = Context::new(InMemoryDataProvider::from_df(&df).to_arc());

    // let mut strategy = Strategy::new(
    //     ctx.clone(),
    //     StrategyConfig {
    //         initial_capital: 1000.0,
    //         continous: true,
    //         buy_with_equity: false,
    //         ..StrategyConfig::default()
    //     },
    // );

    // let mut metrics = TradingViewMetrics::new(
    //     ctx.clone(),
    //     &strategy,
    //     TradingViewMetricsConfig {
    //         risk_free_rate: 0.0,
    //         ..TradingViewMetricsConfig::default()
    //     },
    // );

    // let mut rsi_indicator = RelativeStrengthIndex::new(
    //     ctx.clone(),
    //     RelativeStrengthIndexConfig::default(ctx.clone()),
    // );
    // let mut rsi_strategy = RelativeStrengthIndexStrategy::new(
    //     ctx.clone(),
    //     RelativeStrengthIndexStrategyConfig {
    //         threshold_overbought: 50.0,
    //         threshold_oversold: 50.0,
    //     },
    // );

    // let best_strategy = fit_trades(
    //     Arc::clone(&ctx.data),
    //     FitTradesConfig {
    //         // start_index: 0,
    //         // end_index: 50,
    //         // start_index: 365,
    //         // end_index: 365 * 2,
    //         start_index: strategy.ctx.last_bar_index - 90,
    //         end_index: strategy.ctx.last_bar_index,
    //     },
    // );

    // for i in ctx.clone() {
    //     ctx.bar.index.set(i);

    //     let signal = best_strategy.to_signal(i);
    //     strategy.next(signal);
    //     metrics.next(&strategy);

    //     // let rsi = rsi_indicator.next(());
    //     // let rsi_signal = rsi_strategy.next(rsi);

    //     // strategy.next(rsi_signal);
    //     // metrics.next(&strategy);
    // }

    // println!("{:?}", best_strategy);

    // let currency = "USD";
    // metrics.data.print_overview(currency);
    // metrics.data.plot_net_equity((236, 100));
    // metrics.data.print_summary(currency);

    // let ps_exporter = PineScriptExporter::new();
    // let ps = ps_exporter.to_strategy(&strategy, PineScriptExportStrategyConfig::default());
    // println!("{}", ps);
}
