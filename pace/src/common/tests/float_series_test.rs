#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{rc::Rc, sync::Arc};

    use crate::common::float_series::FloatSeries;
    use crate::core::data_provider::DataProvider;
    use crate::core::incremental::Incremental;
    use crate::testing::fixture::{DataFrameFixtureUtils, Fixture};
    use crate::testing::pace::format_pace_fixture_path;
    use crate::{
        common::fixnan::FixNan,
        core::{context::Context, in_memory_data_provider::InMemoryDataProvider},
        testing::array_snapshot::ArraySnapshot,
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/common/float_series/{}", path))
    }

    fn _test_get(target: &mut FloatSeries, index: usize, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            target.next(target.ctx.bar.close());
            snapshot.push(target.get(index));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn get_operator_0_close() {
        let (df, ctx) = Fixture::load(&format_path("get_operator/0_close.csv"));
        _test_get(&mut FloatSeries::new(ctx.clone()), 0, &df.test_target());
    }

    #[test]
    fn get_operator_1_close() {
        let (df, ctx) = Fixture::load(&format_path("get_operator/1_close.csv"));
        _test_get(&mut FloatSeries::new(ctx.clone()), 1, &df.test_target());
    }

    #[test]
    fn get_operator_14_close() {
        let (df, ctx) = Fixture::load(&format_path("get_operator/14_close.csv"));
        _test_get(&mut FloatSeries::new(ctx.clone()), 14, &df.test_target());
    }
}
