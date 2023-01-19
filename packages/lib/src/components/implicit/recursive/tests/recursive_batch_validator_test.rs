#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        components::{
            component_context::ComponentContext, execution_context::ExecutionContext,
            implicit::recursive::recursive_batch_validator::RecursiveBatchValidator,
            utils::testing::ComponentTestSnapshot,
        },
        data::in_memory_asset_data_provider::InMemoryAssetDataProvider,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RecursiveBatchValidator,
        expected: &[Option<bool>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<bool>::new();
        for cctx in cctx {
            let output = target.next(cctx.get().close());
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_batch_validator_with_length_1() {
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
            &mut RecursiveBatchValidator::new(ctx.clone(), 1),
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
    fn test_batch_validator_with_length_5() {
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
            &mut RecursiveBatchValidator::new(ctx.clone(), 5),
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
    fn test_batch_validator_with_length_1_and_none() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Rc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                None,
                Some(2.0),
                None,
                Some(4.0),
                Some(5.0),
                Some(6.0),
                None,
                Some(8.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut RecursiveBatchValidator::new(ctx.clone(), 1),
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
    fn test_batch_validator_with_length_3_and_none() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Rc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                None,
                None,
                None,
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut RecursiveBatchValidator::new(ctx.clone(), 3),
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
    fn test_batch_validator_with_length_3_and_none_mixed() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Rc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
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
        )));

        _test(
            &mut ctx.clone(),
            &mut RecursiveBatchValidator::new(ctx.clone(), 3),
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
