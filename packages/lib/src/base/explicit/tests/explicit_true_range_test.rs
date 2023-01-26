#[cfg(test)]
mod tests {
    use crate::{
        base::{
            component_context::ComponentContext,
            explicit::{
                explicit_bars::{
                    compute_highest, compute_highest_bars, compute_lowest, compute_lowest_bars,
                },
                explicit_true_range::compute_true_range,
            },
            implicit::recursive::{
                recursive_cross_over::RecursiveCrossOver, recursive_rsi::RecursiveRSI,
                recursive_sma::RecursiveSMA,
            },
            utils::testing::{load_test_artifact_with_target, ComponentTestSnapshot},
        },
        data::types::Timeframe,
    };

    fn _test_true_range(cctx: &mut ComponentContext, handle_na: bool, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let (prev_high, prev_low, prev_close) = if ctx.current_tick == 0 {
                (None, None, None)
            } else {
                (ctx.prev_high(1), ctx.prev_low(1), ctx.prev_close(1))
            };
            let output = compute_true_range(
                ctx.high().unwrap(),
                ctx.low().unwrap(),
                prev_high,
                prev_low,
                prev_close,
                handle_na,
            );
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_true_range_without_handle_na() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("explicit/true_range/btc_1d_without_handle.csv");
        _test_true_range(&mut ctx.clone(), false, &expected);
    }

    #[test]
    fn test_true_range_with_handle_na() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("explicit/true_range/btc_1d_with_handle.csv");
        _test_true_range(&mut ctx.clone(), true, &expected);
    }
}
