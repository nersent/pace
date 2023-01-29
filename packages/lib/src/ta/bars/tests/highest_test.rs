#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::bars::utils::BarUtils,
        testing::fixture::Fixture,
    };

    fn _test_highest(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (!ctx.at_length(length)) {
                snapshot.push(None);
                continue;
            }
            let output = BarUtils::highest(ctx.prev_highs(length));
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_highest_length_14_high() {
        let (_df, ctx, expected) =
            Fixture::load("ta/bars/tests/fixtures/highest/btc_1d_length_14_high.csv");
        _test_highest(&mut ctx.clone(), 14, &expected);
    }
}
