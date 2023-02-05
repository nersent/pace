#[cfg(test)]
mod tests {
    use crate::{
        base::components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        content::balance_of_power_indicator::BalanceOfPowerIndicator,
    };

    fn format_path(path: &str) -> String {
        format!("content/tests/fixtures/balance_of_power/indicator/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut BalanceOfPowerIndicator,
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
    fn default() {
        let (_df, ctx, expected) = Fixture::load(&format_path("default.csv"));
        _test(
            &mut ctx.clone(),
            &mut BalanceOfPowerIndicator::new(ctx.clone()),
            &expected,
        );
    }
}
