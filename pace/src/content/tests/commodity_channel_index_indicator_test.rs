#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src::SrcKind, src_component::SrcComponent},
        content::commodity_channel_index_indicator::{CciIndicator, CciIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/commodity_channel_index/indicator/{}", path)
    }

    fn _test(target: &mut CciIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_hlc3_sma() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_hlc3_sma.csv"));
        _test(
            &mut CciIndicator::new(
                ctx.clone(),
                CciIndicatorConfig {
                    length: 14,
                    src: SrcComponent::build(ctx.clone(), SrcKind::HLC3),
                },
            ),
            &df.test_target(),
        );
    }
}
