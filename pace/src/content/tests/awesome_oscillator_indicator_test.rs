#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component::Component,
            src::SrcKind,
            src_component::{AnySrcComponent, SrcComponent},
        },
        content::awesome_oscillator_indicator::{AoIndicator, AoIndicatorConfig},
        ta::{ma::MaKind, ma_component::MaComponent, tr_component::TrComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/awesome_oscillator/indicator/{}", path)
    }

    fn _test(target: &mut AoIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_5_long_length_34_hl2() {
        let (df, ctx) = Fixture::load_ctx(&format_path("short_length_5_long_length_34_hl2.csv"));
        _test(
            &mut AoIndicator::new(
                ctx.clone(),
                AoIndicatorConfig {
                    long_ma: MaComponent::build(ctx.clone(), MaKind::SMA, 34),
                    short_ma: MaComponent::build(ctx.clone(), MaKind::SMA, 5),
                    long_src: SrcComponent::build(ctx.clone(), SrcKind::HL2),
                    short_src: SrcComponent::build(ctx.clone(), SrcKind::HL2),
                },
            ),
            &df.test_target(),
        );
    }
}
