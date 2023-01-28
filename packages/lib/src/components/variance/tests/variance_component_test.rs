#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        asset::in_memory_asset_data_provider::InMemoryAssetDataProvider,
        components::{
            component_context::ComponentContext, execution_context::ExecutionContext,
            mean::mean_component::MeanComponent, testing::ComponentTestSnapshot,
            variance::variance_component::VarianceComponent,
        },
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut VarianceComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let output = target.next(ctx.close().unwrap());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_mean() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Rc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                Some(2.0),
                Some(4.0),
                Some(8.0),
                Some(16.0),
                Some(32.0),
                Some(64.0),
                Some(128.0),
                Some(256.0),
                Some(512.0),
                Some(1024.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut VarianceComponent::new(ctx.clone()),
            &[
                None,
                Some(2.0),
                Some(9.333333333333332),
                Some(38.333333333333336),
                Some(148.8),
                Some(562.8),
                Some(2104.571428571429),
                Some(7838.214285714285),
                Some(29183.777777777777),
                Some(108832.04444444444),
            ],
        );
    }
}
