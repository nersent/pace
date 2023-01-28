#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        asset::in_memory_asset_data_provider::InMemoryAssetDataProvider,
        components::{
            component_context::ComponentContext, execution_context::ExecutionContext,
            mean::mean_component::MeanComponent,
            stdev::stdev_component::StandardDeviationComponent, testing::ComponentTestSnapshot,
            variance::variance_component::VarianceComponent,
        },
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut StandardDeviationComponent,
        expected: &[Option<f64>],
    ) {
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
            &mut StandardDeviationComponent::new(ctx.clone()),
            &[
                Some(0.0),
                Some(1.4142135623730951),
                Some(3.055050463303893),
                Some(6.191391873668904),
                Some(12.198360545581526),
                Some(23.723406163533937),
                Some(45.87560820928077),
                Some(88.5336901168944),
                Some(170.83260162444924),
                Some(329.89702096933894),
            ],
        );
    }
}
