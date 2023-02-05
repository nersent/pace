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
        content::price_oscillator_indicator::{
            PriceOscillatorIndicator, PriceOscillatorIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!("content/tests/fixtures/price_oscillator/indicator/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut PriceOscillatorIndicator,
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
    fn long_length_21_short_length_10_long_ma_sma_short_ma_sma_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path(
            "long_length_21_short_length_10_long_ma_sma_short_ma_sma_close.csv",
        ));
        _test(
            &mut ctx.clone(),
            &mut PriceOscillatorIndicator::new(
                ctx.clone(),
                PriceOscillatorIndicatorConfig {
                    long_length: 21,
                    short_length: 10,
                    long_ma_type: MovingAverageKind::SMA,
                    short_ma_type: MovingAverageKind::SMA,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
