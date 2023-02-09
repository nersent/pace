#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use crate::base::{
        asset::in_memory_asset_data_provider::InMemoryAssetDataProvider,
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        execution_context::ExecutionContext,
        strategy::{
            metrics::equity_metric_i::{EquityMetric, EquityMetricConfig},
            strategy_execution_context::{
                StrategyExecutionContext, StrategyExecutionContextConfig,
            },
            trade::{
                compute_fill_size, compute_pnl, compute_return, compute_trade_pnl, Trade,
                TradeDirection,
            },
        },
    };

    fn _test(
        cctx: &mut ComponentContext,
        strategy_ctx: &mut StrategyExecutionContext,
        target_metric: &mut EquityMetric,
        trades: &[Option<TradeDirection>],
        expected: &[Option<(f64, f64, f64)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(f64, f64, f64)>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let tick = ctx.current_tick;
            let trade_direction = trades[tick];
            let trade = strategy_ctx.next(trade_direction);
            println!("\n\n[{}]: {:?}\n", tick, trade);
            let output = target_metric.next(trade);
            println!("{:?}\n\n\n", output);
            snapshot.push(Some((output.capital, output.trade_pnl, output.returns)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn empty_on_bar_close_continous() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    continous: true,
                    on_bar_close: true,
                },
            ),
            &mut EquityMetric::new(
                ctx.clone(),
                EquityMetricConfig {
                    initial_capital: 1000.0,
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
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                // 0
                Some(1.0),
                // 1
                Some(1.0),
                // 2
                Some(1.0),
                // 3
                Some(2.0),
                // 4
                Some(4.0),
                // 5
                Some(1.0),
                // 6
                Some(0.5),
                // 7
                Some(10.0),
                // 8
                Some(12.0),
                // 9
                Some(10.0),
                // 10
                Some(6.0),
                // 11
                Some(14.0),
                // 12
                Some(15.0),
                // 13
                Some(18.0),
                // 14
                Some(4.0),
                // 15
                Some(18.0),
                // 16
                Some(20.0),
                // 17
                Some(18.0),
                // 18
                Some(6.0),
                // 19
                Some(1.0),
                // 20
                Some(8.0),
                // 21
                Some(9.0),
                // 22
                Some(10.0),
                // 23
                Some(17.0),
                // 24
                Some(11.0),
                // 25
                Some(11.0),
                // 26
                Some(15.0),
                // 27
                Some(22.0),
                // 28
                Some(6.0),
                // 29
                Some(5.0),
                // 30
                Some(7.0),
                // 30
                Some(1.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    continous: true,
                    on_bar_close: true,
                },
            ),
            &mut EquityMetric::new(
                ctx.clone(),
                EquityMetricConfig {
                    initial_capital: 1000.0,
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(TradeDirection::Long),
                // 3
                None,
                // 4; Duplicated
                Some(TradeDirection::Long),
                // 5
                None,
                // 6
                None,
                // 7
                None,
                // 8
                Some(TradeDirection::Short),
                // 9
                None,
                // 10; Duplicated
                Some(TradeDirection::Short),
                // 11; Duplicated
                Some(TradeDirection::Short),
                // 12
                None,
                // 13
                None,
                // 14
                None,
                // 15
                Some(TradeDirection::Long),
                // 16
                None,
                // 17
                None,
                // 18
                Some(TradeDirection::Short),
                // 19
                None,
                // 20
                Some(TradeDirection::Long),
                // 21
                Some(TradeDirection::Short),
                // 22
                Some(TradeDirection::Long),
                // 23
                Some(TradeDirection::Short),
                // 24
                Some(TradeDirection::Long),
                // 25
                None,
                // 26
                None,
                // 27; Duplicated
                Some(TradeDirection::Long),
                // 28; Duplicated
                Some(TradeDirection::Long),
                // 29
                Some(TradeDirection::Short),
                // 30; Duplicated
                Some(TradeDirection::Short),
                // 31; Duplicated
                Some(TradeDirection::Short),
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
                Some((12000.0, 11000.0, 0.2)),
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
                Some((6000.0, -6000.0, -0.7)),
                // 16; pnl = (20.0 - 18.0) * (6000.0 / 18.0) * 1
                Some((6666.666666, 666.666666, 0.11111111)),
                // 17; pnl = (18.0 - 18.0) * (6000.0 / 18.0) * 1
                Some((6000.0, 0.0, -0.099999)),
                // 18; pnl = (6.0 - 18.0) * (6000.0 / 18.0) * 1
                Some((2000.0, -4000.0, -0.6666666)),
                // 19; pnl = (1.0 - 6.0) * (2000.0 / 6.0) * -1
                Some((3666.6666666, 1666.66666, 0.8333333)),
                // 20; pnl = (8.0 - 6.0) * (2000.0 / 6.0) * -1
                Some((1333.33334, -666.66666, -0.6363636)),
                // 21; pnl = (9.0 - 8.0) * (1333.33334 / 8.0) * 1
                Some((1500.0, 166.6666675, 0.12499)),
                // 22; pnl = (10.0 - 9.0) * (1500.0 / 9.0) * -1
                Some((1333.333333, -166.666666, -0.1111111)),
                // 23; pnl = (17.0 - 10.0) * (1333.333333 / 10.0) * 1
                Some((2266.66666, 933.333333, 0.7)),
                // 24; pnl = (11.0 - 17.0) * (2266.66666 / 17.0) * 1
                Some((3066.66666, 800.0, 0.352941176)),
                // 25; pnl = (11.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((3066.66666, 0.0, 0.0)),
                // 26; pnl = (15.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((4181.8181727, 1115.15151, 0.36363636)),
                // 27; pnl = (22.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((6133.3333333, 3066.666666, 0.466666)),
                // 28; pnl = (6.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((1672.72727, -1393.939390, -0.727272727)),
                // 29; pnl = (5.0 - 11.0) * (3066.66666 / 11.0) * 1
                Some((1393.939391, -1672.727269, -0.16666666)),
                // 30; pnl = (7.0 - 5.0) * (1393.939391 / 5.0) * -1
                Some((836.3636363636364, -557.5757564, -0.4)),
                // 31; pnl = (17.0 - 5.0) * (1393.939391 / 5.0) * -1
                Some((2509.0909038, 1115.1515128, 2.0)),
            ],
        );
    }

    #[test]
    fn equity_on_next_bar_open_continous() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                // 0
                Some(1.0),
                // 1
                Some(1.0),
                // 2
                Some(1.0),
                // 3
                Some(2.0),
                // 4
                Some(4.0),
                // 5
                Some(1.0),
                // 6
                Some(0.5),
                // 7
                Some(10.0),
                // 8
                Some(12.0),
                // 9
                Some(10.0),
                // 10
                Some(6.0),
                // 11
                Some(14.0),
                // 12
                Some(15.0),
                // 13
                Some(18.0),
                // 14
                Some(4.0),
                // 15
                Some(18.0),
                // 16
                Some(19.0),
                // 17
                Some(18.0),
                // 18
                Some(6.0),
                // 19
                Some(1.0),
                // 20
                Some(0.02),
                // 21
                Some(0.01),
                // 22
                Some(10.0),
                // 23
                Some(17.0),
                // 24
                Some(11.0),
                // 25
                Some(11.0),
                // 26
                Some(15.0),
                // 27
                Some(22.0),
                // 28
                Some(6.0),
                // 29
                Some(5.0),
                // 30
                Some(7.0),
                // 31
                Some(1.0),
                // 32
                Some(11.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    continous: true,
                    on_bar_close: false,
                },
            ),
            &mut EquityMetric::new(
                ctx.clone(),
                EquityMetricConfig {
                    initial_capital: 1000.0,
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(TradeDirection::Long),
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
                Some(TradeDirection::Short),
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
                Some(TradeDirection::Long),
                // 16
                None,
                // 17
                None,
                // 18
                Some(TradeDirection::Short),
                // 19
                None,
                // 20
                Some(TradeDirection::Long),
                // 21
                Some(TradeDirection::Short),
                // 22
                Some(TradeDirection::Long),
                // 23
                Some(TradeDirection::Short),
                // 24
                Some(TradeDirection::Long),
                // 25
                None,
                // 26
                None,
                // 27; Duplicated
                Some(TradeDirection::Long),
                // 28; Duplicated
                Some(TradeDirection::Long),
                // 29
                Some(TradeDirection::Short),
                // 30; Duplicated
                Some(TradeDirection::Short),
                // 31; Duplicated
                Some(TradeDirection::Short),
                // 32; Duplicated
                Some(TradeDirection::Short),
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
                Some((5000.0, 4000.0, -0.16666666)),
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
                Some((500.0, -4500.0, -0.5)),
                // 17; pnl = (18.0 - 19.0) * (500.0 / 19) * 1
                Some((473.6842106, -26.3157894, -0.0526315788)),
                // 18; pnl = (6.0 - 19.0) * (500.0 / 19) * 1
                Some((157.8947369, -342.1052631, -0.666666666)),
                // 19; pnl = (1.0 - 19.0) * (500.0 / 19) * 1
                Some((26.31579, -473.684210, -0.833333330)),
                // 20; pnl = (0.02 - 1) * (26.31579 / 1) * -1
                Some((52.1052631, 25.78947, 0.98)),
                // 21; pnl = (0.01 - 1) * (26.31579 / 1) * -1
                Some((52.3684210526316, 26.052631578947377, 0.005050505050504972)),
                // 22; pnl = (10.0 - 0.01) * (52.3684210526316 / 0.01) * -1
                Some((52368.421052552, 52316.0526315, 999.0)),
                // 23; pnl = (10.0 - 17) * (52368.421052552 / 10) * -1
                Some((15710.52631578948, -36657.894736842114, -0.7)),
                // 24; pnl = (11.0 - 17) * (15710.52631578948 / 17) * -1
                Some((10165.634674922605, -5544.891640866876, -0.3529411764705882)),
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
                Some((6469.040247678022, -3696.5944272445836, 0.40000000000000013)),
                // 31; pnl = (1.0 - 7.0) * (6469.040247678022 / 7) * -1
                Some((12013.93188854, 5544.891640866876, 0.85714285714209)),
                // 32; pnl = (11.0 - 7.0) * (6469.040247678022 / 7) * -1
                Some((2772.445820433438, -3696.594427244584, -0.7692307692306)),
            ],
        );
    }

    #[test]
    fn equity_on_bar_close_intermittent() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                // 0
                Some(1.0),
                // 1
                Some(1.0),
                // 2
                Some(1.0),
                // 3
                Some(2.0),
                // 4
                Some(4.0),
                // 5
                Some(1.0),
                // 6
                Some(0.5),
                // 7
                Some(10.0),
                // 8
                Some(12.0),
                // 9
                Some(10.0),
                // 10
                Some(6.0),
                // 11
                Some(14.0),
                // 12
                Some(15.0),
                // 13
                Some(18.0),
                // 14
                Some(4.0),
                // 15
                Some(18.0),
                // 16
                Some(19.0),
                // 17
                Some(18.0),
                // 18
                Some(6.0),
                // 19
                Some(1.0),
                // 20
                Some(0.02),
                // 21
                Some(0.01),
                // 22
                Some(10.0),
                // 23
                Some(17.0),
                // 24
                Some(11.0),
                // 25
                Some(11.0),
                // 26
                Some(15.0),
                // 27
                Some(34.0),
                // 28
                Some(6.0),
                // 29
                Some(5.0),
                // 30
                Some(7.0),
                // 31
                Some(1.0),
                // 32
                Some(11.0),
                // 33
                Some(500.0),
                // 34
                Some(-50.0),
                // 35
                Some(11.0),
                // 36
                Some(11.0),
                // 37
                Some(11.0),
                // 38
                Some(57.0),
                // 39
                Some(11.0),
                // 40
                Some(5.0),
                // 41
                Some(2.0),
                // 42
                Some(61.0),
                // 43
                Some(57.0),
                // 44
                Some(30.0),
                // 45
                Some(6.0),
                // 46
                Some(8.0),
                // 47
                Some(5.0),
                // 48
                Some(10.0),
                // 49
                Some(8.0),
                // 50
                Some(12.0),
                // 51
                Some(16.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    continous: false,
                    on_bar_close: true,
                },
            ),
            &mut EquityMetric::new(
                ctx.clone(),
                EquityMetricConfig {
                    initial_capital: 1000.0,
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(TradeDirection::Long),
                // 3
                None,
                // 4; Duplicated
                Some(TradeDirection::Long),
                // 5
                None,
                // 6
                None,
                // 7; Duplicated
                Some(TradeDirection::Long),
                // 8
                Some(TradeDirection::Short),
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
                Some(TradeDirection::Long),
                // 16; Duplicated
                Some(TradeDirection::Long),
                // 17
                None,
                // 18
                Some(TradeDirection::Short),
                // 19
                None,
                // 20
                Some(TradeDirection::Long),
                // 21
                Some(TradeDirection::Short),
                // 22
                Some(TradeDirection::Long),
                // 23
                Some(TradeDirection::Short),
                // 24
                None,
                // 25
                None,
                // 26
                None,
                // 27
                Some(TradeDirection::Short),
                // 28
                None,
                // 29; Duplicated
                Some(TradeDirection::Short),
                // 30
                None,
                // 31; Duplicated
                Some(TradeDirection::Short),
                // 32
                Some(TradeDirection::Long),
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
                Some(TradeDirection::Short),
                // 39; Duplicated
                Some(TradeDirection::Short),
                // 40
                None,
                // 41; Duplicated
                Some(TradeDirection::Short),
                // 42
                None,
                // 43
                None,
                // 44; Duplicated
                Some(TradeDirection::Short),
                // 45
                Some(TradeDirection::Long),
                // 46
                None,
                // 47
                None,
                // 48
                Some(TradeDirection::Short),
                // 49
                Some(TradeDirection::Long),
                // 50
                Some(TradeDirection::Short),
                // 51
                Some(TradeDirection::Long),
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
                Some((12000.0, 11000.0, 0.2)),
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
                Some((4000.0, -8000.0, -0.66666666)),
                // 19; no trade
                Some((4000.0, 0.0, 0.0)),
                // 20; no trade
                Some((4000.0, 0.0, 0.0)),
                // 21; pnl = (0.01 - 0.02) * (4000.0 / 0.02) * 1
                Some((2000.0, -2000.0, -0.5)),
                // 22; no trade
                Some((2000.0, -0.0, 0.0)),
                // 23; pnl = (17 - 10) * (2000.0 / 10) * 1
                Some((3400.0, 1400.0, 0.7)),
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
                Some((5700.0, 2300.0, -0.14925373)),
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
                Some((10800.0, 5100.0, 0.28571428571)),
                // 46; no trade
                Some((10800.0, 0.0, 0.0)),
                // 47; no trade
                Some((10800.0, 0.0, 0.0)),
                // 48; no trade
                Some((10800.0, 0.0, 0.0)),
                // 49; pnl = (8 - 10) * (10800.0 / 10) * -1
                Some((12960.0, 2160.0, 0.2)),
                // 50; no trade
                Some((12960.0, 0.0, 0.0)),
                // 51; pnl = (16 - 12) * (12960.0 / 12) * -1
                Some((8640.0, -4320.0, -0.33333333333333337)),
            ],
        );
    }

    #[test]
    fn equity_next_bar_open_intermittent() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                // 0
                Some(1.0),
                // 1
                Some(1.0),
                // 2
                Some(1.0),
                // 3
                Some(2.0),
                // 4
                Some(4.0),
                // 5
                Some(1.0),
                // 6
                Some(0.5),
                // 7
                Some(10.0),
                // 8
                Some(12.0),
                // 9
                Some(10.0),
                // 10
                Some(6.0),
                // 11
                Some(14.0),
                // 12
                Some(15.0),
                // 13
                Some(18.0),
                // 14
                Some(4.0),
                // 15
                Some(18.0),
                // 16
                Some(5.0),
                // 17
                Some(10.0),
                // 18
                Some(6.0),
                // 19
                Some(1.0),
                // 20
                Some(2.0),
                // 21
                Some(5.0),
                // 22
                Some(10.0),
                // 23
                Some(20.0),
                // 24
                Some(10.0),
                // 25
                Some(8.0),
                // 26
                Some(15.0),
                // 27
                Some(34.0),
                // 28
                Some(10.0),
                // 29
                Some(5.0),
                // 30
                Some(7.0),
                // 31
                Some(1.0),
                // 32
                Some(12.0),
                // 33
                Some(4.0),
                // 34
                Some(-50.0),
                // 35
                Some(11.0),
                // 36
                Some(11.0),
                // 37
                Some(11.0),
                // 38
                Some(57.0),
                // 39
                Some(8.0),
                // 40
                Some(6.0),
                // 41
                Some(2.0),
                // 42
                Some(12.0),
                // 43
                Some(8.0),
                // 44
                Some(6.0),
                // 45
                Some(4.0),
                // 46
                Some(10.0),
                // 47
                Some(5.0),
                // 48
                Some(10.0),
                // 49
                Some(6.0),
                // 50
                Some(8.0),
                // 51
                Some(2.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    continous: false,
                    on_bar_close: false,
                },
            ),
            &mut EquityMetric::new(
                ctx.clone(),
                EquityMetricConfig {
                    initial_capital: 1000.0,
                },
            ),
            &[
                // 0
                None,
                // 1
                None,
                // 2
                Some(TradeDirection::Long),
                // 3
                None,
                // 4; Duplicated
                Some(TradeDirection::Long),
                // 5
                None,
                // 6
                None,
                // 7; Duplicated
                Some(TradeDirection::Long),
                // 8
                Some(TradeDirection::Short),
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
                Some(TradeDirection::Long),
                // 16; Duplicated
                Some(TradeDirection::Long),
                // 17
                None,
                // 18
                Some(TradeDirection::Short),
                // 19
                None,
                // 20
                Some(TradeDirection::Long),
                // 21; long entry
                Some(TradeDirection::Short),
                // 22; long exit
                Some(TradeDirection::Long),
                // 23; long entry
                Some(TradeDirection::Short),
                // 24; long exit
                None,
                // 25
                None,
                // 26
                None,
                // 27
                Some(TradeDirection::Short),
                // 28
                None,
                // 29; Duplicated
                Some(TradeDirection::Short),
                // 30
                None,
                // 31; Duplicated
                Some(TradeDirection::Short),
                // 32
                Some(TradeDirection::Long),
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
                Some(TradeDirection::Short),
                // 39; Duplicated
                Some(TradeDirection::Short),
                // 40
                None,
                // 41; Duplicated
                Some(TradeDirection::Short),
                // 42
                None,
                // 43
                None,
                // 44; Duplicated
                Some(TradeDirection::Short),
                // 45
                Some(TradeDirection::Long),
                // 46
                None,
                // 47
                None,
                // 48
                Some(TradeDirection::Short),
                // 49; short entry
                Some(TradeDirection::Long),
                // 50; short exit
                Some(TradeDirection::Short),
                // 51; short entry; no trades
                Some(TradeDirection::Long),
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
                Some((5000.0, 4000.0, -0.16666666666666663)),
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
                Some((1000.0, -4000.0, -0.83333333333)),
                // 20; no trade
                Some((1000.0, 0.0, 0.0)),
                // 21; no trade
                Some((1000.0, 0.0, 0.0)),
                // 22; pnl = (10.0 - 5) * (1000.0 / 5) * 1
                Some((2000.0, 1000.0, 1.0)),
                // 23; no trade
                Some((2000.0, 0.0, 0.0)),
                // 24; pnl = (10.0 - 20) * (2000.0 / 20) * 1
                Some((1000.0, -1000.0, -0.5)),
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
                Some((1600.0, 600.0, 1.0)),
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
                Some((1200.0, -400.0, -0.5)),
                // 47; no trades
                Some((1200.0, 0.0, 0.0)),
                // 48; no trades
                Some((1200.0, 0.0, 0.0)),
                // 49; no trades
                Some((1200.0, 0.0, 0.0)),
                // 50; pnl = (8 - 6) * (1200.0 / 6) * -1
                Some((800.0, -400.0, -0.33333333)),
                // 51; no trades
                Some((800.0, 0.0, 0.0)),
            ],
        );
    }
}
