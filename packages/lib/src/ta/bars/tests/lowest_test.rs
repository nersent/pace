#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::bars::bars::Bars,
        testing::fixture::Fixture,
    };

    fn _test_lowest(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (ctx.current_tick < length - 1) {
                snapshot.push(None);
                continue;
            }
            let output = Bars::lowest(ctx.prev_lows(length), length);
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_lowest_length_14_low() {
        let (_df, ctx, expected) =
            Fixture::load("ta/bars/tests/fixtures/lowest/btc_1d_length_14_low.csv");
        _test_lowest(&mut ctx.clone(), 14, &expected);
    }
}
