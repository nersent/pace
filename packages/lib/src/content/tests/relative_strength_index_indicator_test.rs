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
        content::relative_strength_index_indicator::{
            RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/relative_strength_index/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeStrengthIndexIndicator,
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
    fn length_14_open() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_open.csv"));
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexIndicator::new(
                ctx.clone(),
                RelativeStrengthIndexIndicatorConfig {
                    length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Open),
                },
            ),
            &expected,
        );
    }
}
