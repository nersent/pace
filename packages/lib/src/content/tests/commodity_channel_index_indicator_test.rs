#[cfg(test)]
mod tests {
    use crate::{
        base::{
            asset::source::{Source, SourceKind},
            components::{
                component_context::ComponentContext,
                testing::{ComponentTestSnapshot, Fixture},
            },
            ta::ma::MovingAverageKind,
        },
        content::commodity_channel_index_indicator::{
            CommodityChannelIndexIndicator, CommodityChannelIndexIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/commodity_channel_index/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut CommodityChannelIndexIndicator,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_hlc3_sma() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_hlc3_sma.csv"));
        _test(
            &mut ctx.clone(),
            &mut CommodityChannelIndexIndicator::new(
                ctx.clone(),
                CommodityChannelIndexIndicatorConfig {
                    length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::HLC3),
                    ma_kind: MovingAverageKind::SMA,
                },
            ),
            &expected,
        );
    }
}
