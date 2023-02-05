#[cfg(test)]
mod tests {
    use crate::{
        base::{
            components::{
                component_context::ComponentContext,
                testing::{ComponentTestSnapshot, Fixture},
            },
            ta::ma::MovingAverageKind,
        },
        content::volume_oscillator_indicator::{
            VolumeOscillatorIndicator, VolumeOscillatorIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/volume_oscillator/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut VolumeOscillatorIndicator,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_5_long_length_10_ema() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("short_length_5_long_length_10_ema.csv"));
        _test(
            &mut ctx.clone(),
            &mut VolumeOscillatorIndicator::new(
                ctx.clone(),
                VolumeOscillatorIndicatorConfig {
                    short_length: 5,
                    long_length: 10,
                    long_ma_kind: MovingAverageKind::EMA,
                    short_ma_kind: MovingAverageKind::EMA,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn short_length_1_long_length_1_ema() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("short_length_1_long_length_1_ema.csv"));
        _test(
            &mut ctx.clone(),
            &mut VolumeOscillatorIndicator::new(
                ctx.clone(),
                VolumeOscillatorIndicatorConfig {
                    short_length: 1,
                    long_length: 1,
                    long_ma_kind: MovingAverageKind::EMA,
                    short_ma_kind: MovingAverageKind::EMA,
                },
            ),
            &expected,
        );
    }
}
