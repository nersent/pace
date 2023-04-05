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

    fn _test(target: &mut FixNan, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn all_non_nan() {
        let ctx = Context::new(
            InMemoryDataProvider::from_values(Vec::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]))
                .to_arc(),
        );

        _test(
            &mut FixNan::new(ctx.clone()),
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
        );
    }

    #[test]
    fn all_nan() {
        let ctx = Context::new(
            InMemoryDataProvider::from_values(Vec::from([
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
            ]))
            .to_arc(),
        );

        _test(
            &mut FixNan::new(ctx.clone()),
            &[
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
            ],
        );
    }

    #[test]
    fn mixed() {
        let ctx = Context::new(
            InMemoryDataProvider::from_values(Vec::from([
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                1.0,
                2.0,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                3.0,
                f64::NAN,
                4.0,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                5.0,
                6.0,
                7.0,
                f64::NAN,
            ]))
            .to_arc(),
        );

        _test(
            &mut FixNan::new(ctx.clone()),
            &[
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                1.0,
                2.0,
                2.0,
                2.0,
                2.0,
                3.0,
                3.0,
                4.0,
                4.0,
                4.0,
                4.0,
                4.0,
                5.0,
                6.0,
                7.0,
                7.0,
            ],
        );
    }
}
