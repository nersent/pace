#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src::SrcKind, src_component::SrcComponent},
        content::connors_relative_strength_index_indicator::{CrsiIndicator, CrsiIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "tests/content/connors_relative_strength_index/indicator/{}",
            path
        )
    }

    fn _test(target: &mut CrsiIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_3_up_down_len_2_roc_length_100_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path(
            "length_3_up_down_len_2_roc_length_100_close.csv",
        ));

        _test(
            &mut CrsiIndicator::new(
                ctx.clone(),
                CrsiIndicatorConfig {
                    length_rsi: 3,
                    length_up_down: 2,
                    length_roc: 100,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }
}
