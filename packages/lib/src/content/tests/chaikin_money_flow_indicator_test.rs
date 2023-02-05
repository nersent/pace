#[cfg(test)]
mod tests {
    use crate::{
        base::components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        content::chaikin_money_flow_indicator::{
            ChaikinMoneyFlowIndicator, ChaikinMoneyFlowIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/chaikin_money_flow/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ChaikinMoneyFlowIndicator,
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
            &mut ChaikinMoneyFlowIndicator::new(
                ctx.clone(),
                ChaikinMoneyFlowIndicatorConfig { length: 14 },
            ),
            &expected,
        );
    }
}
