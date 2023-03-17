#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component::Component,
            src_component::{AnySrcComponent, SrcComponent},
            src_kind::SrcKind,
        },
        content::bollinger_bands_pb_indicator::{BbpbIndicator, BbpbIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/bollinger_bands_pb/indicator/{}", path)
    }

    fn _test(target: &mut BbpbIndicator, expected: &[Option<f64>]) {
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
            &mut BbpbIndicator::new(
                ctx.clone(),
                BbpbIndicatorConfig {
                    length: 20,
                    mult: 2.0,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_target(),
        );
    }
}
