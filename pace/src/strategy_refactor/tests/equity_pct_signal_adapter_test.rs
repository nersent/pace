#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::Arc,
    };

    use polars::prelude::DataFrame;

    use crate::{
        strategy_refactor::equity_pct_signal_adapter::compute_order_size_for_equity_pct,
        utils::float::Float64Utils,
    };

    const PRECISION: f64 = 0.00001;

    const POINT_VALUE: f64 = 1.0;

    const EXCHANGE_RATE: f64 = 1.0;

    // mult * (contracts + size) * price = equity
    // mult * contracts * price + mult * size * price = equity
    // size = (equity - contracts.abs() * price) / (mult * price)
    // size = (160 - 24 * 20) / 20

    #[test]
    pub fn no_position_to_position() {
        {
            let size = compute_order_size_for_equity_pct(
                1.0,
                1000.0,
                0.0,
                25.0,
                POINT_VALUE,
                EXCHANGE_RATE,
            );
            assert!(
                // (1) * 40 * 25 = 1000
                size.compare_with_precision(40.0, PRECISION),
                "[SAME_PRICE]: no position -> long 100% | GOT: {}",
                size
            );

            let size = compute_order_size_for_equity_pct(
                -1.0,
                1000.0,
                0.0,
                25.0,
                POINT_VALUE,
                EXCHANGE_RATE,
            );
            assert!(
                // (-1) * -40 * 25 = 1000
                size.compare_with_precision(-40.0, PRECISION),
                "[SAME_PRICE]: no position -> short 100% | GOT: {}",
                size
            );
        }
        {
            let size = compute_order_size_for_equity_pct(
                0.5,
                1000.0,
                0.0,
                25.0,
                POINT_VALUE,
                EXCHANGE_RATE,
            );
            assert!(
                // (1) * 20 * 25 = 500
                size.compare_with_precision(20.0, PRECISION),
                "[SAME_PRICE]: no position -> long 50% | GOT: {}",
                size
            );

            let size = compute_order_size_for_equity_pct(
                -0.5,
                1000.0,
                0.0,
                25.0,
                POINT_VALUE,
                EXCHANGE_RATE,
            );
            assert!(
                // (-1) * -20 * 25 = 500
                size.compare_with_precision(-20.0, PRECISION),
                "[SAME_PRICE]: no position -> short 50% | GOT: {}",
                size
            );
        }
    }

    #[test]
    pub fn decreased_position() {
        {
            // filled LONG 100% @ 25.0; EQ = 1000.0; QTY = 40.0 | CURRENT PRICE = 25.0; EQ = 1000
            let size = compute_order_size_for_equity_pct(
                0.5,
                1000.0,
                40.0,
                25.0,
                POINT_VALUE,
                EXCHANGE_RATE,
            );
            // (1) * (40.0 - 20.0) * 25 = 500
            assert!(
                size.compare_with_precision(-20.0, PRECISION),
                "[SAME_PRICE]: long 100% -> long 50% | GOT: {}",
                size
            );

            // filled SHORT 100% @ 25.0; EQ = 1000.0; QTY = 40.0 | CURRENT PRICE = 25.0; EQ = 1000
            let size = compute_order_size_for_equity_pct(
                -0.5,
                1000.0,
                -40.0,
                25.0,
                POINT_VALUE,
                EXCHANGE_RATE,
            );
            assert!(
                // (-1) * (-40.0 + 20.0) * 25 = 500
                size.compare_with_precision(20.0, PRECISION),
                "[SAME_PRICE]: short 100% -> short 50% | GOT: {}",
                size
            );
        }
        {
            // filled LONG 100% @ 25.0; EQ = 1000.0; QTY = 40.0 | CURRENT PRICE: 20.0;
            let size = compute_order_size_for_equity_pct(
                0.5,
                // 800
                1000.0 + (1.0) * (20.0 - 25.0) * 40.0,
                40.0,
                20.0,
                POINT_VALUE,
                EXCHANGE_RATE,
            );
            assert!(
                // (1) * (40.0 - 20.0) * 20 = 400
                size.compare_with_precision(-20.0, PRECISION),
                "[DIFFERENT_PRICE]: long 100% -> long 50% | GOT: {}",
                size
            );

            // filled SHORT 100% @ 25.0; EQ = 1000.0; QTY = 40.0 | CURRENT PRICE: 20.0
            let size = compute_order_size_for_equity_pct(
                -0.5,
                // (-1) * (-40.0 + 26.66666) * 30 = 400
                1000.0 + (-1.0) * (30.0 - 25.0) * 40.0,
                -40.0,
                30.0,
                POINT_VALUE,
                EXCHANGE_RATE,
            );
            assert!(
                size.compare_with_precision(26.6666666, PRECISION),
                "[DIFFERENT_PRICE]: short 100% -> short 50% | GOT: {}",
                size
            );
        }
        {
            // filled LONG 60% @ 25.0; EQ = 1000.0; QTY = (600 / 25) = 24 | CURRENT PRICE: 20.0;
            // 400 + (600 - 5 * 24) = 400 + 480 = 880
            let eq = 0.4 * 1000.0 + 0.6 * 1000.0 + (1.0) * (20.0 - 25.0) * 24.0;
            let size =
                compute_order_size_for_equity_pct(0.2, eq, 24.0, 20.0, POINT_VALUE, EXCHANGE_RATE);
            assert!(
                // size = (880 * 0.2 - 24 * 20) / 20
                // position equity = (1) * (24.0 - 16.0) * 24 = 160
                size.compare_with_precision(-15.2, PRECISION),
                "[DIFFERENT_PRICE]: long 80% -> long 20% | GOT: {} | EQ: {}",
                size,
                eq
            );

            // filled SHORT 60% @ 25.0; EQ = 1000.0; QTY = (600 / 25) = 24 | CURRENT PRICE: 20.0;
            // 400 + (600 - 5 * 24) = 400 + 480 = 880
            let eq = 0.4 * 1000.0 + 0.6 * 1000.0 + (-1.0) * (30.0 - 25.0) * 24.0;
            let size = compute_order_size_for_equity_pct(
                -0.2,
                eq,
                -24.0,
                30.0,
                POINT_VALUE,
                EXCHANGE_RATE,
            );
            assert!(
                // size = (880 * 0.2 - 24 * 30) / (-1 * 30)
                // 720
                size.compare_with_precision(18.13333333333, PRECISION),
                "[DIFFERENT_PRICE]: short 80% -> short 20% | GOT: {} | EQ: {}",
                size,
                eq
            );
        }
    }

    #[test]
    pub fn increased_position() {
        {
            // filled LONG 20% @ 25.0; EQ = 1000.0; QTY = (200 / 25) = 8 | CURRENT PRICE: 20.0;
            // 800 + (200 - 5 * 24) = 800 + 80 = 880
            let eq = 0.8 * 1000.0 + 0.2 * 1000.0 + (1.0) * (20.0 - 25.0) * 24.0;
            let size =
                compute_order_size_for_equity_pct(0.8, eq, 8.0, 20.0, POINT_VALUE, EXCHANGE_RATE);
            assert!(
                // size = (880 * 0.8 - 8 * 20) / 20
                size.compare_with_precision(27.2, PRECISION),
                "[DIFFERENT_PRICE]: long 20% -> long 80% | GOT: {} | EQ: {}",
                size,
                eq
            );
        }
    }

    #[test]
    pub fn flipped_position() {
        {
            // filled LONG 60% @ 25.0; EQ = 1000.0; QTY = (600 / 25) = 24 | CURRENT PRICE: 20.0;
            // 400 + (600 - 5 * 24) = 400 + 480 = 880
            let eq = 0.4 * 1000.0 + 0.6 * 1000.0 + (1.0) * (20.0 - 25.0) * 24.0;
            let size =
                compute_order_size_for_equity_pct(-0.8, eq, 24.0, 20.0, POINT_VALUE, EXCHANGE_RATE);
            assert!(
                // size = (880 * 0.2 - 24 * 20) / 20
                // position equity = (1) * (24.0 - 16.0) * 24 = 160
                size.compare_with_precision(-59.2, PRECISION),
                "[DIFFERENT_PRICE]: long 60% -> short 80% | GOT: {} | EQ: {}",
                size,
                eq
            );
        }
    }

    // {
    //     let size = compute_order_size_for_equity_pct(
    //         0.0,
    //         1000.0,
    //         40.0,
    //         25.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(-40.0, PRECISION),
    //         "[SAME_QTY]: long 100% -> no position 0% | GOT: {}",
    //         size
    //     );

    //     let size = compute_order_size_for_equity_pct(
    //         0.0,
    //         1000.0,
    //         -40.0,
    //         25.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(40.0, PRECISION),
    //         "[SAME_QTY]: short 100% -> no position 0% | GOT: {}",
    //         size
    //     );

    //     let size = compute_order_size_for_equity_pct(
    //         0.0,
    //         1000.0,
    //         40.0,
    //         5.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(-40.0, PRECISION),
    //         "[DIFFERENT_QTY]: long 100% -> no position 0% | GOT: {}",
    //         size
    //     );

    //     let size = compute_order_size_for_equity_pct(
    //         0.0,
    //         1000.0,
    //         -40.0,
    //         5.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(40.0, PRECISION),
    //         "[DIFFERENT_QTY]: short 100% -> no position 0% | GOT: {}",
    //         size
    //     );
    // }

    // {
    //     let size = compute_order_size_for_equity_pct(
    //         -1.0,
    //         1000.0,
    //         40.0,
    //         25.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(-80.0, PRECISION),
    //         "[SAME_QTY]: long 100% -> short 100% | GOT: {}",
    //         size
    //     );

    //     let size = compute_order_size_for_equity_pct(
    //         1.0,
    //         1000.0,
    //         -40.0,
    //         25.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(80.0, PRECISION),
    //         "[SAME_QTY]: short 100% -> long 100% | GOT: {}",
    //         size
    //     );

    //     let size = compute_order_size_for_equity_pct(
    //         -1.0,
    //         1000.0,
    //         40.0,
    //         5.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(-80.0, PRECISION),
    //         "[DIFFERENT_QTY]: long 100% -> short 100% | GOT: {}",
    //         size
    //     );

    //     let size = compute_order_size_for_equity_pct(
    //         1.0,
    //         1000.0,
    //         -40.0,
    //         5.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(80.0, PRECISION),
    //         "[DIFFERENT_QTY]: short 100% -> long 100% | GOT: {}",
    //         size
    //     );
    // }

    // {
    //     let size = compute_order_size_for_equity_pct(
    //         -0.5,
    //         1000.0,
    //         40.0,
    //         25.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(-60.0, PRECISION),
    //         "[SAME_QTY]: long 100% -> short 50% | GOT: {}",
    //         size
    //     );

    //     let size = compute_order_size_for_equity_pct(
    //         0.5,
    //         1000.0,
    //         -40.0,
    //         25.0,
    //         POINT_VALUE,
    //         EXCHANGE_RATE,
    //     );
    //     assert!(
    //         size.compare_with_precision(60.0, PRECISION),
    //         "[SAME_QTY]: short 100% -> long 50% | GOT: {}",
    //         size
    //     );
    // }
}
