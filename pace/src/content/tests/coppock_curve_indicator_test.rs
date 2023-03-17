#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src_component::SrcComponent, src_kind::SrcKind},
        content::coppock_curve_indicator::{CcIndicator, CcIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/coppock_curve/indicator/{}", path)
    }

    fn _test(target: &mut CcIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn long_roc_length_14_short_roc_length_11_ma_length_10_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path(
            "long_roc_length_14_short_roc_length_11_ma_length_10_close.csv",
        ));

        _test(
            &mut CcIndicator::new(
                ctx.clone(),
                CcIndicatorConfig {
                    ma_length: 10,
                    long_roc_length: 14,
                    short_roc_length: 11,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }
}
