#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{rc::Rc, sync::Arc};

    use crate::common::float_series::FloatSeries;
    use crate::core::data_provider::DataProvider;
    use crate::core::incremental::Incremental;
    use crate::pinescript::float::Float;
    use crate::testing::fixture::{DataFrameFixtureUtils, Fixture};
    use crate::testing::pace::format_pace_fixture_path;
    use crate::{
        common::fixnan::FixNan,
        core::{context::Context, in_memory_data_provider::InMemoryDataProvider},
        testing::array_snapshot::ArraySnapshot,
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/pinescript/float/{}", path))
    }

    // plot(volume, title="volume")

    // float target = 1.0

    // target := 2.0
    // target := 4.0
    // target := 5.0

    // plot(target[1], title="_target_")
    fn _test_get_operator_a(target: &mut Float, index: usize, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            target.set(2.0);
            target.set(4.0);
            target.set(5.0);
            snapshot.push(target.get(index));
            target.next(());
        }
        snapshot.assert(expected);
    }

    #[test]
    fn get_operator_0_a() {
        let (df, ctx) = Fixture::load(&format_path("get_operator/0_a.csv"));
        _test_get_operator_a(
            &mut Float::new(ctx.clone()).with_initial_value(1.0),
            0,
            &df.test_target(),
        );
    }

    #[test]
    fn get_operator_1_a() {
        let (df, ctx) = Fixture::load(&format_path("get_operator/1_a.csv"));
        _test_get_operator_a(
            &mut Float::new(ctx.clone()).with_initial_value(1.0),
            1,
            &df.test_target(),
        );
    }

    #[test]
    fn get_operator_14_a() {
        let (df, ctx) = Fixture::load(&format_path("get_operator/14_a.csv"));
        _test_get_operator_a(
            &mut Float::new(ctx.clone()).with_initial_value(1.0),
            14,
            &df.test_target(),
        );
    }
}
