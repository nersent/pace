#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use crate::{
        core::{
            context::Context, data_provider::DataProvider,
            in_memory_data_provider::InMemoryDataProvider, incremental::Incremental,
        },
        statistics::{mean::Mean, welfords_stdev::WelfordsStdev},
        testing::array_snapshot::ArraySnapshot,
    };

    fn _test(target: &mut WelfordsStdev, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close().unwrap());
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn stdev() {
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
            &mut WelfordsStdev::new(ctx.clone()),
            &[
                Some(0.0),
                Some(1.4142135623730951),
                Some(3.055050463303893),
                Some(6.191391873668904),
                Some(12.198360545581526),
                Some(23.723406163533937),
                Some(45.87560820928077),
                Some(88.5336901168944),
                Some(170.83260162444924),
                Some(329.89702096933894),
            ],
        );
    }
}
