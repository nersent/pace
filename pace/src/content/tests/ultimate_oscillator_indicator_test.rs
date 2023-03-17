#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::ultimate_oscillator_indicator::{UoIndicator, UoIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/ultimate_oscillator/indicator/{}", path)
    }

    fn _test(target: &mut UoIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_7_mid_length_14_long_length_28() {
        let (df, ctx) = Fixture::load_ctx(&format_path(
            "short_length_7_mid_length_14_long_length_28.csv",
        ));

        _test(
            &mut UoIndicator::new(
                ctx.clone(),
                UoIndicatorConfig {
                    short_length: 7,
                    mid_length: 14,
                    long_length: 28,
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn short_length_1_mid_length_1_long_length_1() {
        let (df, ctx) = Fixture::load_ctx(&format_path(
            "short_length_1_mid_length_1_long_length_1.csv",
        ));

        _test(
            &mut UoIndicator::new(
                ctx.clone(),
                UoIndicatorConfig {
                    short_length: 1,
                    mid_length: 1,
                    long_length: 1,
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn short_length_30_mid_length_15_long_length_7() {
        let (df, ctx) = Fixture::load_ctx(&format_path(
            "short_length_30_mid_length_15_long_length_7.csv",
        ));

        _test(
            &mut UoIndicator::new(
                ctx.clone(),
                UoIndicatorConfig {
                    short_length: 30,
                    mid_length: 15,
                    long_length: 7,
                },
            ),
            &df.test_target(),
        );
    }
}