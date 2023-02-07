#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use crate::base::{
        asset::in_memory_asset_data_provider::InMemoryAssetDataProvider,
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        execution_context::ExecutionContext,
        strategy::{
            orderbook_i::{Order, OrderBook, OrderBookConfig},
            strategy_execution_context_i::{
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
        target: &mut StrategyExecutionContext,
        trades: &[Option<TradeDirection>],
        expected: &[Option<(Option<Trade>, Vec<Trade>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<Trade>, Vec<Trade>)>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let tick = ctx.current_tick;
            let trade_direction = trades[tick];
            let output = target.next(trade_direction);
            snapshot.push(Some((output.map(|x| *x), target.trades.clone())))
        }
        snapshot.assert(expected);
    }

    fn _test_current_trade(
        cctx: &mut ComponentContext,
        target: &mut StrategyExecutionContext,
        trades: &[Option<TradeDirection>],
        expected: &[Option<Trade>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<Trade>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let tick = ctx.current_tick;
            let trade_direction = trades[tick];
            let output = target.next(trade_direction);
            snapshot.push(output.map(|x| *x))
        }
        snapshot.assert(expected);
    }

    fn _test_trades_history(
        cctx: &mut ComponentContext,
        target: &mut StrategyExecutionContext,
        trades: &[Option<TradeDirection>],
        expected: &[Option<Vec<Trade>>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<Vec<Trade>>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let tick = ctx.current_tick;
            let trade_direction = trades[tick];
            let output = target.next(trade_direction);
            snapshot.push(Some(target.trades.clone()))
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
            &[None, None, None, None, None],
            &[
                Some((None, vec![])),
                Some((None, vec![])),
                Some((None, vec![])),
                Some((None, vec![])),
                Some((None, vec![])),
            ],
        );
    }

    #[test]
    fn current_trade_on_bar_close_continous() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ])),
        )));

        _test_current_trade(
            &mut ctx.clone(),
            &mut StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    continous: true,
                    on_bar_close: true,
                },
            ),
            &[
                None,
                None,
                Some(TradeDirection::Short),
                None,
                Some(TradeDirection::Long),
                None,
                None,
                Some(TradeDirection::Short),
            ],
            &[
                None,
                None,
                Some(Trade {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(5.0),
                    entry_tick: Some(4),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(5.0),
                    entry_tick: Some(4),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(5.0),
                    entry_tick: Some(4),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(8.0),
                    entry_tick: Some(7),
                    exit_price: None,
                    exit_tick: None,
                }),
            ],
        );
    }

    #[test]
    fn trades_history_on_bar_close_continous() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
                Some(9.0),
                Some(10.0),
            ])),
        )));

        _test_trades_history(
            &mut ctx.clone(),
            &mut StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    continous: true,
                    on_bar_close: true,
                },
            ),
            &[
                None,
                None,
                Some(TradeDirection::Short),
                None,
                Some(TradeDirection::Long),
                None,
                None,
                Some(TradeDirection::Short),
                Some(TradeDirection::Long),
                Some(TradeDirection::Short),
            ],
            &[
                Some(vec![]),
                Some(vec![]),
                Some(vec![]),
                Some(vec![]),
                Some(vec![Trade {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: Some(5.0),
                    exit_tick: Some(4),
                }]),
                Some(vec![Trade {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: Some(5.0),
                    exit_tick: Some(4),
                }]),
                Some(vec![Trade {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(3.0),
                    entry_tick: Some(2),
                    exit_price: Some(5.0),
                    exit_tick: Some(4),
                }]),
                Some(vec![
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    Trade {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                ]),
                Some(vec![
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    Trade {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                ]),
                Some(vec![
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(3.0),
                        entry_tick: Some(2),
                        exit_price: Some(5.0),
                        exit_tick: Some(4),
                    },
                    Trade {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(5.0),
                        entry_tick: Some(4),
                        exit_price: Some(8.0),
                        exit_tick: Some(7),
                    },
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(8.0),
                        entry_tick: Some(7),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    Trade {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(10.0),
                        exit_tick: Some(9),
                    },
                ]),
            ],
        );
    }

    #[test]
    fn current_trade_next_bar_open_continous() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
                Some(9.0),
                Some(10.0),
            ])),
        )));

        _test_current_trade(
            &mut ctx.clone(),
            &mut StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    continous: true,
                    on_bar_close: false,
                },
            ),
            &[
                None,
                None,
                Some(TradeDirection::Short),
                None,
                Some(TradeDirection::Long),
                None,
                None,
                None,
                Some(TradeDirection::Short),
                None,
            ],
            &[
                None,
                None,
                None,
                Some(Trade {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(6.0),
                    entry_tick: Some(5),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(6.0),
                    entry_tick: Some(5),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(6.0),
                    entry_tick: Some(5),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Long,
                    is_closed: false,
                    entry_price: Some(6.0),
                    entry_tick: Some(5),
                    exit_price: None,
                    exit_tick: None,
                }),
                Some(Trade {
                    direction: TradeDirection::Short,
                    is_closed: false,
                    entry_price: Some(10.0),
                    entry_tick: Some(9),
                    exit_price: None,
                    exit_tick: None,
                }),
            ],
        );
    }

    #[test]
    fn trades_history_next_bar_open_continous() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
                Some(9.0),
                Some(10.0),
                Some(11.0),
                Some(12.0),
            ])),
        )));

        _test_trades_history(
            &mut ctx.clone(),
            &mut StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    continous: true,
                    on_bar_close: false,
                },
            ),
            &[
                None,
                None,
                Some(TradeDirection::Short),
                None,
                Some(TradeDirection::Long),
                None,
                None,
                Some(TradeDirection::Short),
                None,
                Some(TradeDirection::Long),
                Some(TradeDirection::Short),
                None,
            ],
            &[
                Some(vec![]),
                Some(vec![]),
                Some(vec![]),
                Some(vec![]),
                Some(vec![]),
                Some(vec![Trade {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: Some(6.0),
                    exit_tick: Some(5),
                }]),
                Some(vec![Trade {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: Some(6.0),
                    exit_tick: Some(5),
                }]),
                Some(vec![Trade {
                    direction: TradeDirection::Short,
                    is_closed: true,
                    entry_price: Some(4.0),
                    entry_tick: Some(3),
                    exit_price: Some(6.0),
                    exit_tick: Some(5),
                }]),
                Some(vec![
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    Trade {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                ]),
                Some(vec![
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    Trade {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                ]),
                Some(vec![
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    Trade {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                ]),
                Some(vec![
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(4.0),
                        entry_tick: Some(3),
                        exit_price: Some(6.0),
                        exit_tick: Some(5),
                    },
                    Trade {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(6.0),
                        entry_tick: Some(5),
                        exit_price: Some(9.0),
                        exit_tick: Some(8),
                    },
                    Trade {
                        direction: TradeDirection::Short,
                        is_closed: true,
                        entry_price: Some(9.0),
                        entry_tick: Some(8),
                        exit_price: Some(11.0),
                        exit_tick: Some(10),
                    },
                    Trade {
                        direction: TradeDirection::Long,
                        is_closed: true,
                        entry_price: Some(11.0),
                        entry_tick: Some(10),
                        exit_price: Some(12.0),
                        exit_tick: Some(11),
                    },
                ]),
            ],
        );
    }

    // #[test]
    // fn on_bar_close_continous() {
    //     let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
    //         InMemoryAssetDataProvider::from_values(Vec::from([
    //             Some(1.0),
    //             Some(2.0),
    //             Some(3.0),
    //             Some(4.0),
    //             Some(5.0),
    //             Some(6.0),
    //             Some(7.0),
    //             Some(8.0),
    //             Some(9.0),
    //             Some(10.0),
    //             Some(11.0),
    //             Some(12.0),
    //             Some(13.0),
    //             Some(14.0),
    //             Some(15.0),
    //         ])),
    //     )));

    //     _test(
    //         &mut ctx.clone(),
    //         &mut StrategyExecutionContext::new(
    //             ctx.clone(),
    //             StrategyExecutionContextConfig {
    //                 continous: true,
    //                 on_bar_close: true,
    //             },
    //         ),
    //         &[
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             Some(TradeDirection::Short),
    //             None,
    //             Some(TradeDirection::Long),
    //         ],
    //         &[
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((None, vec![])),
    //             Some((
    //                 Some(Trade {
    //                     direction: TradeDirection::Short,
    //                     is_closed: false,
    //                     entry_price: Some(13.0),
    //                     entry_tick: Some(12),
    //                     exit_price: None,
    //                     exit_tick: None,
    //                 }),
    //                 vec![],
    //             )),
    //             Some((
    //                 Some(Trade {
    //                     direction: TradeDirection::Short,
    //                     is_closed: false,
    //                     entry_price: Some(13.0),
    //                     entry_tick: Some(12),
    //                     exit_price: None,
    //                     exit_tick: None,
    //                 }),
    //                 vec![],
    //             )),
    //             Some((
    //                 Some(Trade {
    //                     direction: TradeDirection::Long,
    //                     is_closed: false,
    //                     entry_price: Some(15.0),
    //                     entry_tick: Some(14),
    //                     exit_price: None,
    //                     exit_tick: None,
    //                 }),
    //                 vec![Trade {
    //                     direction: TradeDirection::Short,
    //                     is_closed: true,
    //                     entry_price: Some(13.0),
    //                     entry_tick: Some(12),
    //                     exit_price: Some(15.0),
    //                     exit_tick: Some(14),
    //                 }],
    //             )),
    //         ],
    //     );
    // }
}
