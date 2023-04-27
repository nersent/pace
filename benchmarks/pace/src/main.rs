#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use kdam::tqdm;
use nersent_pace::{
    common::src::{Src, SrcKind},
    content::{
        macd::{Macd, MacdConfig},
        relative_strength_index::RelativeStrengthIndex,
    },
    core::{
        context::Context,
        data_provider::{AnyDataProvider, DataProvider},
        in_memory_data_provider::InMemoryDataProvider,
        incremental::Incremental,
    },
    polars::io::read_df,
    statistics::common::{mean, stdev},
    ta::{
        average_true_range::Atr,
        exponential_moving_average::Ema,
        moving_average::{Ma, MaKind},
        relative_strength_index::Rsi,
        simple_moving_average::Sma,
        stoch::Stoch,
        symmetrically_weighted_moving_average::Swma,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{borrow::Borrow, fs, path::Path, sync::Arc, time::Instant};

#[derive(Clone, Debug)]
struct Benchmark {
    pub id: String,
    /// Milliseconds
    pub time_list: Vec<f64>,
    pub step_time_list: Vec<f64>,
    pub bars: usize,
}

impl Benchmark {
    pub fn new(id: String) -> Benchmark {
        Self {
            id,
            time_list: Vec::new(),
            step_time_list: Vec::new(),
            bars: 0,
        }
    }

    pub fn run(id: &str, count: usize, cb: &mut dyn FnMut() -> (Instant, Instant)) -> Benchmark {
        let mut instance = Benchmark::new(String::from(id));
        for _ in tqdm!(0..count) {
            let (start_time, end_time) = cb();
            let time_s = (end_time - start_time).as_secs_f64();
            let time_ms = time_s * 1000.0;
            instance.time_list.push(time_ms);
        }
        return instance;
    }

    pub fn mean(&self) -> f64 {
        return mean(&self.time_list);
    }

    pub fn stdev(&self) -> f64 {
        return stdev(&self.time_list);
    }

    pub fn print(&self) {
        println!(
            "\n[{}]: Mean={}ms | Stdev={}ms\n",
            self.id,
            self.mean(),
            self.stdev()
        );
    }
}

#[derive(Copy, Clone, Debug)]
enum DataSize {
    Small,
    Large,
}

fn create_data_provider(size: DataSize) -> AnyDataProvider {
    let filename = match size {
        DataSize::Small => "small.parquet",
        DataSize::Large => "large.parquet",
    };

    let path = Path::new("benchmarks/.data").join(filename);

    let df = read_df(&path);

    return Arc::from(InMemoryDataProvider::from_df(&df));
}

fn create_ctx(data_provider: Arc<dyn DataProvider + Send + Sync>) -> Context {
    return Context::new(Arc::clone(&data_provider));
}

struct PaceBenchmarkRunner {}

impl PaceBenchmarkRunner {
    pub fn run(count: usize, data_provider: AnyDataProvider) -> Vec<Benchmark> {
        let bars = data_provider.get_end_tick() - data_provider.get_start_tick() + 1;

        println!("\nRunning benchmarks for {} bars", bars);

        let mut benchmarks: Vec<Benchmark> = Vec::new();

        benchmarks.push(Benchmark::run("sma_14", count, &mut || {
            let ctx = create_ctx(Arc::clone(&data_provider));
            let mut target = Sma::new(ctx.clone(), 14).to_box();
            let _ctx = target.ctx.clone();
            let start_time = Instant::now();

            for i in ctx.first_bar_index..=ctx.last_bar_index {
                ctx.bar.index.set(i);
                target.next(ctx.bar.close());
            }

            return (start_time, Instant::now());
        }));
        benchmarks.last().unwrap().print();

        benchmarks.push(Benchmark::run("ema_14", count, &mut || {
            let ctx = create_ctx(Arc::clone(&data_provider));
            let mut target = Ema::new(ctx.clone(), 14).to_box();
            let _ctx = target.ctx.clone();
            let start_time = Instant::now();

            for i in ctx.first_bar_index..=ctx.last_bar_index {
                ctx.bar.index.set(i);
                target.next(target.ctx.bar.close());
            }

            return (start_time, Instant::now());
        }));
        benchmarks.last().unwrap().print();

        // benchmarks.push(Benchmark::run("swma", count, &mut || {
        //     let ctx = create_ctx(Arc::clone(&data_provider));
        //     let mut target = Swma::new(ctx.clone());
        //     let _ctx = target.ctx.clone();
        //     let start_time = Instant::now();

        //     for i in ctx.first_bar_index..=ctx.last_bar_index {
        //         ctx.bar.index.set(i);
        //         target.next(target.ctx.bar.close());
        //     }

        //     return (start_time, Instant::now());
        // }));
        // benchmarks.last().unwrap().print();

        benchmarks.push(Benchmark::run("rsi_14", count, &mut || {
            let ctx = create_ctx(Arc::clone(&data_provider));
            let mut target = Rsi::new(ctx.clone(), 14).to_box();
            let _ctx = target.ctx.clone();
            let start_time = Instant::now();

            for i in ctx.first_bar_index..=ctx.last_bar_index {
                ctx.bar.index.set(i);
                target.next(target.ctx.bar.close());
            }

            return (start_time, Instant::now());
        }));
        benchmarks.last().unwrap().print();

        benchmarks.push(Benchmark::run("stoch_14", count, &mut || {
            let ctx = create_ctx(Arc::clone(&data_provider));
            let mut target = Stoch::new(ctx.clone(), 14).to_box();
            let _ctx = target.ctx.clone();
            let start_time = Instant::now();

            for i in ctx.first_bar_index..=ctx.last_bar_index {
                ctx.bar.index.set(i);
                let close = target.ctx.bar.close();
                target.next((close, close, close));
            }

            return (start_time, Instant::now());
        }));
        benchmarks.last().unwrap().print();

        benchmarks.push(Benchmark::run("atr_14", count, &mut || {
            let ctx = create_ctx(Arc::clone(&data_provider));
            let mut target = Atr::new(ctx.clone(), 14).to_box();
            let _ctx = target.ctx.clone();
            let start_time = Instant::now();

            for i in ctx.first_bar_index..=ctx.last_bar_index {
                ctx.bar.index.set(i);
                target.next(());
            }

            return (start_time, Instant::now());
        }));
        benchmarks.last().unwrap().print();

        benchmarks.push(Benchmark::run("macd_12_26", count, &mut || {
            let ctx = create_ctx(Arc::clone(&data_provider));
            let mut target = Macd::new(
                ctx.clone(),
                MacdConfig {
                    short_ma: Ma::new(ctx.clone(), MaKind::EMA, 12).to_box(),
                    long_ma: Ma::new(ctx.clone(), MaKind::EMA, 26).to_box(),
                    short_src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                    long_src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                    signal_ma: Ma::new(ctx.clone(), MaKind::EMA, 9).to_box(),
                },
            )
            .to_box();

            let _ctx = target.ctx.clone();
            let start_time = Instant::now();

            for i in ctx.first_bar_index..=ctx.last_bar_index {
                ctx.bar.index.set(i);
                target.next(());
            }

            return (start_time, Instant::now());
        }));
        benchmarks.last().unwrap().print();

        benchmarks.push(Benchmark::run("macd_12_26_rsi_14", count, &mut || {
            let ctx = create_ctx(Arc::clone(&data_provider));
            let mut target_macd = Macd::new(
                ctx.clone(),
                MacdConfig {
                    short_ma: Ma::new(ctx.clone(), MaKind::EMA, 12).to_box(),
                    long_ma: Ma::new(ctx.clone(), MaKind::EMA, 26).to_box(),
                    short_src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                    long_src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                    signal_ma: Ma::new(ctx.clone(), MaKind::EMA, 9).to_box(),
                },
            )
            .to_box();
            let mut target_rsi = Rsi::new(ctx.clone(), 14).to_box();
            let _ctx = target_macd.ctx.clone();
            let start_time = Instant::now();

            for i in ctx.first_bar_index..=ctx.last_bar_index {
                ctx.bar.index.set(i);
                target_macd.next(());
                target_rsi.next(target_rsi.ctx.bar.close());
            }

            return (start_time, Instant::now());
        }));
        benchmarks.last().unwrap().print();

        // xddddddddd

        // benchmarks.push(Benchmark::run(
        //     "macd_12_26_rsi_14_aroon_14",
        //     count,
        //     &mut || {
        //         let ctx = create_ctx(Arc::clone(&data_provider));
        //         let mut target_macd = Macd::new(
        //             ctx.clone(),
        //             MacdConfig {
        //                 short_ma: Ma::new(ctx.clone(), MaKind::EMA, 12),
        //                 long_ma: Ma::new(ctx.clone(), MaKind::EMA, 26),
        //                 short_src: Src::new(ctx.clone(), SrcKind::Close),
        //                 long_src: Src::new(ctx.clone(), SrcKind::Close),
        //                 signal_ma: Ma::new(ctx.clone(), MaKind::EMA, 9),
        //             },
        //         );
        //         let mut target_rsi = RsiIndicator::new(
        //             ctx.clone(),
        //             RsiIndicatorConfig {
        //                 length: 14,
        //                 src: Src::new(ctx.clone(), SrcKind::Open),
        //             },
        //         );
        //         let mut target_aroon =
        //             AroonIndicator::new(ctx.clone(), AroonIndicatorConfig { length: 14 });
        //         let _ctx = target_macd.ctx.clone();
        //         let start_time = Instant::now();
        //         for _ in _ctx {
        //             target_macd.next(());
        //             target_rsi.next(());
        //             target_aroon.next(());
        //         }
        //         return (start_time, Instant::now());
        //     },
        // ));
        // benchmarks.last().unwrap().print();

        // benchmarks.push(Benchmark::run("dmi_14", count, &mut || {
        //     let ctx = create_ctx(Arc::clone(&data_provider));
        //     let mut target = DmiIndicator::new(
        //         ctx.clone(),
        //         DmiIndicatorConfig {
        //             length: 14,
        //             lensig: 14,
        //         },
        //     );
        //     let _ctx = target.ctx.clone();
        //     let start_time = Instant::now();
        //     for _ in _ctx {
        //         target.next(());
        //     }
        //     return (start_time, Instant::now());
        // }));
        // benchmarks.last().unwrap().print();

        // benchmarks.push(Benchmark::run("aroon_14", count, &mut || {
        //     let ctx = create_ctx(Arc::clone(&data_provider));
        //     let mut target = AroonIndicator::new(ctx.clone(), AroonIndicatorConfig { length: 14 });
        //     let _ctx = target.ctx.clone();
        //     let start_time = Instant::now();
        //     for _ in _ctx {
        //         target.next(());
        //     }
        //     return (start_time, Instant::now());
        // }));

        // benchmarks.last().unwrap().print();

        // benchmarks.push(Benchmark::run(
        //     "coppock_curve_length_10_long_14_short_11",
        //     count,
        //     &mut || {
        //         let ctx = create_ctx(Arc::clone(&data_provider));
        //         let mut target = CcIndicator::new(
        //             ctx.clone(),
        //             CcIndicatorConfig {
        //                 length: 10,
        //                 long_roc_length: 14,
        //                 short_roc_length: 11,
        //                 src: Src::new(ctx.clone(), SrcKind::Close),
        //             },
        //         );
        //         let _ctx = target.ctx.clone();
        //         let start_time = Instant::now();
        //         for _ in _ctx {
        //             target.next(());
        //         }
        //         return (start_time, Instant::now());
        //     },
        // ));

        // benchmarks.last().unwrap().print();

        return benchmarks
            .into_iter()
            .map(|mut r| {
                r.bars = bars;
                return r;
            })
            .collect();
    }
}

#[derive(Serialize, Deserialize)]
struct BenchmarkJsonData {
    id: String,
    benchmarks: Vec<BenchmarkJsonEntryData>,
}

#[derive(Serialize, Deserialize)]
struct BenchmarkJsonEntryData {
    id: String,
    runs: usize,
    bars: usize,
    mean: f64,
    stdev: f64,
}

fn save_benchmarks_to_json(id: &str, benchmarks: Vec<Benchmark>, filename: &str) {
    let data: BenchmarkJsonData = BenchmarkJsonData {
        id: id.to_string(),
        benchmarks: benchmarks
            .iter()
            .map(|benchmark| BenchmarkJsonEntryData {
                id: benchmark.id.to_string(),
                runs: benchmark.time_list.len(),
                mean: benchmark.mean(),
                stdev: benchmark.stdev(),
                bars: benchmark.bars,
            })
            .collect(),
    };

    let path = format!("benchmarks/.out/{}", filename);

    // save json to path using serde
    fs::write(path, serde_json::to_string_pretty(&data).unwrap());
}

fn main() {
    let mut iterations = String::new();
    println!("How many iterations?");
    std::io::stdin().read_line(&mut iterations).unwrap();
    let iterations: usize = iterations.trim().parse().unwrap();

    let mut dataset_size = String::new();
    println!("Which dataset size? 0=both, 1=small, 2=large");
    std::io::stdin().read_line(&mut dataset_size).unwrap();
    let dataset_size: usize = dataset_size.trim().parse().unwrap();

    let dataset_size: Vec<DataSize> = match dataset_size {
        0 => vec![DataSize::Small, DataSize::Large],
        1 => vec![DataSize::Small],
        2 => vec![DataSize::Large],
        _ => panic!("Invalid dataset size"),
    };

    let data_providers = dataset_size
        .iter()
        .map(|size| create_data_provider(*size))
        .collect::<Vec<_>>();

    let benchmarks = data_providers
        .iter()
        .map(|provider| PaceBenchmarkRunner::run(iterations, provider.clone()))
        .flatten()
        .collect::<Vec<_>>();

    save_benchmarks_to_json("pace", benchmarks, "pace.json")

    // let small_data_provider = create_data_provider(DataSize::Small);
    // let large_data_provider = create_data_provider(DataSize::Large);

    // let benchmarks_small = PaceBenchmarkRunner::run(iterations, small_data_provider);
    // let benchmarks_large = PaceBenchmarkRunner::run(iterations / 10, large_data_provider);

    // save_benchmarks_to_json(
    //     "pace",
    //     // benchmarks_large,
    //     benchmarks_small
    //         .iter()
    //         .chain(benchmarks_large.iter())
    //         .map(|r| r.clone())
    //         .collect::<Vec<_>>(),
    //     "pace.json",
    // )
}
