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
        content::bollinger_bands_pb_indicator::{
            BollingerBandsPercentBIndicator, BollingerBandsPercentBIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/bollinger_bands_pb/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut BollingerBandsPercentBIndicator,
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
            &mut BollingerBandsPercentBIndicator::new(
                ctx.clone(),
                BollingerBandsPercentBIndicatorConfig {
                    length: 20,
                    mult: 2.0,
                    source: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
