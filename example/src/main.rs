use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell, UnsafeCell},
    path::Path,
    rc::Rc,
    sync::Arc,
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

// }

// struct Aha {
//     xd: Xd,
//     // aha: dyn IncrementalNew<(), f64>,
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

fn main() {
    // println!("sum: {}", 2.0 + f64::NAN);
    // println!("mult: {}", 2.0 * f64::NAN);
    // println!("pow: {}", f64::powf(2.0, f64::NAN));
    // println!("pow: {}", f64::powf(f64::NAN, 2.0));
    println!("max: {}", f64::max(f64::NAN, 2.0));
    println!("diff: {}", 1.0 - f64::NAN);

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
