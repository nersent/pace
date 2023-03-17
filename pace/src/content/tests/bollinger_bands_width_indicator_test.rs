#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component::Component,
            src::SrcKind,
            src_component::{AnySrcComponent, SrcComponent},
        },
        content::bollinger_bands_width_indicator::{BbwIndicator, BbwIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/bollinger_bands_width/indicator/{}", path)
    }

    fn _test(target: &mut BbwIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_20_mult_2_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_20_sma_mult_2_close.csv"));
        _test(
            &mut BbwIndicator::new(
                ctx.clone(),
                BbwIndicatorConfig {
                    length: 20,
                    mult: 2.0,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }
}
