// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// use serde::{Deserialize, Serialize};
// use serde_json::Result;
// use std::{borrow::Borrow, fs, path::Path, sync::Arc, time::Instant};

// // use kdam::tqdm;
// // use nersent_pace::{
// //     components::{
// //         component::Component, component_context::ComponentContext, src::SrcKind,
// //         src_component::Src,
// //     },
// //     content::{
// //         aroon_indicator::{AroonIndicator, AroonIndicatorConfig},
// //         coppock_curve_indicator::{CcIndicator, CcIndicatorConfig},
// //         directional_movement_index_indicator::{DmiIndicator, DmiIndicatorConfig},
// //         macd_indicator::{MacdIndicator, MacdIndicatorConfig},
// //         relative_strength_index_indicator::{RsiIndicator, RsiIndicatorConfig},
// //     },
// //     data::{data_provider::DataProvider, in_memory_data_provider::InMemoryDataProvider},
// //     statistics::common::{mean, stdev},
// //     ta::{
// //         atr_component::AtrComponent, ema_component::EmaComponent, ma::MaKind,
// //         ma_component::MaComponent, rsi_component::RsiComponent, sma_component::Sma,
// //         stoch_component::StochComponent,
// //     },
// //     utils::polars::read_df,
// // };

// #[derive(Clone, Debug)]
// struct Benchmark {
//     pub id: String,
//     /// Milliseconds
//     pub time_list: Vec<f64>,
//     pub step_time_list: Vec<f64>,
//     pub bars: usize,
// }

// impl Benchmark {
//     pub fn new(id: String) -> Benchmark {
//         Self {
//             id,
//             time_list: Vec::new(),
//             step_time_list: Vec::new(),
//             bars: 0,
//         }
//     }

//     pub fn run(id: &str, count: usize, cb: &mut dyn FnMut() -> (Instant, Instant)) -> Benchmark {
//         let mut instance = Benchmark::new(String::from(id));
//         for _ in tqdm!(0..count) {
//             let (start_time, end_time) = cb();
//             let time_s = (end_time - start_time).as_secs_f64();
//             let time_ms = time_s * 1000.0;
//             instance.time_list.push(time_ms);
//         }
//         return instance;
//     }

//     pub fn mean(&self) -> f64 {
//         return mean(&self.time_list);
//     }

//     pub fn stdev(&self) -> f64 {
//         return stdev(&self.time_list);
//     }

//     pub fn print(&self) {
//         println!(
//             "\n[{}]: Mean={}ms | Stdev={}ms\n",
//             self.id,
//             self.mean(),
//             self.stdev()
//         );
//     }
// }

// enum DataSize {
//     Small,
//     Large,
// }

// fn create_data_provider(size: DataSize) -> Arc<dyn DataProvider + Send + Sync> {
//     let filename = match size {
//         DataSize::Small => "small.parquet",
//         DataSize::Large => "large.parquet",
//     };

//     let path = Path::new("benchmarks/.data").join(filename);

//     let df = read_df(&path);

//     return Arc::from(InMemoryDataProvider::from_df(&df));
// }

// fn create_ctx(data_provider: Arc<dyn DataProvider + Send + Sync>) -> ComponentContext {
//     return ComponentContext::from_data_provider(Arc::clone(&data_provider));
// }

// struct PaceBenchmarkRunner {}

// impl PaceBenchmarkRunner {
//     pub fn run(count: usize, data_provider: Arc<dyn DataProvider + Send + Sync>) -> Vec<Benchmark> {
//         let bars = data_provider.get_end_tick() - data_provider.get_start_tick() + 1;

//         println!("\nRunning benchmarks for {} bars", bars);

//         let mut benchmarks: Vec<Benchmark> = Vec::new();

