#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::volume_oscillator_indicator::{VoIndicator, VoIndicatorConfig},
        ta::{ma::MaKind, ma_component::MaComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/volume_oscillator/indicator/{}", path)
    }

    fn _test(target: &mut VoIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_5_long_length_10_ema() {
        let (df, ctx) = Fixture::load_ctx(&format_path("short_length_5_long_length_10_ema.csv"));

        _test(
            &mut VoIndicator::new(
                ctx.clone(),
                VoIndicatorConfig {
                    short_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 5),
                    long_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 10),
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn short_length_1_long_length_1_ema() {
        let (df, ctx) = Fixture::load_ctx(&format_path("short_length_1_long_length_1_ema.csv"));

        _test(
            &mut VoIndicator::new(
                ctx.clone(),
                VoIndicatorConfig {
                    short_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 1),
                    long_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 1),
                },
            ),
            &df.test_target(),
        );
    }
}
