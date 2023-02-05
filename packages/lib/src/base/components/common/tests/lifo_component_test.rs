#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::base::{
        asset::in_memory_asset_data_provider::InMemoryAssetDataProvider,
        components::{
            common::lifo_component::LifoComponent, component_context::ComponentContext,
            testing::ComponentTestSnapshot,
        },
        execution_context::ExecutionContext,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut LifoComponent,
        expected: &[Option<(Option<f64>, Option<f64>, bool)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>, bool)>::new();
        for cctx in cctx {
            let (first_value, last_value, is_filled) = target.next(cctx.get().close());
            snapshot.push(Some((first_value, last_value, is_filled)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_3() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Rc::from(
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
            &mut LifoComponent::new(ctx.clone(), 3),
            &[
                Some((None, Some(1.0), false)),
                Some((None, Some(2.0), false)),
                Some((Some(1.0), Some(3.0), true)),
                Some((Some(2.0), Some(4.0), true)),
                Some((Some(3.0), Some(5.0), true)),
                Some((Some(4.0), Some(6.0), true)),
                Some((Some(5.0), Some(7.0), true)),
                Some((Some(6.0), Some(8.0), true)),
            ],
        );
    }

    #[test]
    fn length_3_and_nones() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Rc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
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
        )));

        _test(
            &mut ctx.clone(),
            &mut LifoComponent::new(ctx.clone(), 3),
            &[
                Some((None, Some(1.0), false)),
                Some((None, Some(2.0), false)),
                Some((Some(1.0), None, true)),
                Some((Some(2.0), Some(4.0), true)),
                Some((None, Some(5.0), true)),
                Some((Some(4.0), None, true)),
                Some((Some(5.0), Some(7.0), true)),
                Some((None, Some(8.0), true)),
                Some((Some(7.0), Some(9.0), true)),
            ],
        );
    }
}