//         benchmarks.push(Benchmark::run("sma_14", count, &mut || {
//             let ctx = create_ctx(Arc::clone(&data_provider));
//             let mut target = Sma::new(ctx.clone(), 14);
//             let start_time = Instant::now();
//             let _ctx = target.ctx.clone();
//             for _ in _ctx {
//                 target.next(target.ctx.close());
//             }
//             return (start_time, Instant::now());
//         }));
//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run("ema_14", count, &mut || {
//             let ctx = create_ctx(Arc::clone(&data_provider));
//             let mut target = EMa::new(ctx.clone(), 14);
//             let _ctx = target.ctx.clone();
//             let start_time = Instant::now();
//             for _ in _ctx {
//                 target.next(target.ctx.close());
//             }
//             return (start_time, Instant::now());
//         }));
//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run("rsi_14", count, &mut || {
//             let ctx = create_ctx(Arc::clone(&data_provider));
//             let mut target = RsiComponent::new(ctx.clone(), 14);
//             let _ctx = target.ctx.clone();
//             let start_time = Instant::now();
//             for _ in _ctx {
//                 target.next(target.ctx.close());
//             }
//             return (start_time, Instant::now());
//         }));
//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run("stoch_14", count, &mut || {
//             let ctx = create_ctx(Arc::clone(&data_provider));
//             let mut target = StochComponent::new(ctx.clone(), 14);
//             let _ctx = target.ctx.clone();
//             let start_time = Instant::now();
//             for _ in _ctx {
//                 let close = target.ctx.close();
//                 target.next((close, close, close));
//             }
//             return (start_time, Instant::now());
//         }));
//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run("atr_14", count, &mut || {
//             let ctx = create_ctx(Arc::clone(&data_provider));
//             let mut target = AtrComponent::new(ctx.clone(), 14);
//             let _ctx = target.ctx.clone();
//             let start_time = Instant::now();
//             for _ in _ctx {
//                 target.next(());
//             }
//             return (start_time, Instant::now());
//         }));
//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run("macd_12_26", count, &mut || {
//             let ctx = create_ctx(Arc::clone(&data_provider));
//             let mut target = MacdIndicator::new(
//                 ctx.clone(),
//                 MacdIndicatorConfig {
//                     short_ma: Ma::new(ctx.clone(), MaKind::EMA, 12),
//                     long_ma: Ma::new(ctx.clone(), MaKind::EMA, 26),
//                     short_src: Src::new(ctx.clone(), SrcKind::Close),
//                     long_src: Src::new(ctx.clone(), SrcKind::Close),
//                     signal_ma: Ma::new(ctx.clone(), MaKind::EMA, 9),
//                 },
//             );
//             let _ctx = target.ctx.clone();
//             let start_time = Instant::now();
//             for _ in _ctx {
//                 target.next(());
//             }
//             return (start_time, Instant::now());
//         }));
//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run("macd_12_26_rsi_14", count, &mut || {
//             let ctx = create_ctx(Arc::clone(&data_provider));
//             let mut target_macd = MacdIndicator::new(
//                 ctx.clone(),
//                 MacdIndicatorConfig {
//                     short_ma: Ma::new(ctx.clone(), MaKind::EMA, 12),
//                     long_ma: Ma::new(ctx.clone(), MaKind::EMA, 26),
//                     short_src: Src::new(ctx.clone(), SrcKind::Close),
//                     long_src: Src::new(ctx.clone(), SrcKind::Close),
//                     signal_ma: Ma::new(ctx.clone(), MaKind::EMA, 9),
//                 },
//             );
//             let mut target_rsi = RsiIndicator::new(
//                 ctx.clone(),
//                 RsiIndicatorConfig {
//                     length: 14,
//                     src: Src::new(ctx.clone(), SrcKind::Open),
//                 },
//             );
//             let _ctx = target_macd.ctx.clone();
//             let start_time = Instant::now();
//             for _ in _ctx {
//                 target_macd.next(());
//                 target_rsi.next(());
//             }
//             return (start_time, Instant::now());
//         }));
//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run(
//             "macd_12_26_rsi_14_aroon_14",
//             count,
//             &mut || {
//                 let ctx = create_ctx(Arc::clone(&data_provider));
//                 let mut target_macd = MacdIndicator::new(
//                     ctx.clone(),
//                     MacdIndicatorConfig {
//                         short_ma: Ma::new(ctx.clone(), MaKind::EMA, 12),
//                         long_ma: Ma::new(ctx.clone(), MaKind::EMA, 26),
//                         short_src: Src::new(ctx.clone(), SrcKind::Close),
//                         long_src: Src::new(ctx.clone(), SrcKind::Close),
//                         signal_ma: Ma::new(ctx.clone(), MaKind::EMA, 9),
//                     },
//                 );
//                 let mut target_rsi = RsiIndicator::new(
//                     ctx.clone(),
//                     RsiIndicatorConfig {
//                         length: 14,
//                         src: Src::new(ctx.clone(), SrcKind::Open),
//                     },
//                 );
//                 let mut target_aroon =
//                     AroonIndicator::new(ctx.clone(), AroonIndicatorConfig { length: 14 });
//                 let _ctx = target_macd.ctx.clone();
//                 let start_time = Instant::now();
//                 for _ in _ctx {
//                     target_macd.next(());
//                     target_rsi.next(());
//                     target_aroon.next(());
//                 }
//                 return (start_time, Instant::now());
//             },
//         ));
//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run("dmi_14", count, &mut || {
//             let ctx = create_ctx(Arc::clone(&data_provider));
//             let mut target = DmiIndicator::new(
//                 ctx.clone(),
//                 DmiIndicatorConfig {
//                     length: 14,
//                     lensig: 14,
//                 },
//             );
//             let _ctx = target.ctx.clone();
//             let start_time = Instant::now();
//             for _ in _ctx {
//                 target.next(());
//             }
//             return (start_time, Instant::now());
//         }));
//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run("aroon_14", count, &mut || {
//             let ctx = create_ctx(Arc::clone(&data_provider));
//             let mut target = AroonIndicator::new(ctx.clone(), AroonIndicatorConfig { length: 14 });
//             let _ctx = target.ctx.clone();
//             let start_time = Instant::now();
//             for _ in _ctx {
//                 target.next(());
//             }
//             return (start_time, Instant::now());
//         }));

