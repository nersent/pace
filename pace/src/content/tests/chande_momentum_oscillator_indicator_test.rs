#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src::SrcKind, src_component::SrcComponent},
        content::chande_momentum_oscillator_indicator::{CmoIndicator, CmoIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "tests/content/chande_momentum_oscillator/indicator/{}",
            path
        )
    }

    fn _test(target: &mut CmoIndicator, expected: &[Option<f64>]) {
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
            &mut CmoIndicator::new(
                ctx.clone(),
                CmoIndicatorConfig {
                    length: 14,
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
            &mut CmoIndicator::new(
                ctx.clone(),
                CmoIndicatorConfig {
                    length: 2,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }
}
