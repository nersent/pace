#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use crate::core::data_provider::DataProvider;
    use crate::core::incremental::Incremental;
    use crate::{
        common::fixnan::FixNan,
        core::{context::Context, in_memory_data_provider::InMemoryDataProvider},
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut FixNan, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn all_non_nan() {
        let ctx = Context::new(
            InMemoryDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ]))
            .to_arc(),
        );

        _test(
            &mut FixNan::new(ctx.clone()),
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
        let ctx = Context::new(
            InMemoryDataProvider::from_values(Vec::from([
                None, None, None, None, None, None, None, None,
            ]))
            .to_arc(),
        );

        _test(
            &mut FixNan::new(ctx.clone()),
            &[None, None, None, None, None, None, None, None],
        );
    }

    #[test]
    fn mixed() {
        let ctx = Context::new(
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
            ]))
            .to_arc(),
        );

        _test(
            &mut FixNan::new(ctx.clone()),
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
