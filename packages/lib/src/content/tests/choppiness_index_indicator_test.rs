#[cfg(test)]
mod tests {
    use crate::{
        base::components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        content::choppiness_index_indicator::{
            ChoppinessIndexIndicator, ChoppinessIndexIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!("content/tests/fixtures/choppiness_index/indicator/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ChoppinessIndexIndicator,
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
    fn length_14() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14.csv"));
        _test(
            &mut ctx.clone(),
            &mut ChoppinessIndexIndicator::new(
                ctx.clone(),
                ChoppinessIndexIndicatorConfig { length: 14 },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2.csv"));
        _test(
            &mut ctx.clone(),
            &mut ChoppinessIndexIndicator::new(
                ctx.clone(),
                ChoppinessIndexIndicatorConfig { length: 2 },
            ),
            &expected,
        );
    }
}
