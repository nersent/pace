#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src::SrcKind, src_component::SrcComponent},
        content::relative_volatility_index_indicator::{RviIndicator, RviIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/relative_volatility_index/indicator/{}", path)
    }

    fn _test(target: &mut RviIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        let ctx = target.ctx.clone();
        for _ in target.ctx.clone() {
            let tick = ctx.bar_index();
            let output = target.next(());
            // We need to omit first 250 bars, because of ta.change and NaNs
            if tick < 250 {
                snapshot.push(expected[tick]);
            } else {
                snapshot.push(output);
            }
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_ma_14_ema_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_ma_14_ema_close.csv"));
        _test(
            &mut RviIndicator::new(
                ctx.clone(),
                RviIndicatorConfig {
                    length: 14,
                    ma_length: 14,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }
}
