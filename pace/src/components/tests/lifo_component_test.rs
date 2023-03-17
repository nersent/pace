#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use crate::{
        components::{
            component::Component, component_context::ComponentContext,
            lifo_component::LifoComponent,
        },
        data::in_memory_data_provider::InMemoryDataProvider,
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut LifoComponent, expected: &[(Option<f64>, Option<f64>, bool)]) {
        let mut snapshot = ArraySnapshot::<(Option<f64>, Option<f64>, bool)>::new();
        for _ in target.ctx.clone() {
            let (first_value, last_value, is_filled) = target.next(target.ctx.close());
            snapshot.push((first_value, last_value, is_filled));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_3() {
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
            &mut LifoComponent::new(ctx.clone(), 3),
            &[
                (None, Some(1.0), false),
                (None, Some(2.0), false),
                (Some(1.0), Some(3.0), true),
                (Some(2.0), Some(4.0), true),
                (Some(3.0), Some(5.0), true),
                (Some(4.0), Some(6.0), true),
                (Some(5.0), Some(7.0), true),
                (Some(6.0), Some(8.0), true),
            ],
        );
    }

    #[test]
    fn length_3_and_nones() {
        let ctx = ComponentContext::from_data_provider(Arc::from(
            InMemoryDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                None,
                Some(4.0),
                Some(5.0),
                None,
                Some(7.0),
                Some(8.0),
                Some(9.0),
            ])),
        ));

        _test(
            &mut LifoComponent::new(ctx.clone(), 3),
            &[
                (None, Some(1.0), false),
                (None, Some(2.0), false),
                (Some(1.0), None, true),
                (Some(2.0), Some(4.0), true),
                (None, Some(5.0), true),
                (Some(4.0), None, true),
                (Some(5.0), Some(7.0), true),
                (None, Some(8.0), true),
                (Some(7.0), Some(9.0), true),
            ],
        );
    }
}
