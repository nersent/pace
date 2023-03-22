#[cfg(test)]
mod tests {
    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            donchian_channels::{DonchianChannels, DonchianChannelsConfig},
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
        format!("tests/content/donchian_channels/indicator/{}", path)
    }

    fn _test(
        target: &mut DonchianChannels,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<(Option<f64>, Option<f64>, Option<f64>)>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(Some((output.upper, output.basis, output.lower)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14.csv"));
        let expected = df.merge_three_columns("_target_upper_", "_target_basis_", "_target_lower_");
        _test(
            &mut DonchianChannels::new(ctx.clone(), DonchianChannelsConfig { length: 14 }),
            &expected,
        );
    }
}
