#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component::Component,
            src::SrcKind,
            src_component::{AnySrcComponent, SrcComponent},
        },
        content::{
            awesome_oscillator_indicator::{AoIndicator, AoIndicatorConfig},
            macd_indicator::{MacdIndicator, MacdIndicatorConfig},
        },
        ta::{ma::MaKind, ma_component::MaComponent, tr_component::TrComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/macd/indicator/{}", path)
    }

    fn _test(target: &mut MacdIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let (macd, signal) = target.next(());
            snapshot.push(macd);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_12_long_length_26_ema_close() {
        let (df, ctx) =
            Fixture::load_ctx(&format_path("short_length_12_long_length_26_ema_close.csv"));
        _test(
            &mut MacdIndicator::new(
                ctx.clone(),
                MacdIndicatorConfig {
                    short_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 12),
                    long_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 26),
                    short_src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                    long_src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                    signal_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 9),
                },
            ),
            &df.test_target(),
        );
    }
}
