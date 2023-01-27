#[cfg(test)]
mod tests {
    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::true_range::true_range::TrueRange,
        testing::fixture::Fixture,
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
            let output = TrueRange::tr(
                ctx.high().unwrap(),
                ctx.low().unwrap(),
                prev_high,
                prev_low,
                prev_close,
                handle_na,
            );
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_true_range_without_handle_na() {
        let (_df, ctx, expected) =
            Fixture::load("ta/true_range/tests/fixtures/true_range/btc_1d_without_handle.csv");
        _test_true_range(&mut ctx.clone(), false, &expected);
    }

    #[test]
    fn test_true_range_with_handle_na() {
        let (_df, ctx, expected) =
            Fixture::load("ta/true_range/tests/fixtures/true_range/btc_1d_with_handle.csv");
        _test_true_range(&mut ctx.clone(), true, &expected);
    }
}
