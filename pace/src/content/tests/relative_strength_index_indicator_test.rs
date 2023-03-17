#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src_component::SrcComponent, src_kind::SrcKind},
        content::relative_strength_index_indicator::{RsiIndicator, RsiIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/relative_strength_index/indicator/{}", path)
    }

    fn _test(target: &mut RsiIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_open() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_open.csv"));
        _test(
            &mut RsiIndicator::new(
                ctx.clone(),
                RsiIndicatorConfig {
                    length: 14,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Open),
                },
            ),
            &df.test_target(),
        );
    }
}
