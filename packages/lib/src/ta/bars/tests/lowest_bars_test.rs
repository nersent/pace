#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::bars::utils::BarUtils,
        testing::fixture::Fixture,
    };

    fn _test_lowest_bars(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (!ctx.at_length(length)) {
                snapshot.push(None);
                continue;
            }
            let output = BarUtils::lowest_bars(ctx.prev_lows(length), length);
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_lowest_bars_length_14_low() {
        let (_df, ctx, expected) =
            Fixture::load("ta/bars/tests/fixtures/lowest_bars/btc_1d_length_14_low.csv");
        _test_lowest_bars(&mut ctx.clone(), 14, &expected);
    }
}
