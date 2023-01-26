#[cfg(test)]
mod tests {
    use crate::{
        base::{
            component_context::ComponentContext,
            explicit::explicit_bars::{
                compute_highest, compute_highest_bars, compute_lowest, compute_lowest_bars,
            },
            implicit::recursive::{
                recursive_cross_over::RecursiveCrossOver, recursive_rsi::RecursiveRSI,
                recursive_sma::RecursiveSMA,
            },
            utils::testing::{load_test_artifact_with_target, ComponentTestSnapshot},
        },
        data::types::Timeframe,
    };

    fn _test_highest_bars(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (ctx.current_tick < length - 1) {
                snapshot.push(None);
                continue;
            }
            let output = compute_highest_bars(ctx.prev_highs(length), length);
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_highest_bars_length_14_high() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("explicit/highest_bars/btc_1d_length_14_high.csv");
        _test_highest_bars(&mut ctx.clone(), 14, &expected);
    }

    fn _test_lowest_bars(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (ctx.current_tick < length - 1) {
                snapshot.push(None);
                continue;
            }
            let output = compute_lowest_bars(ctx.prev_lows(length), length);
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_lowest_bars_length_14_low() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("explicit/lowest_bars/btc_1d_length_14_low.csv");
        _test_lowest_bars(&mut ctx.clone(), 14, &expected);
    }

    fn _test_highest(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (ctx.current_tick < length - 1) {
                snapshot.push(None);
                continue;
            }
            let output = compute_highest(ctx.prev_highs(length), length);
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_highest_length_14_high() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("explicit/highest/btc_1d_length_14_high.csv");
        _test_highest(&mut ctx.clone(), 14, &expected);
    }

    fn _test_lowest(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (ctx.current_tick < length - 1) {
                snapshot.push(None);
                continue;
            }
            let output = compute_lowest(ctx.prev_lows(length), length);
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_lowest_length_14_low() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("explicit/lowest/btc_1d_length_14_low.csv");
        _test_lowest(&mut ctx.clone(), 14, &expected);
    }
}
