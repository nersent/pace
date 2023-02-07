#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use crate::base::{
        asset::in_memory_asset_data_provider::InMemoryAssetDataProvider,
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        execution_context::ExecutionContext,
        strategy::{
            orderbook::{Order, OrderBook, OrderBookConfig},
            trade::{
                compute_fill_size, compute_pnl, compute_return, compute_trade_pnl, Trade,
                TradeDirection,
            },
        },
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut OrderBook,
        orders: &[Vec<Order>],
        expected: &[Option<(Vec<Order>, Vec<Order>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Vec<Order>, Vec<Order>)>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let tick = ctx.current_tick;
            let price = ctx.close().unwrap();
            let tick_orders = &orders[tick];
            for order in tick_orders {
                target.place(*order);
            }
            let filled_orders = target.next(price);
            snapshot.push(Some((filled_orders, target.orders.clone())));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn no_orders() {
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
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut OrderBook::new(ctx.clone(), OrderBookConfig { slippage: 0 }),
            &[
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            &[
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
            ],
        );
    }

    #[test]
    fn slippage_0() {
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
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut OrderBook::new(ctx.clone(), OrderBookConfig { slippage: 0 }),
            &[
                vec![Order::new(TradeDirection::Long)],
                vec![Order::new(TradeDirection::Short)],
                vec![],
                vec![],
                vec![],
                vec![
                    Order::new(TradeDirection::Short),
                    Order::new(TradeDirection::Long),
                ],
                vec![],
                vec![],
                vec![
                    Order::new(TradeDirection::Short),
                    Order::new(TradeDirection::Long),
                    Order::new(TradeDirection::Long),
                    Order::new(TradeDirection::Long),
                    Order::new(TradeDirection::Short),
                    Order::new(TradeDirection::Long),
                    Order::new(TradeDirection::Short),
                ],
            ],
            &[
                Some((
                    vec![Order {
                        direction: TradeDirection::Long,
                        place_tick: Some(0),
                        fill_price: Some(1.0),
                        fill_tick: Some(0),
                        is_filled: true,
                    }],
                    vec![],
                )),
                Some((
                    vec![Order {
                        direction: TradeDirection::Short,
                        place_tick: Some(1),
                        fill_price: Some(2.0),
                        fill_tick: Some(1),
                        is_filled: true,
                    }],
                    vec![],
                )),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((
                    vec![
                        Order {
                            direction: TradeDirection::Short,
                            place_tick: Some(5),
                            fill_price: Some(6.0),
                            fill_tick: Some(5),
                            is_filled: true,
                        },
                        Order {
                            direction: TradeDirection::Long,
                            place_tick: Some(5),
                            fill_price: Some(6.0),
                            fill_tick: Some(5),
                            is_filled: true,
                        },
                    ],
                    vec![],
                )),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((
                    vec![
                        Order {
                            direction: TradeDirection::Short,
                            place_tick: Some(8),
                            fill_price: Some(9.0),
                            fill_tick: Some(8),
                            is_filled: true,
                        },
                        Order {
                            direction: TradeDirection::Long,
                            place_tick: Some(8),
                            fill_price: Some(9.0),
                            fill_tick: Some(8),
                            is_filled: true,
                        },
                        Order {
                            direction: TradeDirection::Long,
                            place_tick: Some(8),
                            fill_price: Some(9.0),
                            fill_tick: Some(8),
                            is_filled: true,
                        },
                        Order {
                            direction: TradeDirection::Long,
                            place_tick: Some(8),
                            fill_price: Some(9.0),
                            fill_tick: Some(8),
                            is_filled: true,
                        },
                        Order {
                            direction: TradeDirection::Short,
                            place_tick: Some(8),
                            fill_price: Some(9.0),
                            fill_tick: Some(8),
                            is_filled: true,
                        },
                        Order {
                            direction: TradeDirection::Long,
                            place_tick: Some(8),
                            fill_price: Some(9.0),
                            fill_tick: Some(8),
                            is_filled: true,
                        },
                        Order {
                            direction: TradeDirection::Short,
                            place_tick: Some(8),
                            fill_price: Some(9.0),
                            fill_tick: Some(8),
                            is_filled: true,
                        },
                    ],
                    vec![],
                )),
            ],
        );
    }

    #[test]
    fn slippage_1() {
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
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut OrderBook::new(ctx.clone(), OrderBookConfig { slippage: 1 }),
            &[
                vec![Order::new(TradeDirection::Long)],
                vec![],
                vec![],
                vec![Order::new(TradeDirection::Short)],
                vec![],
                vec![],
                vec![
                    Order::new(TradeDirection::Long),
                    Order::new(TradeDirection::Short),
                ],
                vec![],
                vec![],
            ],
            &[
                Some((
                    vec![],
                    vec![Order {
                        direction: TradeDirection::Long,
                        place_tick: Some(0),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![Order {
                        direction: TradeDirection::Long,
                        place_tick: Some(0),
                        fill_price: Some(2.0),
                        fill_tick: Some(1),
                        is_filled: true,
                    }],
                    vec![],
                )),
                Some((vec![], vec![])),
                Some((
                    vec![],
                    vec![Order {
                        direction: TradeDirection::Short,
                        place_tick: Some(3),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![Order {
                        direction: TradeDirection::Short,
                        place_tick: Some(3),
                        fill_price: Some(5.0),
                        fill_tick: Some(4),
                        is_filled: true,
                    }],
                    vec![],
                )),
                Some((vec![], vec![])),
                Some((
                    vec![],
                    vec![
                        Order {
                            direction: TradeDirection::Long,
                            place_tick: Some(6),
                            fill_price: None,
                            fill_tick: None,
                            is_filled: false,
                        },
                        Order {
                            direction: TradeDirection::Short,
                            place_tick: Some(6),
                            fill_price: None,
                            fill_tick: None,
                            is_filled: false,
                        },
                    ],
                )),
                Some((
                    vec![
                        Order {
                            direction: TradeDirection::Long,
                            place_tick: Some(6),
                            fill_price: Some(8.0),
                            fill_tick: Some(7),
                            is_filled: true,
                        },
                        Order {
                            direction: TradeDirection::Short,
                            place_tick: Some(6),
                            fill_price: Some(8.0),
                            fill_tick: Some(7),
                            is_filled: true,
                        },
                    ],
                    vec![],
                )),
                Some((vec![], vec![])),
            ],
        );
    }

    #[test]
    fn slippage_5() {
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
                Some(13.0),
                Some(14.0),
                Some(15.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut OrderBook::new(ctx.clone(), OrderBookConfig { slippage: 5 }),
            &[
                vec![Order::new(TradeDirection::Long)],
                vec![],
                vec![],
                vec![],
                vec![Order::new(TradeDirection::Short)],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            &[
                Some((
                    vec![],
                    vec![Order {
                        direction: TradeDirection::Long,
                        place_tick: Some(0),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![],
                    vec![Order {
                        direction: TradeDirection::Long,
                        place_tick: Some(0),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![],
                    vec![Order {
                        direction: TradeDirection::Long,
                        place_tick: Some(0),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![],
                    vec![Order {
                        direction: TradeDirection::Long,
                        place_tick: Some(0),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![],
                    vec![
                        Order {
                            direction: TradeDirection::Long,
                            place_tick: Some(0),
                            fill_price: None,
                            fill_tick: None,
                            is_filled: false,
                        },
                        Order {
                            direction: TradeDirection::Short,
                            place_tick: Some(4),
                            fill_price: None,
                            fill_tick: None,
                            is_filled: false,
                        },
                    ],
                )),
                Some((
                    vec![Order {
                        direction: TradeDirection::Long,
                        place_tick: Some(0),
                        fill_price: Some(6.0),
                        fill_tick: Some(5),
                        is_filled: true,
                    }],
                    vec![Order {
                        direction: TradeDirection::Short,
                        place_tick: Some(4),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![],
                    vec![Order {
                        direction: TradeDirection::Short,
                        place_tick: Some(4),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![],
                    vec![Order {
                        direction: TradeDirection::Short,
                        place_tick: Some(4),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![],
                    vec![Order {
                        direction: TradeDirection::Short,
                        place_tick: Some(4),
                        fill_price: None,
                        fill_tick: None,
                        is_filled: false,
                    }],
                )),
                Some((
                    vec![Order {
                        direction: TradeDirection::Short,
                        place_tick: Some(4),
                        fill_price: Some(10.0),
                        fill_tick: Some(9),
                        is_filled: true,
                    }],
                    vec![],
                )),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
                Some((vec![], vec![])),
            ],
        );
    }
}
