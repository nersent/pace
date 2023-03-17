#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use crate::{
        components::{component::Component, component_context::ComponentContext},
        data::in_memory_data_provider::InMemoryDataProvider,
        stats::welfords_stdev_component::WelfordsStdevComponent,
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut WelfordsStdevComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.close().unwrap());
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn stdev() {
        let ctx = ComponentContext::from_data_provider(Arc::from(
            InMemoryDataProvider::from_values(Vec::from([
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
        ));

        _test(
            &mut WelfordsStdevComponent::new(ctx.clone()),
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