//         benchmarks.last().unwrap().print();

//         benchmarks.push(Benchmark::run(
//             "coppock_curve_length_10_long_14_short_11",
//             count,
//             &mut || {
//                 let ctx = create_ctx(Arc::clone(&data_provider));
//                 let mut target = CcIndicator::new(
//                     ctx.clone(),
//                     CcIndicatorConfig {
//                         length: 10,
//                         long_roc_length: 14,
//                         short_roc_length: 11,
//                         src: Src::new(ctx.clone(), SrcKind::Close),
//                     },
//                 );
//                 let _ctx = target.ctx.clone();
//                 let start_time = Instant::now();
//                 for _ in _ctx {
//                     target.next(());
//                 }
//                 return (start_time, Instant::now());
//             },
//         ));

//         benchmarks.last().unwrap().print();

//         return benchmarks
//             .into_iter()
//             .map(|mut r| {
//                 r.bars = bars;
//                 return r;
//             })
//             .collect();
//     }
// }

// #[derive(Serialize, Deserialize)]
// struct BenchmarkJsonData {
//     id: String,
//     benchmarks: Vec<BenchmarkJsonEntryData>,
// }

// #[derive(Serialize, Deserialize)]
// struct BenchmarkJsonEntryData {
//     id: String,
//     runs: usize,
//     bars: usize,
//     mean: f64,
//     stdev: f64,
// }

// fn save_benchmarks_to_json(id: &str, benchmarks: Vec<Benchmark>, filename: &str) {
//     let data: BenchmarkJsonData = BenchmarkJsonData {
//         id: id.to_string(),
//         benchmarks: benchmarks
//             .iter()
//             .map(|benchmark| BenchmarkJsonEntryData {
//                 id: benchmark.id.to_string(),
//                 runs: benchmark.time_list.len(),
//                 mean: benchmark.mean(),
//                 stdev: benchmark.stdev(),
//                 bars: benchmark.bars,
//             })
//             .collect(),
//     };

//     let path = format!("benchmarks/.out/{}", filename);

//     // save json to path using serde
//     fs::write(path, serde_json::to_string_pretty(&data).unwrap());
// }

// fn main() {
//     let mut iterations = String::new();
//     println!("How many iterations?");
//     std::io::stdin().read_line(&mut iterations).unwrap();
//     let iterations: usize = iterations.trim().parse().unwrap();

//     let small_data_provider = create_data_provider(DataSize::Small);
//     let large_data_provider = create_data_provider(DataSize::Large);

//     let benchmarks_small = PaceBenchmarkRunner::run(iterations, small_data_provider);
//     let benchmarks_large = PaceBenchmarkRunner::run(iterations, large_data_provider);

//     save_benchmarks_to_json(
//         "pace",
//         // benchmarks_large,
//         benchmarks_small
//             .iter()
//             .chain(benchmarks_large.iter())
//             .map(|r| r.clone())
//             .collect::<Vec<_>>(),
//         "pace.json",
//     )
// }
