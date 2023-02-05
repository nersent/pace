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
        content::bollinger_bands_width_indicator::{
            BollingerBandsWidthIndicator, BollingerBandsWidthIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/bollinger_bands_width/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut BollingerBandsWidthIndicator,
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
    fn length_20_mult_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_20_sma_mult_2_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut BollingerBandsWidthIndicator::new(
                ctx.clone(),
                BollingerBandsWidthIndicatorConfig {
                    length: 20,
                    mult: 2.0,
                    source: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
