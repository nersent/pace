#[cfg(test)]
mod tests {
    use crate::{
        core::incremental::Incremental,
        ta::{cross_over_threshold::CrossOverThreshold, relative_strength_index::Rsi},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/cross/{}", path)
    }

    fn _test(target: &mut CrossOverThreshold, target_rsi: &mut Rsi, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output_rsi = target_rsi.next(target.ctx.bar().close);
            let output = target.next(output_rsi);
            snapshot.push(Some(if output { 1.0 } else { 0.0 }));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn over_with_rsi_length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("over/rsi/length_14_close.csv"));
        _test(
            &mut CrossOverThreshold::new(ctx.clone(), 30.0),
            &mut Rsi::new(ctx.clone(), 14),
            &df.test_target(),
        );
    }
}
