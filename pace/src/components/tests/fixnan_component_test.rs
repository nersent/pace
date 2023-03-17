#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use crate::{
        components::{
            component::Component, component_context::ComponentContext,
            fixnan_component::FixNanComponent,
        },
        data::in_memory_data_provider::InMemoryDataProvider,
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut FixNanComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn all_non_nan() {
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
            &mut FixNanComponent::new(ctx.clone()),
            &[
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ],
        );
    }

    #[test]
    fn all_nan() {
        let ctx =
            ComponentContext::from_data_provider(Arc::from(InMemoryDataProvider::from_values(
                Vec::from([None, None, None, None, None, None, None, None]),
            )));

        _test(
            &mut FixNanComponent::new(ctx.clone()),
            &[None, None, None, None, None, None, None, None],
        );
    }

    #[test]
    fn mixed() {
        let ctx = ComponentContext::from_data_provider(Arc::from(
            InMemoryDataProvider::from_values(Vec::from([
                None,
                None,
                None,
                None,
                Some(1.0),
                Some(2.0),
                None,
                None,
                None,
                Some(3.0),
                None,
                Some(4.0),
                None,
                None,
                None,
                None,
                Some(5.0),
                Some(6.0),
                Some(7.0),
                None,
            ])),
        ));

        _test(
            &mut FixNanComponent::new(ctx.clone()),
            &[
                None,
                None,
                None,
                None,
                Some(1.0),
                Some(2.0),
                Some(2.0),
                Some(2.0),
                Some(2.0),
                Some(3.0),
                Some(3.0),
                Some(4.0),
                Some(4.0),
                Some(4.0),
                Some(4.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(7.0),
            ],
        );
    }
}
