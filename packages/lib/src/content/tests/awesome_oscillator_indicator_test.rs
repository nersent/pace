#[cfg(test)]
mod tests {
    use crate::{
        base::{
            asset::source::{Source, SourceKind},
            components::{
                component_context::ComponentContext,
                testing::{ComponentTestSnapshot, Fixture},
            },
            ta::ma::MovingAverageKind,
        },
        content::awesome_oscillator_indicator::{
            AwesomeOscillatorIndicator, AwesomeOscillatorIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/awesome_oscillator/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut AwesomeOscillatorIndicator,
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
    fn short_length_5_long_length_34_hl2() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("short_length_5_long_length_34_hl2.csv"));
        _test(
            &mut ctx.clone(),
            &mut AwesomeOscillatorIndicator::new(
                ctx.clone(),
                AwesomeOscillatorIndicatorConfig {
                    long_length: 34,
                    short_length: 5,
                    long_ma_type: MovingAverageKind::SMA,
                    short_ma_type: MovingAverageKind::SMA,
                    long_source: Source::from_kind(ctx.clone(), SourceKind::HL2),
                    short_source: Source::from_kind(ctx.clone(), SourceKind::HL2),
                },
            ),
            &expected,
        );
    }
}
