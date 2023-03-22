#[cfg(test)]
mod tests {
    use crate::{
        core::incremental::Incremental,
        ta::{
            average_true_range::Atr, change::Change, exponential_moving_average::Ema,
            true_range::Tr,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/tr/{}", path)
    }

    fn _test(target: &mut Tr, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn without_handle_na() {
        let (df, ctx) = Fixture::load_ctx(&format_path("without_handle.csv"));
        _test(&mut Tr::new(ctx.clone(), false), &df.test_target());
    }

    #[test]
    fn with_handle_na() {
        let (df, ctx) = Fixture::load_ctx(&format_path("with_handle.csv"));
        _test(&mut Tr::new(ctx.clone(), true), &df.test_target());
    }
}
