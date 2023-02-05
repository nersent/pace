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
        content::relative_volatility_index_indicator::{
            RelativeVolatilityIndexIndicator, RelativeVolatilityIndexIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/relative_volatility_index/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeVolatilityIndexIndicator,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let tick = ctx.current_tick;
            let output = target.next();
            // We need to omit first 250 bars, because of ta.change and NaNs
            if tick < 250 {
                snapshot.push(expected[tick]);
            } else {
                snapshot.push(output);
            }
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_ma_14_ema_close() {
        let (_df, ctx, expected) =
            Fixture::load_with_target(&format_path("length_14_ma_14_ema_close.csv"), "_target_");
        _test(
            &mut ctx.clone(),
            &mut RelativeVolatilityIndexIndicator::new(
                ctx.clone(),
                RelativeVolatilityIndexIndicatorConfig {
                    length: 14,
                    ma_length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
