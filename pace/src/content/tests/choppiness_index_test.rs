#[cfg(test)]
mod tests {
    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            choppiness_index::{ChoppinessIndex, ChoppinessIndexConfig},
        },
        core::incremental::Incremental,
        polars::dataframe::DataFrameUtils,
        ta::{
            moving_average::{Ma, MaKind},
            simple_moving_average::Sma,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/choppiness_index/indicator/{}", path)
    }

    fn _test(target: &mut ChoppinessIndex, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14.csv"));
        _test(
            &mut ChoppinessIndex::new(ctx.clone(), ChoppinessIndexConfig { length: 14 }),
            &df.test_target(),
        );
    }

    #[test]
    fn length_2() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2.csv"));
        _test(
            &mut ChoppinessIndex::new(ctx.clone(), ChoppinessIndexConfig { length: 2 }),
            &df.test_target(),
        );
    }
}
