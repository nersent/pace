use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell, UnsafeCell},
    path::Path,
    rc::Rc,
};

use nersent_pace::{
    content::relative_strength_index::{
        RelativeStrengthIndex, RelativeStrengthIndexConfig, RelativeStrengthIndexStrategy,
        RelativeStrengthIndexStrategyConfig,
    },
    core::{
        context::Context,
        data_provider::DataProvider,
        in_memory_data_provider::InMemoryDataProvider,
        incremental::{Incremental, IncrementalDefault},
    },
    polars::io::read_df,
    strategy::{
        metrics::{
            cobra_metrics::{CobraMetrics, CobraMetricsConfig},
            tradingview_metrics::{TradingViewMetrics, TradingViewMetricsConfig},
        },
        strategy::{Strategy, StrategyConfig},
    },
};

fn main() {
    let data_path = Path::new("example/fixtures/btc_1d.csv");
    let df = read_df(&data_path);

    let ctx = Context::new(InMemoryDataProvider::from_df(&df).to_arc());

    let mut strategy = Strategy::new(
        ctx.clone(),
        StrategyConfig {
            initial_capital: 1000.0,
            ..StrategyConfig::default()
        },
    );

    let mut metrics = TradingViewMetrics::new(
        ctx.clone(),
        &strategy,
        TradingViewMetricsConfig {
            risk_free_rate: 0.0,
            ..TradingViewMetricsConfig::default()
        },
    );

    let mut rsi_indicator = RelativeStrengthIndex::new(
        ctx.clone(),
        RelativeStrengthIndexConfig::default(ctx.clone()),
    );
    let mut rsi_strategy = RelativeStrengthIndexStrategy::new(
        ctx.clone(),
        RelativeStrengthIndexStrategyConfig {
            threshold_overbought: 50.0,
            threshold_oversold: 50.0,
        },
    );

    for i in ctx.clone() {
        ctx.bar.index.set(i);

        let rsi = rsi_indicator.next(());
        let rsi_signal = rsi_strategy.next(rsi);

        strategy.next(rsi_signal);
        metrics.next(&strategy);
    }

    let currency = "USD";
    metrics.data.print_overview(currency);
    metrics.data.plot_net_equity((236, 100));
    metrics.data.print_summary(currency);
}
