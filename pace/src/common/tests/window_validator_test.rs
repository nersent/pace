#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use crate::core::incremental::Incremental;
    use crate::{
        common::window_validator::WindowValidator,
        core::{
            context::Context, data_provider::DataProvider,
            in_memory_data_provider::InMemoryDataProvider,
        },
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut WindowValidator, expected: &[Option<bool>]) {
        let mut snapshot = ArraySnapshot::<Option<bool>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close());
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1() {
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
            &mut WindowValidator::new(ctx.clone(), 1),
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
            &mut WindowValidator::new(ctx.clone(), 5),
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
        let ctx = Context::new(
            InMemoryDataProvider::from_values(Vec::from([
                None,
                Some(2.0),
                None,
                Some(4.0),
                Some(5.0),
                Some(6.0),
                None,
                Some(8.0),
            ]))
            .to_arc(),
        );

        _test(
            &mut WindowValidator::new(ctx.clone(), 1),
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
        let ctx = Context::new(
            InMemoryDataProvider::from_values(Vec::from([
                None,
                None,
                None,
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ]))
            .to_arc(),
        );

        _test(
            &mut WindowValidator::new(ctx.clone(), 3),
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
        let ctx = Context::new(
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
            ]))
            .to_arc(),
        );

        _test(
            &mut WindowValidator::new(ctx.clone(), 3),
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
