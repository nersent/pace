#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use crate::base::{
        asset::in_memory_asset_data_provider::InMemoryAssetDataProvider,
        components::{
            common::mean_component::MeanComponent, component_context::ComponentContext,
            testing::ComponentTestSnapshot,
        },
        execution_context::ExecutionContext,
    };

    fn _test(cctx: &mut ComponentContext, target: &mut MeanComponent, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let output = target.next(ctx.close().unwrap());
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_mean() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut MeanComponent::new(ctx.clone()),
            &[
                Some(1.0),
                Some(1.5),
                Some(2.0),
                Some(2.5),
                Some(3.0),
                Some(3.5),
                Some(4.0),
                Some(4.5),
            ],
        );
    }
}
