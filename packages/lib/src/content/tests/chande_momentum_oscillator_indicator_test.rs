#[cfg(test)]
mod tests {
    use crate::{
        base::{
            asset::source::{Source, SourceKind},
            components::{
                component_context::ComponentContext,
                testing::{ComponentTestSnapshot, Fixture},
            },
        },
        content::chande_momentum_oscillator_indicator::{
            ChandeMomentumOscillatorIndicator, ChandeMomentumOscillatorIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/chande_momentum_oscillator/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ChandeMomentumOscillatorIndicator,
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
    fn length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut ChandeMomentumOscillatorIndicator::new(
                ctx.clone(),
                ChandeMomentumOscillatorIndicatorConfig {
                    length: 14,
                    src: Source::from_kind(ctx, SourceKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut ChandeMomentumOscillatorIndicator::new(
                ctx.clone(),
                ChandeMomentumOscillatorIndicatorConfig {
                    length: 2,
                    src: Source::from_kind(ctx, SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
