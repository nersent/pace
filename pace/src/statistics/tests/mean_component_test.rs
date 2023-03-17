#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        components::{component::Component, component_context::ComponentContext},
        data::in_memory_data_provider::InMemoryDataProvider,
        statistics::mean_component::MeanComponent,
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut MeanComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.close().unwrap());
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_mean() {
        let ctx = ComponentContext::from_data_provider(Arc::from(
            InMemoryDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ])),
        ));

        _test(
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
