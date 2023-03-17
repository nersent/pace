#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src_component::SrcComponent, src_kind::SrcKind},
        content::williams_percent_range_indicator::{WprIndicator, WprIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/williams_percent_range/indicator/{}", path)
    }

    fn _test(target: &mut WprIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close.csv"));
        _test(
            &mut WprIndicator::new(
                ctx.clone(),
                WprIndicatorConfig {
                    length: 14,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn length_1_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1_close.csv"));
        _test(
            &mut WprIndicator::new(
                ctx.clone(),
                WprIndicatorConfig {
                    length: 1,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close.csv"));
        _test(
            &mut WprIndicator::new(
                ctx.clone(),
                WprIndicatorConfig {
                    length: 2,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }
}
