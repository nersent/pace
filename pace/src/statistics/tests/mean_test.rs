#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        core::{
            context::Context, data_provider::DataProvider,
            in_memory_data_provider::InMemoryDataProvider, incremental::Incremental,
        },
        statistics::mean::Mean,
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut Mean, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close().unwrap());
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_mean() {
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
            &mut Mean::new(ctx.clone()),
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
