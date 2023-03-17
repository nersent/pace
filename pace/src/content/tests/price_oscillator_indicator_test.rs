#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src::SrcKind, src_component::SrcComponent},
        content::price_oscillator_indicator::{PoIndicator, PoIndicatorConfig},
        ta::{ma::MaKind, ma_component::MaComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/price_oscillator/indicator/{}", path)
    }

    fn _test(target: &mut PoIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn long_length_21_short_length_10_long_ma_sma_short_ma_sma_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path(
            "long_length_21_short_length_10_long_ma_sma_short_ma_sma_close.csv",
        ));
        _test(
            &mut PoIndicator::new(
                ctx.clone(),
                PoIndicatorConfig {
                    short_ma: MaComponent::build(ctx.clone(), MaKind::SMA, 10),
                    long_ma: MaComponent::build(ctx.clone(), MaKind::SMA, 21),
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }
}
