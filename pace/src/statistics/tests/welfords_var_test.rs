#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use crate::{
        core::{
            context::Context, data_provider::DataProvider,
            in_memory_data_provider::InMemoryDataProvider, incremental::Incremental,
        },
        statistics::{mean::Mean, welfords_var::WelfordsVar},
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut WelfordsVar, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close().unwrap());
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn variance() {
        let ctx = Context::new(
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
            ]))
            .to_arc(),
        );

        _test(
            &mut WelfordsVar::new(ctx.clone()),
            &[
                Some(0.0),
                Some(2.0),
                Some(9.333333333333332),
                Some(38.333333333333336),
                Some(148.8),
                Some(562.8),
                Some(2104.571428571429),
                Some(7838.214285714285),
                Some(29183.777777777777),
                Some(108832.04444444444),
            ],
        );
    }
}
