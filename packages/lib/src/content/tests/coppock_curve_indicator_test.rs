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
        content::coppock_curve_indicator::{CoppockCurveIndicator, CoppockCurveIndicatorConfig},
    };

    fn format_path(path: &str) -> String {
        format!("content/tests/fixtures/coppock_curve/indicator/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut CoppockCurveIndicator,
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
    fn long_roc_length_14_short_roc_length_11_ma_length_10_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path(
            "long_roc_length_14_short_roc_length_11_ma_length_10_close.csv",
        ));
        _test(
            &mut ctx.clone(),
            &mut CoppockCurveIndicator::new(
                ctx.clone(),
                CoppockCurveIndicatorConfig {
                    ma_length: 10,
                    long_roc_length: 14,
                    short_roc_length: 11,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
