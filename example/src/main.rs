use std::path::Path;

use nersent_pace::{
    components::{
        component::Component, component_context::ComponentContext,
        component_default::ComponentDefault,
    },
    content::{
        relative_strength_index_indicator::{RsiIndicator, RsiIndicatorConfig},
        relative_strength_index_strategy::{RsiStrategy, RsiStrategyConfig},
    },
    data::in_memory_data_provider::InMemoryDataProvider,
    strategy::{
        metrics::{
            cobra_metrics_component::CobraMetricsComponentConfig,
            tradingview_metrics_component::TradingViewMetricsComponentConfig,
            wrapped_cobra_metrics_component::WrappedCobraMetricsComponent,
            wrapped_tradingview_metrics_component::WrappedTradingViewMetricsComponent,
        },
        strategy_context::{StrategyContext, StrategyContextConfig},
    },
    utils::polars::read_df,
};

fn main() {
    let data_path = Path::new("example/fixtures/btc_1d.csv");
    let df = read_df(&data_path);

    let ctx = ComponentContext::from_data_provider(InMemoryDataProvider::build(&df));

    let strategy_config = StrategyContextConfig {
        initial_capital: 1000.0,
        ..StrategyContextConfig::default()
    };

    let mut strategy = StrategyContext::new(ctx.clone(), strategy_config);
    let mut metrics = WrappedTradingViewMetricsComponent::new(
        strategy.clone(),
        TradingViewMetricsComponentConfig::default(),
    );

    let mut rsi_indicator =
        RsiIndicator::new(ctx.clone(), RsiIndicatorConfig::default(ctx.clone()));
    let mut rsi_strategy = RsiStrategy::new(ctx.clone(), RsiStrategyConfig::default());

    for _ in ctx.clone() {
        let rsi = rsi_indicator.next(());
        let rsi_signal = rsi_strategy.next(rsi);

        strategy.next(rsi_signal);
        metrics.next(());
    }

    let currency = "USD";
    metrics.data.print_overview(currency);
    metrics.data.plot_net_equity((236, 100));
    metrics.data.print_summary(currency);
}
