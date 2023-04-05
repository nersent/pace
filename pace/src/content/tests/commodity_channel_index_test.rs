#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            commodity_channel_index::{CommodityChannelIndex, CommodityChannelIndexConfig},
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
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!(
            "tests/content/commodity_channel_index/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut CommodityChannelIndex, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_hlc3_sma() {
        let (df, ctx) = Fixture::load(&format_path("length_14_hlc3_sma.csv"));
        _test(
            &mut CommodityChannelIndex::new(
                ctx.clone(),
                CommodityChannelIndexConfig {
                    length: 14,
                    src: Src::new(ctx.clone(), SrcKind::HLC3).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
