#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::bars::{compute_highest, compute_highest_bars, compute_lowest, compute_lowest_bars},
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/bars/{}", path)
    }

    fn _test_highest_bars(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (!ctx.at_length(length)) {
                snapshot.push(None);
                continue;
            }
            let output = compute_highest_bars(ctx.prev_highs(length), length);
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn highest_bars_length_14_high() {
        let (_df, ctx, expected) = Fixture::load(&format_path("highest_bars/length_14_high.csv"));
        _test_highest_bars(&mut ctx.clone(), 14, &expected);
    }

    fn _test_lowest_bars(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (!ctx.at_length(length)) {
                snapshot.push(None);
                continue;
            }
            let output = compute_lowest_bars(ctx.prev_lows(length), length);
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn lowest_bars_length_14_low() {
        let (_df, ctx, expected) = Fixture::load(&format_path("lowest_bars/length_14_low.csv"));
        _test_lowest_bars(&mut ctx.clone(), 14, &expected);
    }

    fn _test_highest(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (!ctx.at_length(length)) {
                snapshot.push(None);
                continue;
            }
            let output = compute_highest(ctx.prev_highs(length));
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn highest_length_14_high() {
        let (_df, ctx, expected) = Fixture::load(&format_path("highest/length_14_high.csv"));
        _test_highest(&mut ctx.clone(), 14, &expected);
    }

    fn _test_lowest(cctx: &mut ComponentContext, length: usize, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            if (!ctx.at_length(length)) {
                snapshot.push(None);
                continue;
            }
            let output = compute_lowest(ctx.prev_lows(length));
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn lowest_length_14_low() {
        let (_df, ctx, expected) = Fixture::load(&format_path("lowest/length_14_low.csv"));
        _test_lowest(&mut ctx.clone(), 14, &expected);
    }
}
