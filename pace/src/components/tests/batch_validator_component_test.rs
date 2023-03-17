#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use crate::{
        components::{
            batch_validator_component::BatchValidatorComponent, component::Component,
            component_context::ComponentContext,
        },
        data::in_memory_data_provider::InMemoryDataProvider,
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut BatchValidatorComponent, expected: &[Option<bool>]) {
        let mut snapshot = ArraySnapshot::<Option<bool>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.close());
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1() {
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
            &mut BatchValidatorComponent::new(ctx.clone(), 1),
            &[
                Some(true),
                Some(true),
                Some(true),
                Some(true),
                Some(true),
                Some(true),
                Some(true),
                Some(true),
            ],
        );
    }

    #[test]
    fn length_5() {
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
            &mut BatchValidatorComponent::new(ctx.clone(), 5),
            &[
                Some(true),
                Some(true),
                Some(true),
                Some(true),
                Some(true),
                Some(true),
                Some(true),
                Some(true),
            ],
        );
    }

    #[test]
    fn length_1_and_none() {
        let ctx = ComponentContext::from_data_provider(Arc::from(
            InMemoryDataProvider::from_values(Vec::from([
                None,
                Some(2.0),
                None,
                Some(4.0),
                Some(5.0),
                Some(6.0),
                None,
                Some(8.0),
            ])),
        ));

        _test(
            &mut BatchValidatorComponent::new(ctx.clone(), 1),
            &[
                Some(false),
                Some(true),
                Some(false),
                Some(true),
                Some(true),
                Some(true),
                Some(false),
                Some(true),
            ],
        );
    }

    #[test]
    fn length_3_and_none() {
        let ctx = ComponentContext::from_data_provider(Arc::from(
            InMemoryDataProvider::from_values(Vec::from([
                None,
                None,
                None,
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ])),
        ));

        _test(
            &mut BatchValidatorComponent::new(ctx.clone(), 3),
            &[
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(true),
                Some(true),
                Some(true),
            ],
        );
    }

    #[test]
    fn _length_3_and_none_mixed() {
        let ctx = ComponentContext::from_data_provider(Arc::from(
            InMemoryDataProvider::from_values(Vec::from([
                None,
                None,
                None,
                Some(4.0),
                Some(5.0),
                Some(6.0),
                None,
                Some(8.0),
                Some(9.0),
                Some(10.0),
                None,
                Some(1.0),
                Some(5.0),
                Some(1.0),
                Some(5.0),
                Some(2.0),
            ])),
        ));

        _test(
            &mut BatchValidatorComponent::new(ctx.clone(), 3),
            &[
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
                Some(false),
                Some(true),
                Some(false),
                Some(false),
                Some(false),
                Some(true),
                Some(true),
                Some(true),
            ],
        );
    }
}
