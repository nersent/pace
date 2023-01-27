#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::bars::utils::BarUtils,
        testing::fixture::Fixture,
    };

    fn _test_highest_bars(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (!ctx.at_length(length)) {
                snapshot.push(None);
                continue;
            }
            let output = BarUtils::highest_bars(ctx.prev_highs(length), length);
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_highest_bars_length_14_high() {
        let (_df, ctx, expected) =
            Fixture::load("ta/bars/tests/fixtures/highest_bars/btc_1d_length_14_high.csv");
        _test_highest_bars(&mut ctx.clone(), 14, &expected);
    }
}
